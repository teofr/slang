use language_v2_definition::model::{EvmTargetSpecifier, Language, VersionSpecifier};
use serde::Serialize;

#[derive(Serialize)]
pub struct YulReservedWordModel {
    pub name: String,
    pub enabled: VersionSpecifier,
    pub evm_enabled: EvmTargetSpecifier,
}

pub fn build_yul_reserved_words_model(language: &Language) -> Vec<YulReservedWordModel> {
    language
        .yul_reserved_words
        .iter()
        .flatten()
        .map(|word| YulReservedWordModel {
            name: word.name.to_string(),
            enabled: word.enabled.clone().unwrap_or_default(),
            evm_enabled: word.evm_enabled.clone().unwrap_or_default(),
        })
        .collect()
}

#[derive(Serialize)]
pub struct BuiltInContextModel {
    pub name: String,
    pub scopes: Vec<BuiltInScopeModel>,
}

#[derive(Serialize)]
pub struct BuiltInScopeModel {
    pub name: String,
    pub definitions: Vec<BuiltInDefinitionModel>,
}

#[derive(Serialize)]
pub struct BuiltInDefinitionModel {
    pub name: String,
    pub enabled: VersionSpecifier,
    pub evm_enabled: EvmTargetSpecifier,
    pub internal_parameter: Option<String>,
}

pub fn build_built_ins_model(language: &Language) -> Vec<BuiltInContextModel> {
    language
        .built_ins
        .iter()
        .map(|context| BuiltInContextModel {
            name: context.name.to_string(),
            scopes: context
                .scopes
                .iter()
                .map(|scope| BuiltInScopeModel {
                    name: scope.name.to_string(),
                    definitions: scope
                        .definitions
                        .iter()
                        .map(|def| BuiltInDefinitionModel {
                            name: def.name.to_string(),
                            enabled: def.enabled.clone().unwrap_or_default(),
                            evm_enabled: def.evm_enabled.clone().unwrap_or_default(),
                            internal_parameter: def
                                .internal_parameter
                                .as_ref()
                                .map(|x| x.value.clone()),
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect()
}
