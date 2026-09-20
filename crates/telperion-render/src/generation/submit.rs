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
        crate::submit::fits_wood_counts(
            &self.gpu.device.limits(),
            &prepared.mesh,
            count,
            prepared
                .wood
                .as_ref()
                .map(|w| (w.vertices, w.index_count, w.runs.as_slice())),
        )?;
        let Some(resident) = prepared.resident else {
            return self.submit(&prepared.mesh);
        };
        let mesh = prepared.mesh;
        let (wood_vertices, wood_triangles) = if let Some(wood) = prepared.wood {
            let counts = (wood.vertices, wood.index_count as usize / 3);
            self.wood.submit_resident(
                &self.gpu,
                wood.positions,
                wood.normals,
                wood.coords,
                wood.indices,
                wood.radii,
                wood.index_count,
                wood.runs,
            );
            counts
        } else {
            self.wood.submit(&self.gpu, &mesh.wood);
            (mesh.wood_vertices(), mesh.wood_triangles())
        };
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
        self.set_casters();
        self.level_deviations = mesh
            .foliage
            .element
            .levels
            .iter()
            .map(|l| l.deviation)
            .collect();
        Ok(Submitted {
            wood_vertices,
            wood_triangles,
            foliage_instances: count,
            bounds: mesh.bounds,
        })
    }
}
