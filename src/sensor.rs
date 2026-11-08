#[derive(Debug, Clone, Copy)]
pub struct Reading {
    pub tick: u32,
    pub heart_rate: f64,
    pub temp_c: f64,
}

pub fn simulate(tick: u32) -> Reading {
    let t = tick as f64;
    let mut heart_rate = 75.0 + 4.0 * (t * 0.3).sin();
    let mut temp_c = 36.8 + 0.1 * (t * 0.1).sin();

    if tick % 25 == 0 && tick != 0 {
        heart_rate += 55.0;
        temp_c += 1.5;
    }

    Reading {
        tick,
        heart_rate,
        temp_c,
    }
}
