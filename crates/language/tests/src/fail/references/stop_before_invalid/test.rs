#![allow(unused_crate_dependencies)]

language_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Repeated(
                    name = Bar,
                    reference = Baz,
                    stop_before = RepeatedStopBefore(
                        after = [Baz1],
                        before = [Baz1, Baz3],
                        enabled = Till("2.0.0")
                    )
                ),
                Enum(
                    name = Baz,
                    variants = [EnumVariant(reference = Baz1), EnumVariant(reference = Baz2)]
                ),
                Token(
                    name = Baz1,
                    definitions = [TokenDefinition(scanner = Atom("baz1"))]
                ),
                Token(
                    name = Baz2,
                    definitions = [TokenDefinition(scanner = Atom("baz2"))]
                ),
                Token(
                    name = Baz3,
                    definitions = [TokenDefinition(scanner = Atom("baz3"))]
                )
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
