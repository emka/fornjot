use std::ops::Deref;

use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            sweep::Sweep,
            update::UpdateSketch,
        },
        services::Services,
        storage::Handle,
    },
    math::Vector,
};

pub fn model(x: f64, y: f64, z: f64, services: &mut Services) -> Handle<Solid> {
    let sketch = Sketch::empty()
        .add_region(
            Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                services,
            )
            .insert(services),
        )
        .insert(services);

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., z]);
    (sketch.deref(), surface).sweep(path, services)
}
