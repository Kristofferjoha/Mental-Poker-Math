use std::collections::HashMap;
use std::str::FromStr;

use crate::calculators::equity_calculator::MAX_PLAYERS;
use crate::problems::problem_helpers::Street;
use crate::utils::api::ApiError;

fn parse<T: FromStr>(
    params: &HashMap<String, String>,
    name: &'static str,
    expected: &str
) -> Result<Option<T>, ApiError> {
    let Some(raw) = params.get(name) else {
        return Ok(None);
    };
    raw.parse()
        .map(Some)
        .map_err(|_| ApiError::InvalidParameter {
            name,
            detail: format!("expected {expected}, got {raw:?}")
        })
}

pub fn num_players(params: &HashMap<String, String>) -> Result<usize, ApiError> {
    let parsed: usize = match parse(params, "numPlayers", "a whole number")? {
        Some(n) => n,
        None => return Ok(2)
    };
    if !(2..=MAX_PLAYERS).contains(&parsed) {
        return Err(ApiError::InvalidParameter {
            name: "numPlayers",
            detail: format!("must be between 2 and {MAX_PLAYERS}, got {parsed}")
        });
    }
    Ok(parsed)
}

pub fn streets(
    params: &HashMap<String, String>,
    offered: &[&str]
) -> Result<Vec<String>, ApiError> {
    let Some(raw) = params.get("streets") else {
        return Ok(offered.iter().map(|s| s.to_string()).collect());
    };

    let mut chosen: Vec<String> = Vec::new();
    for name in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if name.parse::<Street>().is_err() {
            return Err(ApiError::InvalidParameter {
                name: "streets",
                detail: format!("unknown street {name:?}; expected one of {}", offered.join(", "))
            });
        }
        if !offered.contains(&name) {
            return Err(ApiError::InvalidParameter {
                name: "streets",
                detail: format!("this mode does not deal {name:?}; expected one of {}", offered.join(", "))
            });
        }
        if !chosen.iter().any(|s| s == name) {
            chosen.push(name.to_string());
        }
    }

    if chosen.is_empty() {
        return Err(ApiError::InvalidParameter {
            name: "streets",
            detail: "expected at least one street".to_string()
        });
    }
    Ok(chosen)
}

pub fn tolerance(params: &HashMap<String, String>, default: f32) -> Result<f32, ApiError> {
    let parsed: f32 = match parse(params, "tolerance", "a number of percentage points")? {
        Some(t) => t,
        None => return Ok(default)
    };
    if !(0.1..=25.0).contains(&parsed) {
        return Err(ApiError::InvalidParameter {
            name: "tolerance",
            detail: format!("must be between 0.1 and 25, got {parsed}")
        });
    }
    Ok(parsed)
}
