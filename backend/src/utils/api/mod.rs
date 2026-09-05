pub mod error;
pub use error::{ApiError, ApiErrorBody};

pub mod params;

pub mod health;
pub use health::health;

pub mod pot_odds_and_equity_api;
pub use pot_odds_and_equity_api::{generate_pot_equity_problem, check_pot_equity_answer};
pub mod pure_equity_api;
pub use pure_equity_api::{generate_pure_equity_problem, check_pure_equity_answer};
pub mod whats_the_nuts_api;
pub use whats_the_nuts_api::{generate_nuts_problem, check_nuts_answer};
pub mod king_of_the_hill_api;
pub use king_of_the_hill_api::{generate_king_of_the_hill_problem, check_king_of_the_hill_answer};
