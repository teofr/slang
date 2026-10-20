use std::cell::Cell;

use crate::cst::{Edge, Node, TerminalKind, TerminalKindExtensions};
use crate::parser::parser_support::parser_result::ParserResult;

/// Ends a repetition early, as described by `RepeatedStopBefore` in the language definition:
/// once an item starting with one of the `after` terminals has been matched, any item starting
/// with one of the `before` terminals is rejected, leaving it to the enclosing parser.
///
/// A fresh helper must be created for each run of the repetition.
#[derive(Default)]
pub struct StopBeforeHelper {
    armed: Cell<bool>,
}

impl StopBeforeHelper {
    pub fn run(
        &self,
        after: &[TerminalKind],
        before: &[TerminalKind],
        result: ParserResult,
    ) -> ParserResult {
        let ParserResult::Match(r#match) = &result else {
            return result;
        };

        let Some(first) = first_terminal_kind(&r#match.nodes) else {
            return result;
        };

        if self.armed.get() && before.contains(&first) {
            // The repetition rewinds this item, and stops:
            return ParserResult::no_match(vec![]);
        }

        if after.contains(&first) {
            self.armed.set(true);
        }

        result
    }
}

fn first_terminal_kind(edges: &[Edge]) -> Option<TerminalKind> {
    edges.iter().find_map(|edge| match &edge.node {
        Node::Terminal(terminal) if terminal.kind.is_trivia() => None,
        Node::Terminal(terminal) => Some(terminal.kind),
        Node::Nonterminal(nonterminal) => first_terminal_kind(&nonterminal.children),
    })
}
