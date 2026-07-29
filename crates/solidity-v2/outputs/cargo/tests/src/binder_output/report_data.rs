use std::fmt::Display;
use std::ops::Range;

use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::ast::{Definition, Identifier, NodeId};
use slang_solidity_v2::compilation::{CompilationUnit, FileId};
use slang_solidity_v2_common::collections::{Map, SortedMap};

use crate::utils::type_display::{definition_value_type, type_or_unresolved};

// Types

type FileSourceMap = SortedMap<FileId, String>;

pub(crate) struct ReportData<'a> {
    pub(crate) compilation: &'a CompilationUnit,
    pub(crate) files: &'a FileSourceMap,
    pub(crate) all_definitions: Vec<CollectedDefinition>,
    pub(crate) all_references: Vec<CollectedReference>,
    pub(crate) unbound_identifiers: Vec<CollectedIdentifier>,
}

/// This `DefinitionId` is local to the `ReportData` and represents the position
/// in the `all_definitions` vector of a given definition (strictly its index+1).
pub(crate) type DefinitionId = usize;

#[derive(Clone)]
pub(crate) struct CollectedIdentifier {
    node: Identifier,
    line: usize,
    column: usize,
}

impl CollectedIdentifier {
    pub(crate) fn file_id(&self) -> &FileId {
        self.node.get_file_id()
    }
    pub(crate) fn range(&self) -> &Range<usize> {
        self.node.get_text_range()
    }
}

pub(crate) struct CollectedDefinition {
    // This is the index+1 of this definition in the `all_definitions` vector.
    pub(crate) definition_id: DefinitionId,
    pub(crate) identifier: CollectedIdentifier,
    definition: Definition,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum CollectedResolution {
    Unresolved,
    BuiltIn,
    // The `DefinitionId` payload is internal to the `ReportData`
    Definition(DefinitionId),
}

pub(crate) struct CollectedReference {
    pub(crate) identifier: CollectedIdentifier,
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
        self.compilation.diagnostics().is_empty()
            && self.unbound_identifiers.is_empty()
            && self
                .all_references
                .iter()
                .all(|reference| reference.resolution != CollectedResolution::Unresolved)
    }
}

// Identifier collector trait, to allow reuse in the two collectors

trait IdentifierCollector {
    fn file_contents(&self, file_id: &FileId) -> &str;

    fn collect_identifier(&self, node: &Identifier) -> CollectedIdentifier {
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
    all_definitions: Vec<CollectedDefinition>,
}

impl<'a> DefinitionCollector<'a> {
    fn collect_from(
        compilation: &CompilationUnit,
        files: &'a SortedMap<FileId, String>,
    ) -> Vec<CollectedDefinition> {
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

impl Visitor for DefinitionCollector<'_> {
    fn visit_identifier(&mut self, node: &Identifier) {
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

impl IdentifierCollector for DefinitionCollector<'_> {
    fn file_contents(&self, file_id: &FileId) -> &str {
        &self.files[file_id]
    }
}

// Identifiers collection and classification

struct ReferenceCollector<'a> {
    files: &'a SortedMap<FileId, String>,
    definitions_by_node_id: &'a Map<NodeId, DefinitionId>,
    all_references: Vec<CollectedReference>,
    unbound_identifiers: Vec<CollectedIdentifier>,
}

impl<'a> ReferenceCollector<'a> {
    fn collect_from(
        compilation: &CompilationUnit,
        files: &'a FileSourceMap,
        definitions_by_node_id: &'a Map<NodeId, DefinitionId>,
    ) -> (Vec<CollectedReference>, Vec<CollectedIdentifier>) {
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

impl Visitor for ReferenceCollector<'_> {
    fn visit_identifier(&mut self, node: &Identifier) {
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

impl IdentifierCollector for ReferenceCollector<'_> {
    fn file_contents(&self, file_id: &FileId) -> &str {
        &self.files[file_id]
    }
}

// Display helpers

impl Display for CollectedDefinition {
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

impl Display for CollectedReference {
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
impl Display for CollectedIdentifier {
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

fn definition_type(definition: &Definition) -> String {
    // The type of value-bearing definitions comes from the shared
    // `definition_value_type`, so binder_output and typing_output agree.
    match definition {
        Definition::Constant(_) => {
            format!(
                "constant, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::Contract(_) => "contract".to_string(),
        Definition::Enum(_) => "enum".to_string(),
        Definition::EnumMember(_) => {
            format!(
                "enum member of {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::Error(_) => "error".to_string(),
        Definition::Event(_) => "event".to_string(),
        Definition::Function(_) => {
            format!(
                "function, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::Import(_) => "import".to_string(),
        Definition::ImportedSymbol(_) => "imported symbol".to_string(),
        Definition::Interface(_) => "interface".to_string(),
        Definition::Library(_) => "library".to_string(),
        Definition::Modifier(_) => "modifier".to_string(),
        Definition::Parameter(_) => {
            format!(
                "parameter, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::StateVariable(_) => {
            format!(
                "state var, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::Struct(_) => "struct".to_string(),
        Definition::StructMember(_) => {
            format!(
                "struct member, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::TypeParameter(_) => "type param".to_string(),
        // A user-defined value type definition names the type itself, so its
        // typing is always the meta-type (it has no value type of its own).
        Definition::UserDefinedValueType(_) => "udvt, type: meta-type".to_string(),
        Definition::Variable(_) => {
            format!(
                "variable, type: {}",
                type_or_unresolved(definition_value_type(definition))
            )
        }
        Definition::YulFunction(_) => "yul function".to_string(),
        Definition::YulParameter(_) => "yul parameter".to_string(),
        Definition::YulVariable(_) => "yul variable".to_string(),
    }
}
