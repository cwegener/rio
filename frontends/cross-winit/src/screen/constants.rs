#[cfg(not(any(target_os = "macos")))]
pub const PADDING_Y: f32 = 2.0;

#[cfg(not(any(target_os = "macos")))]
pub const PADDING_Y_WITH_TAB_ON_TOP: f32 = 15.0;

#[cfg(target_os = "macos")]
pub const PADDING_Y: f32 = 15.;

#[cfg(target_os = "macos")]
pub const PADDING_Y_WITH_SINGLE_NATIVE_TAB: f32 = 17.;
#[cfg(target_os = "macos")]
pub const PADDING_Y_WITH_MANY_NATIVE_TAB: f32 = 32.;

#[cfg(not(any(target_os = "macos")))]
pub const INACTIVE_TAB_WIDTH_SIZE: f32 = 4.;

#[cfg(target_os = "macos")]
pub const INACTIVE_TAB_WIDTH_SIZE: f32 = 16.;

#[cfg(not(any(target_os = "macos")))]
pub const ACTIVE_TAB_WIDTH_SIZE: f32 = 8.;

#[cfg(target_os = "macos")]
pub const ACTIVE_TAB_WIDTH_SIZE: f32 = 26.;

#[cfg(target_os = "macos")]
pub const DEADZONE_START_Y: f64 = 30.;

#[cfg(target_os = "macos")]
pub const DEADZONE_END_Y: f64 = -2.0;

#[cfg(target_os = "macos")]
pub const DEADZONE_START_X: f64 = 80.;

pub const PADDING_X_COLLAPSED_TABS: f32 = 30.;
