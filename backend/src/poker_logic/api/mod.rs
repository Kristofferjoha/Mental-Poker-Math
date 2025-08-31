pub mod odds_and_eq_api;
pub use odds_and_eq_api::{get_new_problem, check_answer};
pub mod pure_eq_api;
pub use pure_eq_api::{get_pure_eq_problem, pure_eq_check_answer};
pub mod pure_pot_odds_api;
pub use pure_pot_odds_api::{get_pure_pot_odds_problem, pure_pot_odds_check_answer};