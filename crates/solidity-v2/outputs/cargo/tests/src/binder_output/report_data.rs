use std::fmt::Display;
use std::ops::Range;

use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::ast::{DataLocation, Definition, Identifier, LiteralKind, NodeId, Type};
use slang_solidity_v2::compilation::{CompilationUnit, FileId};
use slang_solidity_v2_common::collections::{Map, SortedMap};

// Types

type FileSourceMap = SortedMap<FileId, String>;

pub(crate) struct ReportData<'a> {
    pub(crate) compilation: &'a CompilationUnit,
    pub(crate) files: &'a FileSourceMap,
    pub(crate) all_definitions: Vec<CollectedDefinition<'a>>,
    pub(crate) all_references: Vec<CollectedReference<'a>>,
    pub(crate) unbound_identifiers: Vec<CollectedIdentifier<'a>>,
}

/// This `DefinitionId` is local to the `ReportData` and represents the position
/// in the `all_definitions` vector of a given definition (strictly its index+1).
pub(crate) type DefinitionId = usize;

#[derive(Clone)]
pub(crate) struct CollectedIdentifier<'a> {
    node: Identifier<'a>,
    line: usize,
    column: usize,
}

impl CollectedIdentifier<'_> {
    pub(crate) fn file_id(&self) -> &FileId {
        self.node.get_file_id()
    }
    pub(crate) fn range(&self) -> &Range<usize> {
        self.node.get_text_range()
    }
}

pub(crate) struct CollectedDefinition<'a> {
    // This is the index+1 of this definition in the `all_definitions` vector.
    pub(crate) definition_id: DefinitionId,
    pub(crate) identifier: CollectedIdentifier<'a>,
    definition: Definition<'a>,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum CollectedResolution {
    Unresolved,
    BuiltIn,
    // The `DefinitionId` payload is internal to the `ReportData`
    Definition(DefinitionId),
}

pub(crate) struct CollectedReference<'a> {
    pub(crate) identifier: CollectedIdentifier<'a>,
    pub(crate) resolution: CollectedResolution,
}

// Implementation

impl<'a> ReportData<'a> {
    pub(crate) fn prepare(compilation: &'a CompilationUnit, files: &'a FileSourceMap) -> Self {
        let all_definitions = DefinitionCollector::collect_from(compilation, files);
        // This is used to map the reference resolutions to the internal
        // `DefinitionId` of the report.
        let definitions_by_node_id: Map<NodeId, DefinitionId> = all_definitions
            .iter()
            .map(|definition| (definition.definition.node_id(), definition.definition_id))
            .collect();
        let (all_references, unbound_identifiers) =
            ReferenceCollector::collect_from(compilation, files, &definitions_by_node_id);

        Self {
            compilation,
            files,
            all_definitions,
            all_references,
            unbound_identifiers,
        }
    }

    pub(crate) fn all_resolved(&self) -> bool {
        self.unbound_identifiers.is_empty()
            && self
                .all_references
                .iter()
                .all(|reference| reference.resolution != CollectedResolution::Unresolved)
    }
}

// Identifier collector trait, to allow reuse in the two collectors

trait IdentifierCollector<'a> {
    fn file_contents(&self, file_id: &FileId) -> &str;

    fn collect_identifier(&self, node: &Identifier<'a>) -> CollectedIdentifier<'a> {
        let range = node.get_text_range().clone();
        let file_id = node.get_file_id();
        let file_contents = self.file_contents(file_id);
        let (line, column) = Self::byte_offset_to_line_column(file_contents, range.start);
        CollectedIdentifier {
            node: node.clone(),
            line,
            column,
        }
    }

    fn byte_offset_to_line_column(contents: &str, byte_offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for (index, ch) in contents.char_indices() {
            if index >= byte_offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }
}

/// Collects definitions in order of appearance in the AST. File visiting order
/// is determined by `CompilationUnit` given to
/// `DefinitionCollector::collect_from()`.
struct DefinitionCollector<'a> {
    files: &'a FileSourceMap,
    all_definitions: Vec<CollectedDefinition<'a>>,
}

impl<'a> DefinitionCollector<'a> {
    fn collect_from(
        compilation: &'a CompilationUnit,
        files: &'a SortedMap<FileId, String>,
    ) -> Vec<CollectedDefinition<'a>> {
        let mut collector = Self {
            files,
            all_definitions: Vec::new(),
        };
        for file in compilation.files() {
            let source_unit = file.ast();
            accept_source_unit(&source_unit, &mut collector);
        }
        collector.all_definitions
    }
}

impl<'a> Visitor<'a> for DefinitionCollector<'a> {
    fn visit_identifier(&mut self, node: &Identifier<'a>) {
        let Some(definition) = node.named_definition() else {
            return;
        };
        let identifier = self.collect_identifier(node);
        self.all_definitions.push(CollectedDefinition {
            definition_id: self.all_definitions.len() + 1,
            definition: definition.clone(),
            identifier: identifier.clone(),
        });
    }
}

