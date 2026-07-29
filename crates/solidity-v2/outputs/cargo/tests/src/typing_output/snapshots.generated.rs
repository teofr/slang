// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::typing_output::runner::run;

mod builtins {
    use super::*;

    #[test]
    fn msg_and_block() -> Result<()> {
        run("builtins", "msg_and_block")
    }
}

mod expressions {
    use super::*;

    #[test]
    fn arithmetic() -> Result<()> {
        run("expressions", "arithmetic")
    }

    #[test]
    fn conditional() -> Result<()> {
        run("expressions", "conditional")
    }

    #[test]
    fn long_snippet() -> Result<()> {
        run("expressions", "long_snippet")
    }

    #[test]
    fn named_return() -> Result<()> {
        run("expressions", "named_return")
    }

    #[test]
    fn number_literals() -> Result<()> {
        run("expressions", "number_literals")
    }

    #[test]
    fn references() -> Result<()> {
        run("expressions", "references")
    }
}

mod meta_types {
    use super::*;

    #[test]
    fn type_conversions() -> Result<()> {
        run("meta_types", "type_conversions")
    }
}

mod user_types {
    use super::*;

    #[test]
    fn wrap_unwrap() -> Result<()> {
        run("user_types", "wrap_unwrap")
    }
}
