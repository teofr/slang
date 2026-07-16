use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;
use crate::ir::NodeIdentity;

#[test]
fn test_node_id() {
    const CONTENTS: &str = r###"
string constant GREETING = "hello" "world";
contract MyContract {
    constructor() {}
    function withParams(uint256 first, bool second) public {}
}"###;

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&"test.sol".into(), CONTENTS, LanguageVersion::LATEST);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(
        &"test.sol".into(),
        &source_unit,
        &CONTENTS,
        &mut id_generator,
    );

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    let ir::SourceUnitMember::ConstantDefinition(ref constant) = ir_root.members[0] else {
        panic!("Expected ConstantDefinition");
    };
    let ir::SourceUnitMember::ContractDefinition(ref contract) = ir_root.members[1] else {
        panic!("Expected ContractDefinition");
    };

    // A sequence node reports its own `NodeId`.
    assert_eq!(contract.node_id(), Some(contract.id()));

    // A choice node delegates to its inner node.
    assert_eq!(ir_root.members[1].node_id(), Some(contract.id()));

    // A terminal node reports its own `NodeId`.
    assert_eq!(contract.name.node_id(), Some(contract.name.id()));

    let ir::ContractMember::FunctionDefinition(ref constructor) = contract.members[0] else {
        panic!("Expected FunctionDefinition for constructor");
    };
    let ir::ContractMember::FunctionDefinition(ref function) = contract.members[1] else {
        panic!("Expected FunctionDefinition");
    };

    // An empty collection has no `NodeId`.
    assert_eq!(constructor.parameters.node_id(), None);

    // A non-empty collection reports the `NodeId` of its first element.
    assert_eq!(
        function.parameters.node_id(),
        Some(function.parameters[0].id())
    );

    // External nodes are not represented in the source code, so they have no `NodeId`.
    assert_eq!(constructor.kind.node_id(), None);

    // Optional fields report the `NodeId` of their value, if any.
    assert_eq!(constructor.name.node_id(), None);

    // A choice node whose variant is itself a collection (eg. an `Expression`
    // holding a `StringExpression` made of one-or-more string literals) reports
    // the `NodeId` of the first terminal of that collection.
    let value = constant.value.as_ref().expect("constant has a value");
    let ir::Expression::StringExpression(string_expression) = value else {
        panic!("Expected StringExpression");
    };
    let ir::StringExpression::StringLiterals(literals) = string_expression else {
        panic!("Expected StringLiterals");
    };
    assert!(literals.len() > 1, "expected concatenated string literals");
    assert_eq!(value.node_id(), string_expression.node_id());
    assert_eq!(string_expression.node_id(), Some(literals[0].id()));
}
