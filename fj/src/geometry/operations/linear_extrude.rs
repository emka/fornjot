use nalgebra::{Point, Vector};

use crate::geometry::{
    aabb::Aabb,
    attributes::{BoundingVolume, Surface, SurfaceSample},
};

pub struct LinearExtrude<Sketch> {
    pub sketch: Sketch,
    pub height: f32,
}

impl<Sketch> LinearExtrude<Sketch> {
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<Sketch> BoundingVolume<3> for LinearExtrude<Sketch>
where
    Sketch: BoundingVolume<2>,
{
    fn aabb(&self) -> Aabb<3> {
        self.sketch
            .aabb()
            .extend(-self.height / 2.0, self.height / 2.0)
    }
}

impl<Sketch> Surface<3> for LinearExtrude<Sketch>
where
    Sketch: Surface<2>,
{
    fn sample(&self, point: impl Into<Point<f32, 3>>) -> SurfaceSample<3> {
        let point = point.into();

        let sample_xy = self.sketch.sample(point.xy());

        let d_xy = sample_xy.distance;
        let d_z = point.z.abs() - self.height / 2.0;

        let w = Vector::from([f32::max(d_xy, 0.0), f32::max(d_z, 0.0)]);

        let distance = f32::min(f32::max(d_xy, d_z), 0.0) + w.magnitude();

        SurfaceSample { point, distance }
    }
}

// `LinearExtrude` is covered by a bunch of unit tests in `cylinder`.
