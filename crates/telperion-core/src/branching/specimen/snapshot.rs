//! Schema 2: little-endian bincode with fixed-width integers, including usize
//! as u64. Only the chronicle and writer frontiers persist; derived reads,
//! contacts, meshes and instrumentation never do. Schema changes are explicit:
//! schema 2 adds each node's stem flag, so a schema-1 snapshot is refused.
use super::*;
use bincode::Options;

const HEADER: &[u8; 8] = b"TLPS\x02\0\0\0";
fn codec() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_little_endian()
        .with_limit(512 * 1024 * 1024)
        .reject_trailing_bytes()
}
pub(super) fn empty_bias() -> GrowthBias {
    GrowthBias::new(Envelope::default(), 0, Default::default()).unwrap()
}
impl Specimen {
    pub fn snapshot(&self) -> Result<Vec<u8>> {
        if self.timeline.is_none() {
            return Err(Error::InvalidInput("specimen snapshot frontier missing"));
        }
        let mut bytes = HEADER.to_vec();
        codec()
            .serialize_into(&mut bytes, self)
            .map_err(|_| Error::ResourceLimit("specimen snapshot encoding"))?;
        bytes.extend(checksum(&bytes).to_le_bytes());
        Ok(bytes)
    }
    pub fn from_snapshot(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < HEADER.len() + 8 || bytes.len() > 512 * 1024 * 1024 {
            return Err(Error::InvalidInput("specimen snapshot size"));
        }
        let (bytes, digest) = bytes.split_at(bytes.len() - 8);
        if checksum(bytes).to_le_bytes() != digest {
            return Err(Error::InvalidInput("specimen snapshot checksum"));
        }
        let payload = bytes
            .strip_prefix(HEADER)
            .ok_or(Error::InvalidInput("specimen snapshot schema"))?;
        let mut specimen: Self =
            codec()
                .deserialize(payload)
                .map_err(|err| Error::InvalidValue {
                    field: "specimen snapshot payload",
                    value: err.to_string(),
                })?;
        let t = specimen
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen snapshot frontier"))?;
        crate::growth::Age::from_years(t.age.years())?;
        retention::checked_cap(specimen.history_cap())?;
        specimen.params.envelope.validate()?;
        specimen.params.habit.validate()?;
        specimen.radii.resolved()?;
        specimen.bias = GrowthBias::new(
            specimen.params.envelope,
            specimen.params.seed,
            specimen.params.bias,
        )?;
        Ok(specimen)
    }
}

// FNV-1a detects accidental corruption, including valid-but-altered floats.
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}
