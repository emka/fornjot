use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{Cycle, HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::HalfEdgeBuilder;

/// Builder API for [`Cycle`]
#[derive(Default)]
pub struct CycleBuilder {
    half_edges: Vec<HalfEdgeBuilder>,
}

impl CycleBuilder {
    /// Create an instance of `CycleBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a half-edge to the cycle
    pub fn add_half_edge(mut self, half_edge: HalfEdgeBuilder) -> Self {
        self.half_edges.push(half_edge);
        self
    }

    /// Create a cycle whose half-edges are connected to the provided half-edges
    ///
    /// The half-edges of the new circle will be coincident with the provided
    /// half-edges, but will point in the opposite direction.
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    pub fn connect_to_edges<Es>(edges: Es) -> Self
    where
        Es: IntoIterator<Item = (Handle<HalfEdge>, Curve, [Point<1>; 2])>,
        Es::IntoIter: Clone + ExactSizeIterator,
    {
        let half_edges = edges
            .into_iter()
            .circular_tuple_windows()
            .map(|((prev, _, _), (_, curve, boundary))| {
                HalfEdgeBuilder::new(curve, boundary)
                    .with_start_vertex(prev.start_vertex().clone())
            })
            .collect();

        Self { half_edges }
    }

    /// Create a polygon
    pub fn polygon<P, Ps>(points: Ps) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let half_edges = points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .map(|(start, end)| {
                HalfEdgeBuilder::line_segment([start, end], None)
            })
            .collect();

        Self { half_edges }
    }

    /// Build the cycle
    pub fn build(self, objects: &mut Service<Objects>) -> Cycle {
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|half_edge| half_edge.build(objects).insert(objects));
        Cycle::new(half_edges)
    }
}
