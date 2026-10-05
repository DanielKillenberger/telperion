use super::*;
impl Renderer {
    pub fn submit_prepared(&mut self, prepared: Prepared) -> Result<Submitted> {
        if !Arc::ptr_eq(&self.identity, &prepared.identity) {
            return Err(telperion_core::Error::InvalidInput("generation renderer mismatch").into());
        }
        if let Some(error) = self.gpu.lost() {
            return Err(error);
        }
        let count = prepared.count();
        crate::submit::fits_count(&self.gpu.device.limits(), &prepared.mesh, count)?;
        let Some(resident) = prepared.resident else {
            return self.submit(&prepared.mesh);
        };
        let mesh = prepared.mesh;
        self.wood.submit(&self.gpu, &mesh.curve);
        self.foliage.submit_resident(
            &self.gpu,
            &mesh.foliage.element,
            mesh.foliage.instances.reference,
            resident.count,
            resident.leaves,
            resident.masses,
        );
        self.scene
            .place_figure(&self.gpu, mesh.bounds.max.y - mesh.bounds.min.y);
        self.scene.set_crown(resident.crown);
        self.scene
            .set_leaf_reference(mesh.foliage.instances.reference);
        self.scene.section_roundness = mesh.foliage.element.section_roundness;
        self.bounds = Some(mesh.bounds);
        self.level_deviations = mesh
            .foliage
            .element
            .levels
            .iter()
            .map(|l| l.deviation)
            .collect();
        Ok(Submitted {
            wood_vertices: mesh.wood_vertices(),
            wood_triangles: mesh.wood_triangles(),
            foliage_instances: count,
            bounds: mesh.bounds,
        })
    }
}
