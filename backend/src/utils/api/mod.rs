pub mod odds_and_equity_api;
pub use odds_and_equity_api::{generate_pot_equity_problem, check_pot_equity_answer};
pub mod pure_equity_api;
pub use pure_equity_api::{generate_pure_equity_problem, check_pure_equity_answer};
pub mod pure_pot_odds_api;
pub use pure_pot_odds_api::{generate_pure_pot_odds_problem, check_pure_pot_odds_answer};