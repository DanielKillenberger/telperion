//! Specimen C ABI. A rebuild is transactional; output staging is owned by the
//! binding until its next read. Snapshot staging has a separate explicit release.
use super::*;

fn request(e: &Engine) -> Result<Value> {
    serde_json::from_slice(&e.input).map_err(|_| Error::InvalidInput("specimen request JSON"))
}
#[no_mangle]
pub extern "C" fn specimen_build(cap: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = request(&e)
            .and_then(|v| params::parse(&v))
            .and_then(|family| e.specimen.build(&family, cap))
            .map(|handle| json!({"handle":handle}));
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_read(handle: u32, age: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e.specimen.read(handle, Some(age)).and_then(|read| {
            let mut output = Output::default();
            for n in &read.tree.nodes {
                output.structure.extend([
                    n.position.x,
                    n.position.y,
                    n.position.z,
                    n.radius,
                    n.start_radius,
                    n.base_radius,
                ]);
                output
                    .topology
                    .extend([n.parent.unwrap_or(u32::MAX), n.branch, n.kind as u32]);
            }
            output.instances.matrices = read.placements.iter().map(|p| p.transform).collect();
            e.specimen_ids = bincode::serialize(&(
                read.tree
                    .nodes
                    .iter()
                    .map(|n| n.identity)
                    .collect::<Vec<_>>(),
                read.placements
                    .iter()
                    .map(|p| p.identity)
                    .collect::<Vec<_>>(),
                read.shed,
            ))
            .map_err(|_| Error::ResourceLimit("specimen read identities"))?;
            e.output = output;
            e.revision = e.revision.wrapping_add(1);
            Ok(
                json!({"age":age, "envelope":{
                    "height":read.envelope.height,"spread":read.envelope.spread,
                    "crownBase":read.envelope.crown_base,"fullness":read.envelope.fullness,"shoulder":read.envelope.shoulder
                }, "surfaceHeight":read.surface_height,
                "diagnostics":read.tree.diagnostics,"crossover":read.tree.crossover}),
            )
        });
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_frontier(handle: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e
            .specimen
            .specimen(handle)
            .map(|s| json!({"frontier":s.age(),"historyCap":s.history_cap()}));
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_advance(handle: u32, years: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e.specimen.advance(handle, years).and_then(|changes| {
            e.specimen_record = bincode::serialize(&changes)
                .map_err(|_| Error::ResourceLimit("specimen change record"))?;
            Ok(json!({"frontier": e.specimen.specimen(handle).unwrap().age()}))
        });
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_changes(handle: u32, from: f64, to: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e.specimen.changes(handle, from, to).and_then(|changes| {
            e.specimen_record = bincode::serialize(&changes)
                .map_err(|_| Error::ResourceLimit("specimen change record"))?;
            Ok(json!(null))
        });
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_snapshot(handle: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.specimen_bytes.clear();
        let result = e.specimen.snapshot(handle).map(|bytes| {
            e.specimen_bytes = bytes;
            json!({"schema":1})
        });
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_snapshot_release() {
    ENGINE.with(|e| e.borrow_mut().specimen_bytes = Vec::new());
}
#[no_mangle]
pub extern "C" fn specimen_snapshot_alloc(len: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = if len > 512 * 1024 * 1024 {
            Err(Error::ResourceLimit("specimen snapshot size"))
        } else {
            reserve(&mut e.specimen_bytes, len as usize)
        };
        status(&mut e, result.map(|_| json!(null)))
    })
}
#[no_mangle]
pub extern "C" fn specimen_import() -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let bytes = std::mem::take(&mut e.specimen_bytes);
        let result = e
            .specimen
            .import(&bytes)
            .map(|handle| json!({"handle":handle}));
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn specimen_release(handle: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e.specimen.specimen(handle).map(|_| ());
        if result.is_ok() {
            e.specimen.release();
            e.specimen_record = Vec::new();
            e.specimen_ids = Vec::new();
            e.specimen_bytes = Vec::new();
            e.output = Output::default();
            e.revision = e.revision.wrapping_add(1);
        }
        status(&mut e, result.map(|_| json!(null)))
    })
}

#[no_mangle]
pub extern "C" fn specimen_node_ceiling(handle: u32, limit: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = if !limit.is_finite()
            || limit < 0.0
            || limit.fract() != 0.0
            || limit > telperion_core::branching::NODE_CEILING as f64
        {
            Err(Error::InvalidValue {
                field: "node ceiling",
                value: limit.to_string(),
            })
        } else {
            e.specimen.set_node_ceiling(handle, limit as usize)
        };
        status(&mut e, result.map(|_| json!(null)))
    })
}
#[no_mangle]
pub extern "C" fn specimen_history_cap(handle: u32, years: f64) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let result = e.specimen.set_history_cap(handle, years);
        status(&mut e, result.map(|_| json!(null)))
    })
}
