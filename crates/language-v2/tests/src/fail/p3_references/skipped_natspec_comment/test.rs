#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    versions = ["1.0.0"],
    evm_targets = [],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Trivia(
                        name = Whitespace,
                        skipped = true,
                        scanner = Atom(" ")
                    ),
                    Trivia(
                        name = NatSpecComment,
                        category = NatSpecComment,
                        skipped = true,
                        scanner = Atom("///")
                    ),
                    Struct(name = One, fields = (two = Required(Two))),
                    Token(name = Two, scanner = Atom("two"))
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
