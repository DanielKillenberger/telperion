//! The device's outputs held to the CPU's, field by field: each field's
//! rows, those that differ (or stand past the ruled tolerance,
//! `ruled.rs`), and the first that does.

/// Each field compared: its rows, those that differ, and the first.
#[derive(Default)]
pub(super) struct Agreement {
    pub fields: Vec<(String, usize, usize, String)>,
}

impl Agreement {
    pub(super) fn eq<T: PartialEq + std::fmt::Debug>(&mut self, name: &str, dev: &[T], cpu: &[T]) {
        let differ = dev.iter().zip(cpu).filter(|(a, b)| a != b).count();
        let length = dev.len() != cpu.len();
        let rows: Vec<String> = (dev.iter().zip(cpu).enumerate())
            .filter(|(_, (a, b))| a != b)
            .take(4)
            .map(|(k, (a, b))| format!("row {k}: device {a:?}, cpu {b:?}"))
            .collect();
        let mut first = rows.join("; ");
        if length {
            first += &format!(" (device {} rows, cpu {})", dev.len(), cpu.len());
        }
        self.fields.push((
            name.to_string(),
            cpu.len(),
            differ + usize::from(length),
            first,
        ));
    }

    /// f32 bits held to the ruled standard (`ruled.rs`): a row differs
    /// where it is further than `tolerance` of the field's largest
    /// magnitude from the CPU's.
    pub(super) fn within(&mut self, name: &str, dev: &[u32], cpu: &[u32], tolerance: f64) {
        let f = |b: &u32| f64::from(f32::from_bits(*b));
        self.close(name, &dev.iter().map(f).collect::<Vec<_>>(), &cpu.iter().map(f).collect::<Vec<_>>(), tolerance);
    }

    fn close(&mut self, name: &str, dev: &[f64], cpu: &[f64], tolerance: f64) {
        let scale = cpu.iter().map(|b| b.abs()).fold(f64::MIN_POSITIVE, f64::max);
        let far: Vec<u32> = dev.iter().zip(cpu).map(|(a, b)| u32::from((a - b).abs() > tolerance * scale || a.is_nan() != b.is_nan())).collect();
        self.eq(name, &far, &vec![0; far.len()]);
        if dev.len() != cpu.len() {
            self.eq(&format!("{name} rows"), &[dev.len()], &[cpu.len()]);
        }
    }

    /// Records `width` words wide, their words at `reals` f64 bits within
    /// the ruled tolerance of each one's column, the rest exact.
    pub(super) fn records(&mut self, name: &str, dev: &[u64], cpu: &[u64], (width, reals): (usize, &[usize])) {
        for k in 0..width {
            let (d, c): (Vec<u64>, Vec<u64>) = (dev.iter().skip(k).step_by(width).copied().collect(), cpu.iter().skip(k).step_by(width).copied().collect());
            let field = format!("{name} {k}");
            if reals.contains(&k) {
                let f = |v: &[u64]| v.iter().map(|&b| f64::from_bits(b)).collect::<Vec<_>>();
                self.close(&field, &f(&d), &f(&c), super::ruled::FLOAT_TOLERANCE);
            } else {
                self.eq(&field, &d, &c);
            }
        }
    }

    pub fn exact(&self) -> bool {
        self.fields.iter().all(|f| f.2 == 0)
    }

    pub fn report(&self) -> String {
        let mut out = String::new();
        for (name, rows, differ, first) in &self.fields {
            out += &format!("{name:>24} {rows:>10} rows {differ:>8} differ {first}\n");
        }
        out
    }
}
