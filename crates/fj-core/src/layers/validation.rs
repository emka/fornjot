use crate::{
    objects::{AnyObject, Stored},
    validate::{Validation, ValidationError},
};

use super::State;

impl State for Validation {
    type Command = ValidationCommand;
    type Event = ValidationEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let mut errors = Vec::new();

        match command {
            ValidationCommand::ValidateObject { object } => {
                object.validate_with_config(&self.config, &mut errors);

                for err in errors {
                    events.push(ValidationEvent::ValidationFailed {
                        object: object.clone(),
                        err,
                    });
                }
            }
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        match event {
            ValidationEvent::ValidationFailed { object, err } => {
                self.errors.insert(object.id(), err.clone());
            }
        }
    }
}

/// Command for `Layer<Validation>`
pub enum ValidationCommand {
    /// Validate the provided object
    ValidateObject {
        /// The object to validate
        object: AnyObject<Stored>,
    },
}

/// Event produced by `Layer<Validation>`
#[derive(Clone)]
pub enum ValidationEvent {
    /// Validation of an object failed
    ValidationFailed {
        /// The object for which validation failed
        object: AnyObject<Stored>,

        /// The validation error
        err: ValidationError,
    },
}
