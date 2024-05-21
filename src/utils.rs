#[cfg(feature = "serde")]
pub mod serde_notnan {
  use ordered_float::NotNan;
}
#[cfg(feature = "serde")]
pub use self::serde_notnan::*;