impl<'a> IdentifierCollector<'a> for DefinitionCollector<'a> {
    fn file_contents(&self, file_id: &FileId) -> &str {
        &self.files[file_id]
    }
}

// Identifiers collection and classification

struct ReferenceCollector<'a, 'm> {
    files: &'a SortedMap<FileId, String>,
    definitions_by_node_id: &'m Map<NodeId, DefinitionId>,
    all_references: Vec<CollectedReference<'a>>,
    unbound_identifiers: Vec<CollectedIdentifier<'a>>,
}

impl<'a, 'm> ReferenceCollector<'a, 'm> {
    fn collect_from(
        compilation: &'a CompilationUnit,
        files: &'a FileSourceMap,
        definitions_by_node_id: &'m Map<NodeId, DefinitionId>,
    ) -> (Vec<CollectedReference<'a>>, Vec<CollectedIdentifier<'a>>) {
        let mut collector = Self {
            files,
            definitions_by_node_id,
            all_references: Vec::new(),
            unbound_identifiers: Vec::new(),
        };
        for file in compilation.files() {
            let source_unit = file.ast();
            accept_source_unit(&source_unit, &mut collector);
        }
        (collector.all_references, collector.unbound_identifiers)
    }
}

impl<'a> Visitor<'a> for ReferenceCollector<'a, '_> {
    fn visit_identifier(&mut self, node: &Identifier<'a>) {
        let mut bound = false;

        // An identifier is a definition when it is the name of one of the
        // collected definitions (this includes definitions that are identifiers
        // by themselves, like enum members).
        if node.is_name_of_definition() {
            bound = true;
        }

        // The same identifier may additionally be acting as a reference (eg. the
        // name in an import deconstruction).
        if node.is_reference() {
            let identifier = self.collect_identifier(node);
            let resolution = if node.resolve_to_built_in().is_some() {
                CollectedResolution::BuiltIn
            } else if let Some(definition) = node.resolve_to_immediate_definition() {
                let definition_id = self
                    .definitions_by_node_id
                    .get(&definition.node_id())
                    .expect("resolution references an existing definition");
                CollectedResolution::Definition(*definition_id)
            } else {
                CollectedResolution::Unresolved
            };
            self.all_references.push(CollectedReference {
                identifier: identifier.clone(),
                resolution,
            });
            bound = true;
        }

        if !bound {
            let identifier = self.collect_identifier(node);
            self.unbound_identifiers.push(identifier);
        }
    }
}

impl<'a> IdentifierCollector<'a> for ReferenceCollector<'a, '_> {
    fn file_contents(&self, file_id: &FileId) -> &str {
        &self.files[file_id]
    }
}

// Display helpers

impl Display for CollectedDefinition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let identifier = &self.identifier;
        write!(
            f,
            "Def: #{id} [\"{identifier}\" @ {file_id}:{line}:{column}] ({def_type})",
            id = self.definition_id,
            identifier = identifier.node.name(),
            file_id = identifier.node.get_file_id(),
            def_type = definition_type(&self.definition),
            line = identifier.line,
            column = identifier.column,
        )
    }
}

impl Display for CollectedReference<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let identifier = &self.identifier;
        write!(
            f,
            "Ref: [\"{identifier}\" @ {file_id}:{line}:{column}] -> {definition}",
            identifier = identifier.node.name(),
            file_id = identifier.node.get_file_id(),
            definition = match &self.resolution {
                CollectedResolution::Unresolved => "unresolved".to_string(),
                CollectedResolution::BuiltIn => "built-in".to_string(),
                CollectedResolution::Definition(definition_id) => {
                    format!("#{definition_id}")
                }
            },
            line = identifier.line,
            column = identifier.column,
        )
    }
}

/// Displays a collected identifier as unbound by default
impl Display for CollectedIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "???: [\"{identifier}\" @ {file_id}:{line}:{column}]",
            identifier = self.node.name(),
            file_id = self.node.get_file_id(),
            line = self.line,
            column = self.column,
        )
    }
}

// Data display helpers

fn definition_name(definition: &Definition<'_>) -> String {
    definition.identifier().name().to_string()
}

