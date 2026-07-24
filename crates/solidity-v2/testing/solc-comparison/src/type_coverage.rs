//! Walks a compiled unit's AST and reports the in-scope nodes whose
//! [`get_type()`](slang_solidity_v2::ast) returns `None` — i.e. positions where
//! slang doesn't (yet) assign a type.
//!
//! "In scope" is **expression nodes plus typed declarations** (parameters,
//! state variables, constants, and variable declarations) — the positions that
//! semantically carry a type. Control-flow statements, pragmas, and the like
//! have no type by design and are ignored.
//!
//! Expressions are visited through the enum-level [`Visitor::enter_expression`]
//! hook, which fires exactly once per expression position. This deliberately
//! excludes identifiers that appear as *definition names* (a contract, function,
//! or struct name), which reuse the same `Identifier` node but aren't
//! expressions and carry no type — only identifiers used as expressions (i.e.
//! references) are `Expression::Identifier`, so only those are checked.
//!
//! Two expression variants are **not** checked: `StringExpression` and
//! `ElementaryType` (an elementary type used as an expression, e.g. the callee
//! in a `uint(x)` conversion). Both are enums that wrap terminal lists rather
//! than a single located node, so they expose no uniform text-range accessor.
//! They're skipped via explicit match arms (not a wildcard) so this stays
//! visible and a newly-added variant is still a compile error.

use std::ops::Range;

use slang_solidity_v2::ast::visitor::{accept_source_unit, Visitor};
use slang_solidity_v2::ast::{
    ConstantDefinition, Expression, Parameter, StateVariableDefinition, VariableDeclaration,
};
use slang_solidity_v2::compilation::CompilationUnit;
use slang_solidity_v2_common::files::FileId;

/// Lists every in-scope node in `unit` whose `get_type()` is `None`, each
/// rendered as `"<kind> <file>:<start>..<end>"`, sorted for determinism. An
/// empty result means every in-scope node has a type.
pub fn untyped_nodes(unit: &CompilationUnit) -> Vec<String> {
    let mut visitor = UntypedCollector::default();
    for file in unit.files() {
        accept_source_unit(&file.ast(), &mut visitor);
    }
    visitor.untyped.sort();
    visitor.untyped
}

#[derive(Default)]
struct UntypedCollector {
    untyped: Vec<String>,
}

impl UntypedCollector {
    fn record(&mut self, typed: bool, kind: &str, file_id: &FileId, range: &Range<usize>) {
        if !typed {
            self.untyped
                .push(format!("{kind} {file_id}:{}..{}", range.start, range.end));
        }
    }
}

/// The `(kind, file_id, range)` of an [`Expression`], or `None` for the two
/// variants this tool skips (see the module docs). Matched exhaustively so a
/// new variant is a compile error, which keeps this tool honest as the AST
/// grows. Location accessors live on the concrete node structs, not the enum,
/// hence the match.
fn expression_location(node: &Expression) -> Option<(&'static str, &FileId, &Range<usize>)> {
    macro_rules! located {
        ($($variant:ident),* $(,)?) => {
            match node {
                // Enums over terminal lists, with no single text range: skipped.
                Expression::StringExpression(_) | Expression::ElementaryType(_) => None,
                $(
                    Expression::$variant(inner) => {
                        Some((stringify!($variant), inner.get_file_id(), inner.get_text_range()))
                    }
                )*
            }
        };
    }

    located! {
        AssignmentExpression, ConditionalExpression, OrExpression, AndExpression,
        EqualityExpression, InequalityExpression, BitwiseOrExpression, BitwiseXorExpression,
        BitwiseAndExpression, ShiftExpression, AdditiveExpression, MultiplicativeExpression,
        ExponentiationExpression, PostfixExpression, PrefixExpression, FunctionCallExpression,
        CallOptionsExpression, MemberAccessExpression, IndexAccessExpression, NewExpression,
        TupleExpression, TypeExpression, ArrayExpression, HexNumberExpression,
        DecimalNumberExpression, PayableKeyword, ThisKeyword, SuperKeyword, TrueKeyword,
        FalseKeyword, Identifier,
    }
}

impl Visitor for UntypedCollector {
    // Every expression position, checked uniformly via the enum's `get_type()`.
    fn enter_expression(&mut self, node: &Expression) -> bool {
        if let Some((kind, file_id, range)) = expression_location(node) {
            self.record(node.get_type().is_some(), kind, file_id, range);
        }
        true
    }

    // Typed declarations: the declared entity carries a type of its own.
    fn enter_parameter(&mut self, node: &Parameter) -> bool {
        self.record(
            node.get_type().is_some(),
            "Parameter",
            node.get_file_id(),
            node.get_text_range(),
        );
        true
    }

    fn enter_state_variable_definition(&mut self, node: &StateVariableDefinition) -> bool {
        self.record(
            node.get_type().is_some(),
            "StateVariableDefinition",
            node.get_file_id(),
            node.get_text_range(),
        );
        true
    }

    fn enter_constant_definition(&mut self, node: &ConstantDefinition) -> bool {
        self.record(
            node.get_type().is_some(),
            "ConstantDefinition",
            node.get_file_id(),
            node.get_text_range(),
        );
        true
    }

    fn enter_variable_declaration(&mut self, node: &VariableDeclaration) -> bool {
        self.record(
            node.get_type().is_some(),
            "VariableDeclaration",
            node.get_file_id(),
            node.get_text_range(),
        );
        true
    }
}
