//! Retained native handle with transactional replacement. Readers own their
//! outputs; a failed replacement preserves both the specimen and its revision.
#[cfg(feature = "geometry")]
use crate::{
    branching::{ChangeRecord, Specimen, SpecimenRead},
    presets::Family,
    Error, Result,
};

#[cfg(feature = "geometry")]
#[derive(Default)]
pub struct SpecimenStore {
    specimen: Option<Specimen>,
    revision: u32,
}
#[cfg(feature = "geometry")]
impl SpecimenStore {
    pub fn build(&mut self, family: &Family, history_cap: f64) -> Result<u32> {
        let specimen = Specimen::build_with_history_cap(family, history_cap)?;
        Ok(self.replace(specimen))
    }
    fn replace(&mut self, specimen: Specimen) -> u32 {
        self.release();
        self.specimen = Some(specimen);
        self.revision
    }
    pub fn release(&mut self) {
        self.specimen = None;
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn specimen(&self, handle: u32) -> Result<&Specimen> {
        self.specimen
            .as_ref()
            .filter(|_| handle == self.revision)
            .ok_or(Error::InvalidInput("invalid specimen handle"))
    }
    pub fn read(&self, handle: u32, age: Option<f64>) -> Result<SpecimenRead> {
        let s = self.specimen(handle)?;
        age.map_or_else(|| s.read(), |age| s.read_at_age(age))
    }
    pub fn advance(&mut self, handle: u32, years: f64) -> Result<ChangeRecord> {
        self.specimen(handle)?;
        self.specimen.as_mut().unwrap().advance(years)
    }
    pub fn set_node_ceiling(&mut self, handle: u32, limit: usize) -> Result<()> {
        self.specimen(handle)?;
        self.specimen.as_mut().unwrap().set_node_ceiling(limit)
    }
    pub fn set_history_cap(&mut self, handle: u32, years: f64) -> Result<()> {
        self.specimen(handle)?;
        self.specimen.as_mut().unwrap().set_history_cap(years)
    }
    pub fn changes(&self, handle: u32, from: f64, to: f64) -> Result<ChangeRecord> {
        self.specimen(handle)?.changes_between(from, to)
    }
    #[cfg(feature = "json")]
    pub fn snapshot(&self, handle: u32) -> Result<Vec<u8>> {
        self.specimen(handle)?.snapshot()
    }
    #[cfg(feature = "json")]
    pub fn import(&mut self, bytes: &[u8]) -> Result<u32> {
        Ok(self.replace(Specimen::from_snapshot(bytes)?))
    }
}

#[cfg(feature = "geometry")]
mod view;
#[cfg(feature = "geometry")]
pub use view::SpecimenView;

#[cfg(feature = "json")]
pub(crate) mod portable;
