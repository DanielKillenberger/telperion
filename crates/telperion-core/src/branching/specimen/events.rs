//! Year-indexed identity events. The usual append hits the current year directly;
//! interval lookup binary-searches the year range, never individual nodes.
use super::*;

#[derive(Clone, Default)]
pub(super) struct Events(Vec<(u64, Vec<NodeKey>)>);
impl Events {
    pub fn record(&mut self, year: u64, key: NodeKey) {
        if let Some((_, keys)) = self.0.last_mut().filter(|(last, _)| *last == year) {
            keys.push(key);
            return;
        }
        let at = self.0.partition_point(|(stamp, _)| *stamp < year);
        if self.0.get(at).is_none_or(|(stamp, _)| *stamp != year) {
            self.0.insert(at, (year, Vec::new()));
        }
        self.0[at].1.push(key);
    }
    pub fn forget(&mut self, ids: &DenseSlotMap<NodeKey, usize>) {
        for (_, keys) in &mut self.0 {
            keys.retain(|key| ids[*key] != usize::MAX);
        }
        self.0.retain(|(_, keys)| !keys.is_empty());
    }
    pub fn between(&self, lo: u64, hi: u64) -> impl Iterator<Item = NodeKey> + '_ {
        let first = self.0.partition_point(|(year, _)| *year < lo);
        self.0[first..]
            .iter()
            .take_while(move |(year, _)| *year <= hi)
            .flat_map(|(_, keys)| keys.iter().copied())
    }
}