fn definition_type(definition: &Definition<'_>) -> String {
    match definition {
        Definition::Constant(constant) => {
            format!(
                "constant, type: {}",
                type_or_unresolved(constant.get_type())
            )
        }
        Definition::Contract(_) => "contract".to_string(),
        Definition::Enum(_) => "enum".to_string(),
        Definition::EnumMember(enum_member) => {
            format!(
                "enum member of {}",
                type_or_unresolved(enum_member.get_type())
            )
        }
        Definition::Error(_) => "error".to_string(),
        Definition::Event(_) => "event".to_string(),
        Definition::Function(function) => {
            format!(
                "function, type: {}",
                type_or_unresolved(function.get_type())
            )
        }
        Definition::Import(_) => "import".to_string(),
        Definition::ImportedSymbol(_) => "imported symbol".to_string(),
        Definition::Interface(_) => "interface".to_string(),
        Definition::Library(_) => "library".to_string(),
        Definition::Modifier(_) => "modifier".to_string(),
        Definition::Parameter(parameter) => {
            format!(
                "parameter, type: {}",
                type_or_unresolved(parameter.get_type())
            )
        }
        Definition::StateVariable(state_variable) => {
            format!(
                "state var, type: {}",
                type_or_unresolved(state_variable.get_type())
            )
        }
        Definition::Struct(_) => "struct".to_string(),
        Definition::StructMember(struct_member) => {
            format!(
                "struct member, type: {}",
                type_or_unresolved(struct_member.get_type())
            )
        }
        Definition::TypeParameter(_) => "type param".to_string(),
        // A user-defined value type definition names the type itself, so its
        // typing is always the meta-type (it has no value type of its own).
        Definition::UserDefinedValueType(_) => "udvt, type: meta-type".to_string(),
        Definition::Variable(variable) => {
            format!(
                "variable, type: {}",
                type_or_unresolved(variable.get_type())
            )
        }
        Definition::YulFunction(_) => "yul function".to_string(),
        Definition::YulParameter(_) => "yul parameter".to_string(),
        Definition::YulVariable(_) => "yul variable".to_string(),
    }
}

fn type_or_unresolved(type_: Option<Type<'_>>) -> String {
    match type_ {
        Some(type_) => type_display(&type_),
        None => "unresolved".to_string(),
    }
}

#[allow(clippy::too_many_lines)]
fn type_display(type_: &Type<'_>) -> String {
    match type_ {
        Type::Address(address) => {
            if address.is_payable() {
                "address payable".to_string()
            } else {
                "address".to_string()
            }
        }
        Type::Array(array) => format!(
            "{element_type}[] {location}",
            element_type = type_display(&array.element_type()),
            location = data_location_display(array.location()),
        ),
        Type::ArraySlice(slice) => {
            format!("{array} slice", array = type_display(&slice.array_type()))
        }
        Type::Boolean(_) => "bool".to_string(),
        Type::ByteArray(byte_array) => format!("bytes{width}", width = byte_array.width()),
        Type::Bytes(bytes) => {
            format!(
                "bytes {location}",
                location = data_location_display(bytes.location())
            )
        }
        Type::Contract(contract) => definition_name(&contract.definition()),
        Type::Enum(enum_) => definition_name(&enum_.definition()),
        Type::Error(error) => format!("error {name}", name = definition_name(&error.definition())),
        Type::Event(event) => format!("event {name}", name = definition_name(&event.definition())),
        Type::FixedPointNumber(fixed) => {
            format!(
                "{signed}fixed{bits}x{precision_bits}",
                signed = if fixed.is_signed() { "" } else { "u" },
                bits = fixed.bits(),
                precision_bits = fixed.decimal_places(),
            )
        }
        Type::FixedSizeArray(fixed_size_array) => format!(
            "{element_type}[{size}] {location}",
            element_type = type_display(&fixed_size_array.element_type()),
            size = fixed_size_array.size(),
            location = data_location_display(fixed_size_array.location()),
        ),
        Type::Function(function) => {
            format!(
                "function ({parameters}) returns {returns}",
                parameters = function
                    .parameter_types()
                    .iter()
                    .map(type_display)
                    .collect::<Vec<_>>()
                    .join(", "),
                returns = type_display(&function.return_type()),
            )
        }
        Type::Integer(integer) => {
            format!(
                "{signed}int{bits}",
                signed = if integer.is_signed() { "" } else { "u" },
                bits = integer.bits(),
            )
        }
        Type::Interface(interface) => definition_name(&interface.definition()),
        Type::Library(library) => definition_name(&library.definition()),
        Type::Literal(literal) => match literal.kind() {
            LiteralKind::Integer { value } => format!("lit-integer({value})"),
            LiteralKind::HexInteger { value, bytes } => {
                format!("lit-hex({value}, {bytes})")
            }
            LiteralKind::Rational { value } => format!("lit-rational({value})"),
            LiteralKind::HexString { value } => format!("lit-hexstring({})", value.len()),
            LiteralKind::String { value } => format!("lit-string({})", value.len()),
        },
        Type::Mapping(mapping) => {
            format!(
                "{key} => {value}",
                key = type_display(&mapping.key_type()),
                value = type_display(&mapping.value_type()),
            )
        }
        // Meta-types are expression-only typings (the type of an expression
        // that refers to a type rather than a value). This report only renders
        // the types of value-bearing definitions, so a meta-type never reaches
        // here.
        Type::MetaType(_) | Type::UserMetaType(_) => {
            unreachable!("meta-types are not rendered as definition types")
        }
        Type::String(string) => {
            format!(
                "string {location}",
                location = data_location_display(string.location())
            )
        }
        Type::Struct(struct_) => {
            format!(
                "{name} {location}",
                name = definition_name(&struct_.definition()),
                location = data_location_display(struct_.location()),
            )
        }
        Type::Tuple(tuple) => {
            format!(
                "({types})",
                types = tuple
                    .types()
                    .iter()
                    .map(type_display)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Type::UserDefinedValue(user_defined_value) => {
            definition_name(&user_defined_value.definition())
        }
    }
}

fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}
