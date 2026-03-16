use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::{FileWalker, PathExtensions};
use infra_utils::terminal::Terminal;
use strum::IntoEnumIterator;

use crate::toolchains::mkdocs::Mkdocs;
use crate::toolchains::pipenv::PipEnv;
use crate::toolchains::public_api::UserFacingCrate;
use crate::utils::ClapExtensions;

#[derive(Clone, Copy)]
enum LintMode {
    /// Check-only: report violations and fail without modifying files.
    Fail,
    /// Auto-fix: automatically fix violations when possible.
    Fix,
}

impl LintMode {
    fn should_fix(self) -> bool {
        matches!(self, LintMode::Fix)
    }
}

#[derive(Clone, Debug, Default, Parser)]
pub struct LintController {
    /// Check mode: report violations and fail without modifying files.
    #[arg(long, conflicts_with = "fix")]
    fail: bool,

    /// Fix mode: automatically fix violations when possible.
    #[arg(long, conflicts_with = "fail")]
    fix: bool,

    #[clap(trailing_var_arg = true)]
    commands: Vec<LintCommand>,
}

impl LintController {
    fn resolve_mode(&self) -> LintMode {
        if self.fail {
            LintMode::Fail
        } else if self.fix {
            LintMode::Fix
        } else if GitHub::is_running_in_ci() {
            LintMode::Fail
        } else {
            LintMode::Fix
        }
    }

    pub fn execute(&self) -> Result<()> {
        let mode = self.resolve_mode();

        let mut commands = self.commands.clone();

        if commands.is_empty() {
            // Execute all commands if none are provided:
            commands.extend(LintCommand::value_variants().iter().cloned());
        } else {
            // Sort and deduplicate user provided commands by order of definition:
            commands.sort();
            commands.dedup();
        }

        for command in commands {
            command.execute_with_mode(mode)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum LintCommand {
    /// Run `cargo clippy` to check for any Rust lints.
    Clippy,
    /// Run `cargo doc` to generate Rustdoc documentation and check for any broken links.
    Rustdoc,
    /// Run `cargo test --doc` to ensure code in rustdoc comments is valid.
    RustdocTest,
    /// Check mkdocs documentation for any build issues or broken links.
    Mkdocs,
    /// Check for spelling issues in Markdown files.
    Cspell,
    /// Format all non-Rust source files.
    Prettier,
    /// Check for broken links in Markdown files.
    MarkdownLinkCheck,
    /// Check for violations in Markdown files.
    MarkdownLint,
    /// Format all Rust source files.
    Rustfmt,
    /// Check for violations in Bash files.
    Shellcheck,
    /// Check for type errors in TypeScript files.
    Tsc,
    /// Check for violations issues in Yaml files.
    Yamllint,
    /// Check for violations in TypeScript documentation files.
    Typedoc,
}

impl LintCommand {
    fn execute_with_mode(&self, mode: LintMode) -> Result<()> {
        Terminal::step(format!("lint {name}", name = self.clap_name()));

        match self {
            LintCommand::Clippy => run_clippy(mode),
            LintCommand::Rustdoc => run_rustdoc(mode),
            LintCommand::RustdocTest => run_rustdoc_test(),
            LintCommand::Mkdocs => run_mkdocs(),
            LintCommand::Cspell => run_cspell(),
            LintCommand::Prettier => run_prettier(mode),
            LintCommand::MarkdownLinkCheck => run_markdown_link_check()?,
            LintCommand::MarkdownLint => run_markdown_lint(mode)?,
            LintCommand::Rustfmt => run_rustfmt(mode),
            LintCommand::Shellcheck => run_shellcheck()?,
            LintCommand::Tsc => run_tsc(),
            LintCommand::Yamllint => run_yamllint()?,
            LintCommand::Typedoc => run_typedoc()?,
        }

        Ok(())
    }
}

fn run_clippy(mode: LintMode) {
    let mut command = Command::new("cargo")
        .arg("clippy")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--all-targets")
        .flag("--no-deps");

    if mode.should_fix() {
        command = command
            .flag("--fix")
            .flag("--allow-dirty")
            .flag("--allow-staged");
    } else {
        command = command.add_strict_build_rustflags();
    }

    command.run();
}

fn run_rustdoc(mode: LintMode) {
    let mut command = Command::new("cargo")
        .arg("doc")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--no-deps")
        .flag("--document-private-items");

    if !mode.should_fix() {
        command = command.add_strict_build_rustflags();
    }

    command.run();
}

fn run_rustdoc_test() {
    let mut command = Command::new("cargo")
        .arg("test")
        .flag("--doc")
        .flag("--all-features");

    for crate_name in UserFacingCrate::iter() {
        command = command.property("--package", crate_name.to_string());
    }
    command.run();
}

fn run_mkdocs() {
    Mkdocs::check();
}

fn run_cspell() {
    Command::new("cspell")
        .arg("lint")
        .flag("--show-context")
        .flag("--show-suggestions")
        .flag("--dot")
        .flag("--gitignore")
        .run();
}

fn run_prettier(mode: LintMode) {
    if mode.should_fix() {
        Command::new("prettier").property("--write", ".").run();
    } else {
        Command::new("prettier").property("--check", ".").run();
    }
}

fn run_markdown_link_check() -> Result<()> {
    let config_file = Path::repo_path(".markdown-link-check.json");

    let markdown_files = FileWalker::from_repo_root().find(["**/*.md"])?;

    Command::new("markdown-link-check")
        .property("--config", config_file.unwrap_str())
        .run_xargs(markdown_files);

    Ok(())
}

fn run_markdown_lint(mode: LintMode) -> Result<()> {
    let markdown_files = FileWalker::from_repo_root()
        .find(["**/*.md"])?
        .inspect(|path| println!("{}", path.display()));

    let mut command = Command::new("markdownlint").flag("--dot");

    if mode.should_fix() {
        command = command.flag("--fix");
    }

    command.run_xargs(markdown_files);

    Ok(())
}

fn run_rustfmt(mode: LintMode) {
    let mut command = Command::new("cargo-fmt")
        .arg(format!("+{}", env!("RUST_NIGHTLY_VERSION")))
        .flag("--all")
        .flag("--verbose");

    if !mode.should_fix() {
        command = command.flag("--check");
    }

    command.run();
}

fn run_shellcheck() -> Result<()> {
    let bash_files = FileWalker::from_repo_root()
        .find(["scripts/**"])?
        .inspect(|path| println!("{}", path.display()));

    Command::new("shellcheck").run_xargs(bash_files);

    Ok(())
}

fn run_tsc() {
    let root_project = Path::repo_path("tsconfig.json");

    Command::new("tsc")
        .property("--build", root_project.unwrap_str())
        .run();
}

fn run_yamllint() -> Result<()> {
    let config_file = Path::repo_path(".yamllint.yml");

    let yaml_files = FileWalker::from_repo_root()
        .find(["**/*.yml", "**/*.yaml", "!pnpm-lock.yaml"])?
        .inspect(|path| println!("{}", path.display()));

    PipEnv::run("yamllint")
        .flag("--strict")
        .property("--config-file", config_file.unwrap_str())
        .run_xargs(yaml_files);

    Ok(())
}

fn run_typedoc() -> Result<()> {
    let options_files = FileWalker::from_repo_root().find(["**/typedoc.mjs"])?;

    for options_file in options_files {
        Command::new("typedoc")
            .property("--options", options_file.unwrap_string())
            .run();
    }

    Ok(())
}
