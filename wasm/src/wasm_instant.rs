// lifted from crossy_multi
// in turn lifted from
// https://github.com/rust-lang/rust/issues/48564#issuecomment-698712971

// std::time::Instant is not supported in wasm
// Use js performance.now() instead

use wasm_bindgen::prelude::*;

use std::convert::TryInto;
use std::time::Duration;
use std::ops;

#[wasm_bindgen(inline_js = r#"
export function performance_now() {
  return performance.now();
}"#)]
extern "C" {
    fn performance_now() -> f64;
}

// performance.now returns the number of ms since pageload.
// But we want to be able to represent times from before the start of pageload
// So store the number of microseconds as an i128.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmInstant(i128);

const MS_TO_US : f64 = 1000.0;

impl WasmInstant {
    pub fn now() -> Self
    {
        Self((performance_now() * MS_TO_US) as i128)
    }

    pub fn duration_since(&self, earlier: WasmInstant) -> Duration
    { 
        Duration::from_micros((self.0 - earlier.0) as u64)
    }

    pub fn elapsed(&self) -> Duration
    {
        Self::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_add(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_sub(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }

    pub fn saturating_duration_since(&self, earlier: WasmInstant) -> Duration { 
        Duration::from_micros(self.0.saturating_sub(earlier.0) as u64)
    }
}

impl ops::Add<Duration> for WasmInstant { type Output = WasmInstant; fn add(self, other: Duration) -> WasmInstant { self.checked_add(other).unwrap() } }
impl ops::Sub<Duration> for WasmInstant { type Output = WasmInstant; fn sub(self, other: Duration) -> WasmInstant { self.checked_sub(other).unwrap() } }
impl ops::Sub<WasmInstant>  for WasmInstant { type Output = Duration; fn sub(self, other: WasmInstant) -> Duration { self.duration_since(other) } }
impl ops::AddAssign<Duration> for WasmInstant { fn add_assign(&mut self, other: Duration) { *self = *self + other; } }
impl ops::SubAssign<Duration> for WasmInstant { fn sub_assign(&mut self, other: Duration) { *self = *self - other; } }