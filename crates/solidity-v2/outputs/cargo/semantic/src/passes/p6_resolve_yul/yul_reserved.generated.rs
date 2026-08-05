// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

/// Whether `name` is reserved as a Yul identifier for the given language
/// version, without being an available built-in. These names may not be
/// declared in inline assembly, yet have no built-in signature to resolve to
/// (opcode mnemonics like `dup1`/`push0`/`jump`, object-access names like
/// `datasize`, and names promoted to reserved such as `difficulty`).
///
/// This mirrors `solc`'s reserved-identifier check (error 5017): the name is
/// still parsed as an ordinary identifier, and the reservation is a semantic
/// fact keyed by name and language version.
pub(crate) fn yul_reserved(name: &str, language_version: LanguageVersion) -> bool {
    match name {
        "dup1" => true,
        "dup2" => true,
        "dup3" => true,
        "dup4" => true,
        "dup5" => true,
        "dup6" => true,
        "dup7" => true,
        "dup8" => true,
        "dup9" => true,
        "dup10" => true,
        "dup11" => true,
        "dup12" => true,
        "dup13" => true,
        "dup14" => true,
        "dup15" => true,
        "dup16" => true,
        "swap1" => true,
        "swap2" => true,
        "swap3" => true,
        "swap4" => true,
        "swap5" => true,
        "swap6" => true,
        "swap7" => true,
        "swap8" => true,
        "swap9" => true,
        "swap10" => true,
        "swap11" => true,
        "swap12" => true,
        "swap13" => true,
        "swap14" => true,
        "swap15" => true,
        "swap16" => true,
        "push1" => true,
        "push2" => true,
        "push3" => true,
        "push4" => true,
        "push5" => true,
        "push6" => true,
        "push7" => true,
        "push8" => true,
        "push9" => true,
        "push10" => true,
        "push11" => true,
        "push12" => true,
        "push13" => true,
        "push14" => true,
        "push15" => true,
        "push16" => true,
        "push17" => true,
        "push18" => true,
        "push19" => true,
        "push20" => true,
        "push21" => true,
        "push22" => true,
        "push23" => true,
        "push24" => true,
        "push25" => true,
        "push26" => true,
        "push27" => true,
        "push28" => true,
        "push29" => true,
        "push30" => true,
        "push31" => true,
        "push32" => true,
        "jump" => true,
        "jumpi" => true,
        "jumpdest" => true,
        "datasize" => true,
        "dataoffset" => true,
        "datacopy" => true,
        "linkersymbol" => true,
        "setimmutable" => true,
        "loadimmutable" => true,
        "difficulty" => true,
        "push0" => {
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_20).contains(language_version)
        }
        _ => false,
    }
}
