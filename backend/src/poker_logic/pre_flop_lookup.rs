use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PreflopEquity {
    pub hand1: String,
    pub hand2: String,
    pub equity: f32,
}
