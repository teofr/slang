use std::ops::Range;

use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::ast::{Expression, Identifier, SourceUnit};
use slang_solidity_v2::compilation::{CompilationUnit, FileId};
use slang_solidity_v2_common::collections::{Map, SortedMap};

use crate::utils::type_display::{definition_value_type, type_display};

type FileSourceMap = SortedMap<FileId, String>;

pub(crate) struct ReportData<'a> {
    pub(crate) compilation: &'a CompilationUnit,
    pub(crate) files: &'a FileSourceMap,
    pub(crate) typings_by_file: Map<FileId, Vec<CollectedTyping>>,
}

/// A source range together with the type the typing pass assigned to it (or
/// `None` when no type was recorded). Covers both expressions and the name
/// identifiers of value-bearing definitions.
pub(crate) struct CollectedTyping {
    pub(crate) range: Range<usize>,
    type_display: Option<String>,
}

impl CollectedTyping {
    /// The label shown for this node on the annotated source, ie. its rendered
    /// type or `<no type>`.
    pub(crate) fn type_label(&self) -> String {
        self.type_display
            .clone()
            .unwrap_or_else(|| "<no type>".to_string())
    }
}

impl<'a> ReportData<'a> {
    pub(crate) fn prepare(compilation: &'a CompilationUnit, files: &'a FileSourceMap) -> Self {
        let typings_by_file = compilation
            .files()
            .map(|file| (file.id().clone(), TypingCollector::collect(&file.ast())))
            .collect();

        Self {
            compilation,
            files,
            typings_by_file,
        }
    }

    /// A snapshot is a success when the source was analyzed without any
    /// diagnostics.
    ///
    /// Note that a successful analysis can still leave some expressions
    /// untyped: built-in namespaces (eg. `msg`, `block`) and the callee of a
    /// call (eg. `T.wrap` in `T.wrap(x)`) legitimately carry no value type.
    /// Those are shown as `<no type>`; a change to any node's type is caught by
    /// the snapshot diff regardless of status.
    pub(crate) fn is_success(&self) -> bool {
        self.compilation.diagnostics().is_empty()
    }
}

/// Collects the typings of a single file, in source order: every expression,
/// plus the name identifier of each value-bearing definition.
#[derive(Default)]
struct TypingCollector {
    typings: Vec<CollectedTyping>,
}

impl TypingCollector {
    fn collect(source_unit: &SourceUnit) -> Vec<CollectedTyping> {
        let mut collector = Self::default();
        accept_source_unit(source_unit, &mut collector);
        collector.typings
    }
}

impl Visitor for TypingCollector {
    fn enter_expression(&mut self, node: &Expression) -> bool {
        let type_display = node.get_type().map(|type_| type_display(&type_));

        self.typings.push(CollectedTyping {
            range: node.get_text_range(),
            type_display,
        });

        // Keep descending so nested expressions are reported too.
        true
    }

    fn visit_identifier(&mut self, node: &Identifier) {
        // Annotate the declaration site of value-bearing definitions with their
        // declared type. References/uses are identifiers too, but they are
        // covered as expressions by `enter_expression`, so only definition
        // names (which are not expressions) are collected here.
        let Some(definition) = node.named_definition() else {
            return;
        };
        let Some(type_) = definition_value_type(&definition) else {
            return;
        };

        self.typings.push(CollectedTyping {
            range: node.get_text_range().clone(),
            type_display: Some(type_display(&type_)),
        });
    }
}
