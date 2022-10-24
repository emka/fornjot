use fj_math::Transform;

use crate::{
    objects::Objects, partial::PartialCycle, validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialCycle {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let surface = self
            .surface
            .clone()
            .map(|surface| surface.transform(transform, objects))
            .transpose()?;
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|edge| {
                Ok(edge
                    .into_partial()
                    .transform(transform, objects)?
                    .with_surface(surface.clone())
                    .into())
            })
            .collect::<Result<_, ValidationError>>()?;

        Ok(Self {
            surface,
            half_edges,
        })
    }
}
