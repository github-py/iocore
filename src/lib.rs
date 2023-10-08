//! IO Core
//!
//! ```perl
//!  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠛⠛⠛⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿               ___ ___   ___
//!  ⣿⣿⣿⣿⣿⣿⡿⠟⣋⣡⣴⣶⣾⣿⡇⢸⣶⣄⠀⠀⠈⠙⠻⢿⣿⣿⣿⣿⣿⣿              |_ _/ _ \ / __|___ _ _ ___
//!  ⣿⣿⣿⣿⡿⢋⣴⣾⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣆⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿               | | (_) | (__/ _ \ '_/ -_)
//!  ⣿⣿⣿⠏⣰⣿⣿⣿⣿⣿⣿⡿⠟⠛⠃⠘⢿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿              |___\___/ \___\___/_| \___|
//!  ⣿⣿⠏⣸⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠈⢿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿
//!  ⣿⡏⢠⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿
//!  ⣿⠁⢸⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿
//!  ⣿⠀⢈⣉⣉⣉⣉⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿                      _/_/        _/      _/    _/_/
//!  ⣿⡀⠀⠈⠛⠿⢿⣿⣷⣶⣤⣤⣤⣄⣀⣠⣤⣄⡉⠻⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿       _/      _/  _/    _/    _/_/    _/_/  _/    _/
//!  ⣿⣇⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠛⠛⠛⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿      _/      _/  _/    _/      _/      _/  _/    _/
//!  ⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿       _/  _/    _/    _/      _/      _/  _/    _/
//!  ⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿        _/        _/_/    _/  _/  _/  _/    _/_/
//!  ⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿
//!  ⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿⣿
//!  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣤⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//! ```

pub mod exceptions;
pub mod coreio;
pub mod sr;

pub use exceptions::*;
pub use coreio::*;
