//! IO Core
//!
//! > ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠛⠛⠛⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//! > ⣿⣿⣿⣿⣿⣿⡿⠟⣋⣡⣴⣶⣾⣿⡇⢸⣶⣄⠀⠀⠈⠙⠻⢿⣿⣿⣿⣿⣿⣿
//! > ⣿⣿⣿⣿⡿⢋⣴⣾⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣆⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿
//! > ⣿⣿⣿⠏⣰⣿⣿⣿⣿⣿⣿⡿⠟⠛⠃⠘⢿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿
//! > ⣿⣿⠏⣸⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠈⢿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿
//! > ⣿⡏⢠⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿
//! > ⣿⠁⢸⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿
//! > ⣿⠀⢈⣉⣉⣉⣉⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿
//! > ⣿⡀⠀⠈⠛⠿⢿⣿⣷⣶⣤⣤⣤⣄⣀⣠⣤⣄⡉⠻⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿
//! > ⣿⣇⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠛⠛⠛⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿
//! > ⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿
//! > ⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿
//! > ⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿
//! > ⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿⣿
//! > ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣤⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//!
pub mod exceptions;
pub mod coreio;
pub mod sr;

pub use exceptions::*;
pub use coreio::*;