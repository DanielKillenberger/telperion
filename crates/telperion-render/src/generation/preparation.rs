use super::*;

impl Generator {
    pub async fn prepare_async(&self, family: &Family, delivery: Delivery) -> Result<Prepared> {
        if delivery == Delivery::Resident && !self.resident_allowed {
            return Err(telperion_core::Error::InvalidInput(
                "standalone generation requires CPU delivery",
            )
            .into());
        }
        let total = Clock::now();
        let mut metrics = Metrics::default();
        let started = Clock::now();
        let grown = executor::grow(family)?;
        metrics.skeleton_ms = started.elapsed_ms();
        let started = Clock::now();
        let x = grown.expansion()?;
        let mut uploaded_wood = None;
        let mut shared_stations = None;
        let mut wood_attempted = false;
        let mut early_wood_ms = 0.0;
        let mut station_unsupported = false;
        if delivery == Delivery::Resident && x.supports_stations() && x.round_section() {
            let begin = Clock::now();
            let needs_contacts = x.seats();
            let mut candidate_stations = None;
            let mut station_ms = 0.0;
            let mut station_bytes = 0;
            let compact = if needs_contacts {
                x.compact_with_contacts().and_then(|shared| {
                    metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                    if shared.surface().qualified() {
                        let station_start = Clock::now();
                        candidate_stations = x
                            .compact_stations(&shared)?
                            .map(|p| (p.segments, p.count, p.ring_size));
                        station_ms = station_start.elapsed_ms();
                        station_bytes = candidate_stations.as_ref().map_or(0, |(s, _, _)| {
                            (s.capacity() * size_of::<foliage::prepared::StationSegment>()) as u64
                        });
                    }
                    metrics.shared_prepare_cpu_bytes = positions::cpu_bytes(shared.surface())
                        + metrics.shared_contact_cpu_bytes
                        + station_bytes;
                    Ok(shared.into_surface())
                })
            } else {
                x.compact()
            };
            metrics.position_prepare_ms = begin.elapsed_ms() - station_ms;
            let compact = match compact {
                Ok(p) => Some(p),
                Err(telperion_core::Error::InvalidInput("compact surface float32 overflow")) => {
                    metrics.position_fallback = Some("compact float32 range");
                    None
                }
                Err(error) => return Err(error.into()),
            };
            if let Some(p) = compact {
                metrics.shared_prepare_cpu_bytes = metrics
                    .shared_prepare_cpu_bytes
                    .max(positions::cpu_bytes(&p) + station_bytes);
                if !needs_contacts {
                    let scopes = io::scope(&self.gpu);
                    let pending = self.begin_positions(p, &mut metrics);
                    let station_start = Clock::now();
                    let station_result = x.stations();
                    #[cfg(test)]
                    let station_result = super::tests::late_station_result(
                        station_result,
                        pending.as_ref().ok().is_some_and(|p| p.is_some()),
                    );
                    station_ms = station_start.elapsed_ms();
                    let result = match pending {
                        Ok(Some(pending)) => self.complete_positions(pending, &mut metrics).await,
                        Ok(None) => Ok(None),
                        Err(error) => {
                            let _ = io::complete(&self.gpu).await;
                            Err(error)
                        }
                    };
                    let errors = io::errors(&self.gpu, scopes).await;
                    let prepared = station_result?;
                    errors?;
                    let candidate = result?;
                    candidate_stations = prepared.map(|p| (p.segments, p.count, p.ring_size));
                    station_unsupported = candidate_stations.is_none();
                    station_bytes = candidate_stations.as_ref().map_or(0, |(s, _, _)| {
                        (s.capacity() * size_of::<foliage::prepared::StationSegment>()) as u64
                    });
                    metrics.shared_prepare_cpu_bytes = metrics
                        .shared_prepare_cpu_bytes
                        .max(metrics.position_cpu_bytes + station_bytes);
                    if station_unsupported {
                        drop(candidate);
                        metrics.gpu_positions = false;
                        metrics.position_retained_metadata_bytes = 0;
                        metrics.position_fallback = Some("station capability");
                    } else {
                        uploaded_wood = candidate;
                    }
                } else if candidate_stations.is_some() || !p.qualified() {
                    let scopes = io::scope(&self.gpu);
                    let result = self.emit_positions(p, &mut metrics).await;
                    let errors = io::errors(&self.gpu, scopes).await;
                    uploaded_wood = result?;
                    errors?;
                    metrics.shared_prepare_cpu_bytes = metrics
                        .shared_prepare_cpu_bytes
                        .max(metrics.position_cpu_bytes + station_bytes);
                } else {
                    metrics.position_fallback = Some("station capability");
                }
                if let Some(wood) = &uploaded_wood {
                    shared_stations = candidate_stations;
                    metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                    wood_attempted = true;
                }
            }
            early_wood_ms = begin.elapsed_ms() - station_ms;
        } else if delivery == Delivery::Resident {
            metrics.position_fallback = Some("profile or station capability");
        }
        if uploaded_wood.is_none()
            && delivery == Delivery::Resident
            && x.seats()
            && x.supports_stations()
        {
            let prepare_start = Clock::now();
            let shared = x.prepared_with_contacts()?;
            metrics.wood_prepare_ms = prepare_start.elapsed_ms();
            early_wood_ms += metrics.wood_prepare_ms;
            if let Some(shared) = shared {
                metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                if let Some(p) = x.shared_stations(&shared)? {
                    let station_cpu_bytes = (p.segments.capacity()
                        * size_of::<foliage::prepared::StationSegment>())
                        as u64;
                    metrics.shared_prepare_cpu_bytes = metrics.shared_prepare_cpu_bytes.max(
                        wood::prepared_cpu_bytes(shared.surface())
                            + metrics.shared_contact_cpu_bytes
                            + station_cpu_bytes,
                    );
                    let station_data = (p.segments, p.count, p.ring_size);
                    let upload_start = Clock::now();
                    let scopes = io::scope(&self.gpu);
                    let result = self.upload_wood(shared.into_surface(), &mut metrics);
                    let errors = io::errors(&self.gpu, scopes).await;
                    uploaded_wood = result?;
                    errors?;
                    early_wood_ms += upload_start.elapsed_ms();
                    metrics.shared_prepare_cpu_bytes = metrics.shared_prepare_cpu_bytes.max(
                        metrics.wood_prepared_cpu_bytes
                            + metrics.wood_metadata_cpu_bytes
                            + station_cpu_bytes,
                    );
                    wood_attempted = true;
                    if let Some(wood) = &uploaded_wood {
                        metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                        shared_stations = Some(station_data);
                    }
                }
            } else {
                wood_attempted = true;
                metrics.wood_fallback = Some("CPU triangle admission");
            }
        }
        let stations = if shared_stations.is_some() || station_unsupported {
            None
        } else {
            x.stations()?
        };
        if stations.is_none() && shared_stations.is_none() {
            drop(uploaded_wood);
            metrics.gpu_positions = false;
            metrics.position_retained_metadata_bytes = 0;
            let mesh = x.mesh()?;
            metrics.wood_backend = Some(Backend::CpuFallback);
            metrics.wood_fallback = Some("CPU foliage preparation fallback");
            metrics.wood_cpu_bytes = wood::cpu_bytes(&mesh.wood);
            metrics.instances = mesh.foliage_instances() as u32;
            metrics.total_ms = total.elapsed_ms();
            return Ok(Prepared {
                identity: self.identity.clone(),
                mesh,
                resident: None,
                wood: None,
                backend: Backend::CpuFallback,
                metrics,
            });
        };
        let element = x.element();
        metrics.base_cpu_bytes =
            (x.tree().nodes.capacity() * size_of::<telperion_core::tree::Node>()
                + element.positions.capacity() * size_of::<telperion_core::math::Vec3>()
                + (element.indices.capacity()
                    + element.level_indices.capacity()
                    + element.coords.capacity())
                    * 4
                + element.levels.capacity() * size_of::<foliage::Level>()) as u64;
        metrics.descriptors_ms = started.elapsed_ms() - early_wood_ms;
        let scopes = io::scope(&self.gpu);
        let shared_positions = shared_stations
            .as_ref()
            .is_some_and(|(_, count, _)| *count > 0);
        let computed = if let Some((segments, count, ring_size)) = shared_stations {
            if count == 0 {
                self.empty()
            } else {
                let p = foliage::prepared::PreparedStations {
                    segments,
                    count,
                    ring_size,
                    rings: std::borrow::Cow::Borrowed(&uploaded_wood.as_ref().unwrap().positions),
                };
                let (leaves, twig) = (x.leaves(), x.twig());
                self.compute_buffer_async(p, &leaves, twig, element, x.reference(), &mut metrics)
                    .await
            }
        } else {
            let stations = stations.unwrap();
            if stations.count == 0 {
                self.empty()
            } else {
                let (leaves, twig) = (x.leaves(), x.twig());
                self.compute_async(
                    stations,
                    &leaves,
                    twig,
                    element,
                    x.reference(),
                    &mut metrics,
                )
                .await
            }
        };
        let errors = io::errors(&self.gpu, scopes).await;
        let mut resident = Some(computed?);
        errors?;
        if let Some(wood) = &uploaded_wood {
            metrics.gpu_compute_peak_bytes += wood.gpu_metadata_bytes()
                + if shared_positions {
                    0
                } else {
                    wood.positions.size()
                };
        }
        let full = resident.as_ref().unwrap().full;
        metrics.retained_gpu_bytes = resident.as_ref().map_or(0, |r| {
            r.leaves.region().capacity() + r.masses.region().capacity()
        });
        let mut instances = Instances::new(x.reference());
        if delivery == Delivery::Cpu {
            let start = Clock::now();
            let output = resident.take().unwrap();
            instances.leaves =
                io::read_leaves_async(&self.gpu, output.leaves.buffer(), output.count).await?;
            drop(output);
            instances.validate()?;
            metrics.readback_ms = start.elapsed_ms();
        }
        let started = Clock::now();
        let mut resident_wood = None;
        if let Some(uploaded) = uploaded_wood {
            let scopes = io::scope(&self.gpu);
            let result = self.expand_uploaded_wood(uploaded, &mut metrics).await;
            let errors = io::errors(&self.gpu, scopes).await;
            resident_wood = result?;
            errors?;
        } else if delivery == Delivery::Resident && !wood_attempted {
            let compact = x.prepared_wood()?;
            metrics.wood_prepare_ms = started.elapsed_ms();
            if let Some(compact) = compact {
                let scopes = io::scope(&self.gpu);
                let result = self.expand_wood(compact, &mut metrics).await;
                let errors = io::errors(&self.gpu, scopes).await;
                resident_wood = result?;
                errors?;
            } else {
                metrics.wood_fallback = Some("CPU triangle admission");
            }
        }
        metrics.wood_backend = Some(if delivery == Delivery::Cpu {
            Backend::Cpu
        } else if resident_wood.is_some() {
            Backend::Gpu
        } else {
            Backend::CpuFallback
        });
        let wood = if resident_wood.is_some() {
            surface::SurfaceMesh::default()
        } else {
            x.wood()?
        };
        metrics.wood_ms = started.elapsed_ms() + early_wood_ms;
        metrics.wood_cpu_bytes = wood::cpu_bytes(&wood);
        if let Some(w) = &resident_wood {
            metrics.retained_gpu_bytes += w.bytes();
        }
        let bounds = union(
            resident_wood.as_ref().map_or(wood.bounds, |w| w.bounds),
            full,
        )
        .ok_or(telperion_core::Error::InvalidInput("mesh has no geometry"))?;
        let mesh = TreeMesh {
            wood,
            foliage: mesh::Foliage {
                element: x.into_element(),
                instances,
            },
            bounds,
        };
        crate::submit::fits_wood_counts(
            &self.gpu.device.limits(),
            &mesh,
            metrics.instances as usize,
            resident_wood
                .as_ref()
                .map(|w| (w.vertices, w.index_count, w.runs.as_slice())),
        )?;
        metrics.total_ms = total.elapsed_ms();
        Ok(Prepared {
            identity: self.identity.clone(),
            mesh,
            resident,
            wood: resident_wood,
            backend: Backend::Gpu,
            metrics,
        })
    }
}
