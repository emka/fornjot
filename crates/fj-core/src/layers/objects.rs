//! Layer infrastructure for [`Objects`]

use crate::{
    objects::{AboutToBeStored, AnyObject, Objects},
    validation::Validation,
};

use super::{Command, Event, Layer};

impl Layer<Objects> {
    /// Insert an object into the stores
    ///
    /// Passes any events produced to the validation layer.
    pub fn insert(
        &mut self,
        object: AnyObject<AboutToBeStored>,
        validation: &mut Layer<Validation>,
    ) {
        let mut events = Vec::new();
        self.process(InsertObject { object }, &mut events);

        for event in events {
            validation.process(event, &mut Vec::new());
        }
    }
}

/// Insert an object into the stores
///
/// This struct serves as both event and command for `Layer<Objects>`, as well
/// as a command for `Layer<Validation>`.
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: AnyObject<AboutToBeStored>,
}

impl Command<Objects> for InsertObject {
    type Result = ();
    type Event = InsertObject;

    fn decide(self, _: &Objects, events: &mut Vec<Self::Event>) {
        events.push(self);
    }
}

impl Event<Objects> for InsertObject {
    fn evolve(&self, state: &mut Objects) {
        self.object.clone().insert(state);
    }
}
