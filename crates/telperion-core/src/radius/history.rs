//! A slice records one common pipe scale. Suffix maxima retain wood lost to
//! forks or shedding without touching every living node. A rising scale replaces
//! obsolete maxima; equal samples consume no additional storage.
#[derive(Clone, Default)]
pub(super) struct History {
    count: usize,
    maxima: Vec<(usize, f64)>,
}
impl History {
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn push(&mut self, value: f64) {
        while self
            .maxima
            .last()
            .is_some_and(|&(_, previous)| previous <= value)
        {
            self.maxima.pop();
        }
        self.maxima.push((self.count, value));
        self.count += 1;
    }
    pub fn maximum(&self, from: usize) -> f64 {
        let i = self.maxima.partition_point(|&(slice, _)| slice < from);
        self.maxima.get(i).map_or(0.0, |&(_, value)| value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_maximum_survives_scale_falls_and_later_recovery() {
        let mut history = History::default();
        let mut values = Vec::new();
        for value in [5.0, 2.0, 4.0, 1.0, 1.0, 0.5, 6.0, 3.0] {
            history.push(value);
            values.push(value);
            for from in 0..=values.len() {
                assert_eq!(
                    history.maximum(from),
                    values[from..].iter().copied().fold(0.0, f64::max)
                );
            }
        }
    }
}
