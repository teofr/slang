use slang_solidity_v2_common::nodes::NodeId;

use super::Pass;
use crate::binder::{Definition, ParameterDefinition, Scope, Typing};
use crate::types::{FunctionType, Type, TypeId};

/// Disambiguation functions that require typing (aka overload resolution)
impl Pass<'_> {
    fn get_function_definition_parameters(
        &self,
        definition_id: Option<NodeId>,
    ) -> Option<&[ParameterDefinition]> {
        let Some(Definition::Function(function_definition)) =
            self.binder.find_definition_by_id(definition_id?)
        else {
            return None;
        };

        let Scope::Parameters(parameters_scope) = self
            .binder
            .get_scope_by_id(function_definition.parameters_scope_id)
        else {
            unreachable!("incorrect scope kind, expected parameters");
        };
        Some(&parameters_scope.parameters)
    }

    pub(super) fn lookup_function_matching_positional_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[Typing],
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let Some(parameters) =
                self.get_function_definition_parameters(function_type.definition_id)
            else {
                return false;
            };
            // If the first argument was already bound to a receiver (eg. `a.foo`
            // where `foo` is attached to the type of `a` via a `using`
            // directive), then it has been matched when the binding was created,
            // so only the remaining parameters are matched here.
            let Some(parameters) = self.parameters_after_receiver(function_type, parameters) else {
                return false;
            };
            parameters.len() == argument_typings.len()
                && self.parameters_match_positional_arguments(
                    parameters,
                    argument_typings,
                    function_type.is_externally_visible(),
                )
        })
    }

    /// Drops the bound first parameter (the receiver) from `parameters` when
    /// `function_type` has a receiver bound, returning `None` if the function is
    /// somehow bound but has no parameters. Returns the parameters unchanged for
    /// functions that are not bound to a receiver.
    fn parameters_after_receiver<'a>(
        &self,
        function_type: &FunctionType,
        parameters: &'a [ParameterDefinition],
    ) -> Option<&'a [ParameterDefinition]> {
        if function_type.partial_application.receiver_type().is_some() {
            parameters.split_first().map(|(_receiver, rest)| rest)
        } else {
            Some(parameters)
        }
    }

    fn parameters_match_positional_arguments(
        &self,
        parameters: &[ParameterDefinition],
        argument_typings: &[Typing],
        external_call: bool,
    ) -> bool {
        parameters
            .iter()
            .zip(argument_typings)
            .all(|(parameter, argument_typing)| {
                parameter.type_id.is_some_and(|type_id| {
                    self.parameter_type_matches_argument_typing(
                        type_id,
                        argument_typing,
                        external_call,
                    )
                })
            })
    }

    fn parameter_type_matches_argument_typing(
        &self,
        parameter_type: TypeId,
        argument_typing: &Typing,
        external_call: bool,
    ) -> bool {
        match argument_typing {
            Typing::Resolved(type_id) => {
                if external_call {
                    self.types
                        .implicitly_convertible_to_for_external_call(*type_id, parameter_type)
                } else {
                    self.types
                        .implicitly_convertible_to(*type_id, parameter_type)
                }
            }
            Typing::This(type_id) => self
                .types
                .implicitly_convertible_to(*type_id, parameter_type),
            _ => false,
        }
    }

    pub(super) fn lookup_function_matching_named_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[(String, Typing)],
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let Some(parameters) =
                self.get_function_definition_parameters(function_type.definition_id)
            else {
                return false;
            };
            // If the first argument was bound to a receiver (eg. `a.foo` via a
            // `using` directive), it has already been matched, so only the
            // remaining parameters take part in named-argument matching.
            let Some(parameters) = self.parameters_after_receiver(function_type, parameters) else {
                return false;
            };

            if parameters.iter().any(|parameter| parameter.name.is_none()) {
                // function has an unnamed parameter and we cannot use it for
                // matching named arguments
                return false;
            }

            parameters.len() == argument_typings.len()
                && self.parameters_match_named_arguments(
                    parameters,
                    argument_typings,
                    function_type.is_externally_visible(),
                )
        })
    }

    fn parameters_match_named_arguments(
        &self,
        parameters: &[ParameterDefinition],
        argument_typings: &[(String, Typing)],
        external_call: bool,
    ) -> bool {
        argument_typings
            .iter()
            .all(|(argument_name, argument_typing)| {
                let Some(parameter) = parameters.iter().find(|parameter| {
                    parameter
                        .name
                        .as_ref()
                        .is_some_and(|name| name == argument_name)
                }) else {
                    return false;
                };
                parameter.type_id.is_some_and(|type_id| {
                    self.parameter_type_matches_argument_typing(
                        type_id,
                        argument_typing,
                        external_call,
                    )
                })
            })
    }

    fn get_event_definition_parameters(
        &self,
        definition_id: NodeId,
    ) -> Option<&[ParameterDefinition]> {
        let Some(Definition::Event(event_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            return None;
        };

        let Scope::Parameters(parameters_scope) = self
            .binder
            .get_scope_by_id(event_definition.parameters_scope_id)
        else {
            unreachable!("incorrect scope kind, expected parameters");
        };
        Some(&parameters_scope.parameters)
    }

    pub(super) fn lookup_event_matching_positional_arguments(
        &self,
        definition_ids: &[NodeId],
        argument_typings: &[Typing],
    ) -> Option<NodeId> {
        for definition_id in definition_ids {
            let Some(parameters) = self.get_event_definition_parameters(*definition_id) else {
                continue;
            };
            if parameters.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.parameters_match_positional_arguments(parameters, argument_typings, false) {
                    return Some(*definition_id);
                }
            }
        }
        None
    }

    pub(super) fn lookup_event_matching_named_arguments(
        &self,
        definition_ids: &[NodeId],
        argument_typings: &[(String, Typing)],
    ) -> Option<NodeId> {
        for definition_id in definition_ids {
            let Some(parameters) = self.get_event_definition_parameters(*definition_id) else {
                continue;
            };

            if parameters.iter().any(|parameter| parameter.name.is_none()) {
                // cannot match if any parameter is unnamed
                continue;
            }

            if parameters.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.parameters_match_named_arguments(parameters, argument_typings, false) {
                    return Some(*definition_id);
                }
            }
        }
        None
    }
}
