//! Hand-written adjustments to the tree produced by the generated parser, for the few constructs
//! that the language definition cannot express.

use std::rc::Rc;

use crate::cst::{
    BaseRewriter, Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode, TerminalKind,
    TerminalKindExtensions,
};
use crate::parser::{ParseOutput, Parser};

/// Applied to the output of every parse, after the generated parser functions.
pub(crate) trait FixUp {
    fn fix_up(&self, output: ParseOutput) -> ParseOutput;
}

impl FixUp for Parser {
    fn fix_up(&self, mut output: ParseOutput) -> ParseOutput {
        if self.version_is_at_least_0_4_22 {
            let root = Node::Nonterminal(Rc::clone(&output.tree));
            if let Some(Node::Nonterminal(tree)) = FunctionTypeVisibility.rewrite_node(&root) {
                output.tree = tree;
            }
        }

        output
    }
}

/// `solc` ends the attributes of a function type at the second visibility, if the first one was
/// `internal` or `external`, leaving it to the enclosing state variable, as in
/// `function() external public x;`.
///
/// The generated parser consumes all of them as `FunctionTypeAttributes`, so this moves them
/// to the `StateVariableAttributes` that follow.
struct FunctionTypeVisibility;

impl BaseRewriter for FunctionTypeVisibility {
    fn rewrite_block(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        // Blocks cannot contain state variables. Skip traversing them:
        Some(Node::Nonterminal(Rc::clone(node)))
    }

    fn rewrite_state_variable_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(
            move_second_visibility(node)
                .map_or_else(|| Node::Nonterminal(Rc::clone(node)), Node::Nonterminal),
        )
    }
}

fn move_second_visibility(variable: &Rc<NonterminalNode>) -> Option<Rc<NonterminalNode>> {
    let type_name = child_with_label(variable, EdgeLabel::TypeName)?;
    let function_type = child_with_label(type_name, EdgeLabel::Variant)?;
    if function_type.kind != NonterminalKind::FunctionType {
        return None;
    }

    // `returns` is parsed after the attributes, so moving them would reorder the source text.
    // `solc` reports an error in that case anyway.
    if child_with_label(function_type, EdgeLabel::Returns).is_some() {
        return None;
    }

    let function_attributes = child_with_label(function_type, EdgeLabel::Attributes)?;
    let split = split_index(&function_attributes.children)?;

    let (kept, moved) = function_attributes.children.split_at(split);

    // `solc` reports an error for anything else, and it has no `StateVariableAttribute` to map to:
    if !moved.iter().all(|item| {
        matches!(
            first_terminal_kind(&item.node),
            Some(
                TerminalKind::InternalKeyword
                    | TerminalKind::PrivateKeyword
                    | TerminalKind::PublicKeyword
                    | TerminalKind::ConstantKeyword
            )
        )
    }) {
        return None;
    }

    let function_attributes =
        NonterminalNode::create(NonterminalKind::FunctionTypeAttributes, kept.to_vec());
    let function_type = replace_child(function_type, EdgeLabel::Attributes, function_attributes);
    let type_name = replace_child(type_name, EdgeLabel::Variant, function_type);

    let variable_attributes = child_with_label(variable, EdgeLabel::Attributes)?;
    let variable_attributes = NonterminalNode::create(
        NonterminalKind::StateVariableAttributes,
        moved
            .iter()
            .map(|item| Edge {
                label: item.label,
                node: Node::nonterminal(
                    NonterminalKind::StateVariableAttribute,
                    item.node.children().to_vec(),
                ),
            })
            .chain(variable_attributes.children.iter().cloned())
            .collect(),
    );

    let variable = replace_child(variable, EdgeLabel::TypeName, type_name);
    Some(replace_child(
        &variable,
        EdgeLabel::Attributes,
        variable_attributes,
    ))
}

/// Returns the index of the first visibility that follows an `internal` or `external` one.
fn split_index(items: &[Edge]) -> Option<usize> {
    let mut armed = false;

    for (index, item) in items.iter().enumerate() {
        match first_terminal_kind(&item.node)? {
            TerminalKind::InternalKeyword
            | TerminalKind::ExternalKeyword
            | TerminalKind::PrivateKeyword
            | TerminalKind::PublicKeyword
                if armed =>
            {
                return Some(index);
            }
            TerminalKind::InternalKeyword | TerminalKind::ExternalKeyword => armed = true,
            _ => {}
        }
    }

    None
}

fn child_with_label(node: &Rc<NonterminalNode>, label: EdgeLabel) -> Option<&Rc<NonterminalNode>> {
    node.children
        .iter()
        .find(|edge| edge.label == label)
        .and_then(|edge| edge.node.as_nonterminal())
}

fn replace_child(
    node: &Rc<NonterminalNode>,
    label: EdgeLabel,
    child: Rc<NonterminalNode>,
) -> Rc<NonterminalNode> {
    let mut children = node.children.clone();
    if let Some(edge) = children.iter_mut().find(|edge| edge.label == label) {
        edge.node = Node::Nonterminal(child);
    }

    NonterminalNode::create(node.kind, children)
}

fn first_terminal_kind(node: &Node) -> Option<TerminalKind> {
    match node {
        Node::Terminal(terminal) => Some(terminal.kind),
        Node::Nonterminal(nonterminal) => nonterminal
            .children
            .iter()
            .filter(|edge| !matches!(&edge.node, Node::Terminal(t) if t.kind.is_trivia()))
            .find_map(|edge| first_terminal_kind(&edge.node)),
    }
}
