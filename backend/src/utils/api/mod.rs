pub mod error;
pub use error::{ApiError, ApiErrorBody};

pub mod pot_odds_and_equity_api;
pub use pot_odds_and_equity_api::{generate_pot_equity_problem, check_pot_equity_answer};
pub mod pure_equity_api;
pub use pure_equity_api::{generate_pure_equity_problem, check_pure_equity_answer};
