use fj_interop::ext::ArrayExt;

use crate::{
    builder::GlobalEdgeBuilder,
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, Vertex,
    },
    partial::{util::merge_arrays, MaybePartial},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialHalfEdge {
    curve: MaybePartial<Curve>,
    vertices: [MaybePartial<Vertex>; 2],
    global_form: MaybePartial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Access the curve that the [`HalfEdge`] is defined in
    pub fn curve(&self) -> MaybePartial<Curve> {
        self.curve.clone()
    }

    /// Access the vertices that bound this [`HalfEdge`] in the [`Curve`]
    pub fn vertices(&self) -> [MaybePartial<Vertex>; 2] {
        self.vertices.clone()
    }

    /// Access the global form of the [`HalfEdge`]
    pub fn global_form(&self) -> MaybePartial<GlobalEdge> {
        self.global_form.clone()
    }

    /// Extract the global curve from either the curve or global form
    ///
    /// If a global curve is available through both, the curve is preferred.
    pub fn extract_global_curve(&self) -> MaybePartial<GlobalCurve> {
        self.curve
            .global_form()
            .unwrap_or_else(|| self.global_form.curve())
    }

    /// Update the partial half-edge with the given surface
    pub fn with_surface(mut self, surface: Handle<Surface>) -> Self {
        self.curve = self
            .curve
            .update_partial(|curve| curve.with_surface(Some(surface.clone())));

        self.vertices = self.vertices.map(|vertex| {
            vertex.update_partial(|vertex| {
                let surface_form =
                    vertex.surface_form().update_partial(|surface_vertex| {
                        surface_vertex.with_surface(Some(surface.clone()))
                    });

                vertex.with_surface_form(surface_form)
            })
        });

        self
    }

    /// Update the partial half-edge with the given curve
    pub fn with_curve(
        mut self,
        curve: Option<impl Into<MaybePartial<Curve>>>,
    ) -> Self {
        if let Some(curve) = curve {
            self.curve = curve.into();
        }
        self
    }

    /// Update the partial half-edge with the given back vertex
    pub fn with_back_vertex(
        mut self,
        vertex: impl Into<MaybePartial<Vertex>>,
    ) -> Self {
        let [from, _] = &mut self.vertices;
        *from = vertex.into();

        self
    }

    /// Update the partial half-edge with the given front vertex
    pub fn with_front_vertex(
        mut self,
        vertex: impl Into<MaybePartial<Vertex>>,
    ) -> Self {
        let [_, to] = &mut self.vertices;
        *to = vertex.into();

        self
    }

    /// Update the partial half-edge with the given vertices
    pub fn with_vertices(
        mut self,
        vertices: Option<[impl Into<MaybePartial<Vertex>>; 2]>,
    ) -> Self {
        let vertices = vertices.map(|vertices| vertices.map(Into::into));
        if let Some([back, front]) = vertices {
            self.vertices = [back, front];
        }
        self
    }

    /// Update the partial half-edge with the given global form
    pub fn with_global_form(
        mut self,
        global_form: Option<impl Into<MaybePartial<GlobalEdge>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = global_form.into();
        }
        self
    }

    /// Merge this partial object with another
    pub fn merge_with(self, other: Self) -> Self {
        Self {
            curve: self.curve.merge_with(other.curve),
            vertices: merge_arrays(self.vertices, other.vertices),
            global_form: self.global_form.merge_with(other.global_form),
        }
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<HalfEdge>, ValidationError> {
        let curve = self.curve.into_full(objects)?;
        let vertices = self.vertices.try_map_ext(|vertex| {
            vertex
                .update_partial(|vertex| vertex.with_curve(Some(curve.clone())))
                .into_full(objects)
        })?;

        let global_form = self
            .global_form
            .update_partial(|partial| {
                partial.update_from_curve_and_vertices(&curve, &vertices)
            })
            .into_full(objects)?;

        Ok(objects
            .half_edges
            .insert(HalfEdge::new(vertices, global_form))?)
    }
}

impl From<&HalfEdge> for PartialHalfEdge {
    fn from(half_edge: &HalfEdge) -> Self {
        let [back_vertex, front_vertex] =
            half_edge.vertices().clone().map(Into::into);

        Self {
            curve: half_edge.curve().clone().into(),
            vertices: [back_vertex, front_vertex],
            global_form: half_edge.global_form().clone().into(),
        }
    }
}

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    curve: MaybePartial<GlobalCurve>,
    vertices: Option<[MaybePartial<GlobalVertex>; 2]>,
}

impl PartialGlobalEdge {
    /// Access the curve that the [`GlobalEdge`] is defined in
    pub fn curve(&self) -> MaybePartial<GlobalCurve> {
        self.curve.clone()
    }

    /// Access the vertices that bound the [`GlobalEdge`] in the curve
    pub fn vertices(&self) -> Option<[MaybePartial<GlobalVertex>; 2]> {
        self.vertices.clone()
    }

    /// Update the partial global edge with the given curve
    pub fn with_curve(
        mut self,
        curve: Option<impl Into<MaybePartial<GlobalCurve>>>,
    ) -> Self {
        if let Some(curve) = curve {
            self.curve = curve.into();
        }
        self
    }

    /// Update the partial global edge with the given vertices
    pub fn with_vertices(
        mut self,
        vertices: Option<[impl Into<MaybePartial<GlobalVertex>>; 2]>,
    ) -> Self {
        if let Some(vertices) = vertices {
            self.vertices = Some(vertices.map(Into::into));
        }
        self
    }

    /// Merge this partial object with another
    pub fn merge_with(self, other: Self) -> Self {
        // This is harder than it needs to be, because `vertices` uses the
        // redundant combination of `Option` and `MaybePartial`. There's some
        // code relying on that, however, so we have to live with it for now.
        let vertices = match (self.vertices, other.vertices) {
            (Some(a), Some(b)) => Some(merge_arrays(a, b)),
            (Some(vertices), None) | (None, Some(vertices)) => Some(vertices),
            (None, None) => None,
        };

        Self {
            curve: self.curve.merge_with(other.curve),
            vertices,
        }
    }

    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<GlobalEdge>, ValidationError> {
        let curve = self.curve.into_full(objects)?;
        let vertices = self
            .vertices
            .expect("Can't build `GlobalEdge` without vertices")
            .try_map_ext(|global_vertex| global_vertex.into_full(objects))?;

        Ok(objects
            .global_edges
            .insert(GlobalEdge::new(curve, vertices))?)
    }
}

impl From<&GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: &GlobalEdge) -> Self {
        Self {
            curve: global_edge.curve().clone().into(),
            vertices: Some(
                global_edge
                    .vertices()
                    .access_in_normalized_order()
                    .map(Into::into),
            ),
        }
    }
}
