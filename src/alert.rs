use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Alert {
    pub tick: u32,
    pub metric: String,
    pub value: f64,
    pub z_score: f64,
}
