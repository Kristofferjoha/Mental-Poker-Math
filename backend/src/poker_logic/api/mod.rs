pub mod logging;
pub mod game1_api;
pub use game1_api::{get_new_problem, check_answer};
pub mod pure_eq_api;
pub use pure_eq_api::{get_pure_eq_problem, pure_eq_check_answer};