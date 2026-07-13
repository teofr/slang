//! Exercises the public AST wrappers for meta-types (`Type::MetaType` and
//! `Type::UserMetaType`) and their accessors, reached through
//! `Expression::get_type()`.

use crate::{ast, define_fixture};

define_fixture!(
    MetaTypes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract C {
    struct S { uint a; }

    function f(uint x, bytes memory b) internal pure {
        // `S(x)` — the operand `S` carries a `UserMetaType`.
        S(x);
        // `(uint[])` — the index-access type expression carries a `MetaType`
        // (of an array).
        abi.decode(b, (uint[]));
    }
}
"#,
);

/// Collects the `get_type()` of every meta-type-bearing expression: the operand
/// of each function call (eg. `S` in `S(x)`) and each index-access type
/// expression (eg. `uint[]`).
#[derive(Default)]
struct MetaTypeCollector {
    types: Vec<ast::Type>,
}

impl ast::visitor::Visitor for MetaTypeCollector {
    fn enter_function_call_expression(&mut self, node: &ast::FunctionCallExpression) -> bool {
        if let Some(type_) = node.operand().get_type() {
            self.types.push(type_);
        }
        true
    }

    fn enter_index_access_expression(&mut self, node: &ast::IndexAccessExpression) -> bool {
        if let Some(type_) = node.get_type() {
            self.types.push(type_);
        }
        true
    }
}

#[test]
fn meta_type_ast_wrappers_are_reachable_via_get_type() {
    let unit = MetaTypes::build_compilation_unit();
    let ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut collector = MetaTypeCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // `Type::UserMetaType` resolves back to the named definition.
    let user_meta = collector
        .types
        .iter()
        .find_map(|type_| match type_ {
            ast::Type::UserMetaType(user) => Some(user),
            _ => None,
        })
        .expect("expected a `Type::UserMetaType` from the `S(x)` operand");
    assert_eq!(user_meta.definition().identifier().name(), "S");

    // `Type::MetaType` wraps the type it is the meta-type of (here, an array).
    let meta = collector
        .types
        .iter()
        .find_map(|type_| match type_ {
            ast::Type::MetaType(meta) => Some(meta),
            _ => None,
        })
        .expect("expected a `Type::MetaType` from the `uint[]` type expression");
    assert!(
        matches!(meta.meta_type(), ast::Type::Array(_)),
        "expected the `uint[]` meta-type to wrap an array",
    );
}
