pub mod logger;
pub mod config;

pub mod prelude {
    pub use crate::logging::logger;
    pub use crate::logging::config;
}
