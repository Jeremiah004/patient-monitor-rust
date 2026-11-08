use std::collections::VecDeque;

pub struct Detector {
    window: VecDeque<f64>,
    size: usize,
    threshold: f64,
    min_std: f64,
}

impl Detector {
    pub fn new(size: usize, threshold: f64, min_std: f64) -> Self {
        Self {
            window: VecDeque::with_capacity(size),
            size,
            threshold,
            min_std,
        }
    }

    /// Returns Some(z_score) if the value is an anomaly, otherwise None.
    pub fn check(&mut self, value: f64) -> Option<f64> {
        if self.window.len() < self.size {
            self.window.push_back(value);
            return None;
        }

        let n = self.window.len() as f64;
        let mean = self.window.iter().sum::<f64>() / n;
        let variance = self.window.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let std = variance.sqrt().max(self.min_std);

        let z = (value - mean).abs() / std;
        if z > self.threshold {
            return Some(z); // anomalies stay out of the baseline
        }

        self.window.pop_front();
        self.window.push_back(value);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_alert_during_warmup() {
        let mut d = Detector::new(5, 3.0, 1.0);
        for _ in 0..5 {
            assert!(d.check(100.0).is_none());
        }
    }

    #[test]
    fn steady_values_are_not_flagged() {
        let mut d = Detector::new(5, 3.0, 1.0);
        for _ in 0..20 {
            assert!(d.check(70.0).is_none());
        }
    }

    #[test]
    fn spike_is_flagged() {
        let mut d = Detector::new(5, 3.0, 1.0);
        for _ in 0..5 {
            d.check(70.0);
        }
        assert!(d.check(130.0).is_some());
    }

    #[test]
    fn spike_does_not_corrupt_baseline() {
        let mut d = Detector::new(5, 3.0, 1.0);
        for _ in 0..5 {
            d.check(70.0);
        }
        d.check(130.0);
        assert!(d.check(70.0).is_none());
    }
}
