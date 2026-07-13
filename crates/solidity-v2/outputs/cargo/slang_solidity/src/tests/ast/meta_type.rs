//! Exercises the public AST wrappers for meta-types (`Type::MetaType` and
//! `Type::UserMetaType`) and their accessors, reached through
//! `Expression::get_type()`.

use crate::abi::AbiType;
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
        // `uint(x)` — the elementary-type operand `uint` carries a stored
        // `MetaType` typing of its own.
        uint(x);
    }
}
"#,
);

/// Collects, for every function call: the `get_type()` of its operand (eg. `S`
/// in `S(x)`) and whether it is a type conversion; plus the `get_type()` of
/// each index-access type expression (eg. `uint[]`).
#[derive(Default)]
struct MetaTypeCollector {
    types: Vec<ast::Type>,
    conversions: Vec<bool>,
}

impl ast::visitor::Visitor for MetaTypeCollector {
    fn enter_function_call_expression(&mut self, node: &ast::FunctionCallExpression) -> bool {
        if let Some(type_) = node.operand().get_type() {
            self.types.push(type_);
        }
        self.conversions.push(node.is_type_conversion());
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

    // `Type::MetaType` wraps the type it is the meta-type of: an array for the
    // `uint[]` type expression, and an integer for the `uint` cast operand
    // (whose typing is stored on the elementary-type node itself).
    let metas = collector
        .types
        .iter()
        .filter_map(|type_| match type_ {
            ast::Type::MetaType(meta) => Some(meta.meta_type()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        metas.iter().any(|meta| matches!(meta, ast::Type::Array(_))),
        "expected a `Type::MetaType` wrapping an array from the `uint[]` type expression",
    );
    assert!(
        metas
            .iter()
            .any(|meta| matches!(meta, ast::Type::Integer(_))),
        "expected a `Type::MetaType` wrapping an integer from the `uint(x)` cast operand",
    );

    // `is_type_conversion` in source order: `S(x)` is a construction through a
    // type name (meta operand), `abi.decode(...)` is a plain call, `uint(x)`
    // is an elementary cast.
    assert_eq!(collector.conversions, vec![true, false, true]);

    // Meta-types have no ABI representation.
    for type_ in &collector.types {
        if matches!(type_, ast::Type::MetaType(_) | ast::Type::UserMetaType(_)) {
            assert!(AbiType::try_from(type_).is_err());
        }
    }
}
