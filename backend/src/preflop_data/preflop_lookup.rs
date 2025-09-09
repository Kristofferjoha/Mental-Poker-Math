use serde::Deserialize;

/// Represents the equity of hand1 against hand2 in a preflop scenario from preflop_equity.json.

#[derive(Deserialize, Debug, Clone)]
pub struct PreflopEquity {
    pub hand1: String,
    pub hand2: String,
    pub equity: f32,
}