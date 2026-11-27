use super::super::{ContractDefinition, Definition, InterfaceDefinition};

pub enum ContractBase<'a> {
    Contract(ContractDefinition<'a>),
    Interface(InterfaceDefinition<'a>),
}

impl<'a> ContractBase<'a> {
    pub(crate) fn from_definition(definition: &Definition<'a>) -> Option<Self> {
        match definition {
            Definition::Contract(contract) => Some(Self::Contract(contract.clone())),
            Definition::Interface(interface) => Some(Self::Interface(interface.clone())),
            _ => None,
        }
    }
}
