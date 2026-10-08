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
        match prepared.wood {
            Some(curve) => self.wood.submit_on_device(curve),
            None => self.wood.submit(&self.gpu, &mesh.curve),
        }
        self.foliage.submit_resident(
            &self.gpu,
            &mesh.foliage.element,
            mesh.foliage.instances.reference,
            resident.count,
            resident.leaves,
            resident.masses,
        );
        match (prepared.organ, mesh.organ.as_ref()) {
            // The florets on the device (fn-229), as the leaves are.
            (Some(organ), Some(o)) => {
                let reference = o.instances.reference;
                let (count, placements) = (organ.count, organ.leaves);
                self.organ.submit_resident(
                    &self.gpu,
                    &o.element,
                    reference,
                    count,
                    placements,
                    organ.masses,
                );
                self.scene.set_organ_reference(Some(reference));
            }
            (_, organ) => self.submit_organ(organ, crate::Level::default()),
        }
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
