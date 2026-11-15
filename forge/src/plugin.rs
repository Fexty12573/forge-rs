//! Plugins as a type with state, instead of free callback functions.
//!
//! Implement [`Plugin`] for your state type and put `#[forge::plugin]` on the impl. The attribute
//! exports the loader callbacks for every hook you implement, stores the plugin in a static and calls
//! each hook with `&mut self`.
//!
//! ```ignore
//! struct MyPlugin {
//!     frames: u64,
//!     logo: Option<Owned<Texture>>,
//! }
//!
//! #[forge::plugin]
//! impl forge::Plugin for MyPlugin {
//!     const NAME: &'static str = "My Plugin";
//!
//!     fn new() -> Self {
//!         Self { frames: 0, logo: None }
//!     }
//!
//!     fn on_update(&mut self, _dt: f32) {
//!         self.frames += 1;
//!     }
//! }
//!
//! // Anywhere else, e.g. in a game function hook:
//! MyPlugin::with(|plugin| plugin.frames = 0);
//! ```
//!
//! # Locking
//! The plugin lives in a [`Mutex`], and every hook runs while holding it. Hooks are therefore never
//! called concurrently, even though the loader calls them from different game threads (one hook waits
//! for the other to return).
//!
//! The mutex is not recursive. Calling [`Plugin::with`] while the current thread is already inside a
//! hook or another `with` panics. The usual way to hit this is a game function hook that calls `with`,
//! where the game function is called from inside one of your plugin hooks. Use [`Plugin::try_with`]
//! in game function hooks that can be reached that way.
//!
//! `#[forge::plugin]` defines `forge_onLoad` and `forge_onInit`, so it can't be combined with
//! `#[forge::entry]`. The other callback attributes (`#[forge::imgui_render]`, ...) can still be used
//! for hooks the impl doesn't define.

use core::ffi::c_char;

use crate::os::sync::Mutex;

#[cfg(feature = "nvn")]
use crate::nvn::window::Window;

/// A plugin and its state. See the [module documentation](self).
///
/// Every hook has an empty default, and `#[forge::plugin]` only exports the hooks the impl defines,
/// so the loader skips the rest entirely.
pub trait Plugin: Send + Sized + 'static {
    /// Name shown in the loader's plugin list. At most 63 bytes are used.
    const NAME: &'static str = "";

    /// Creates the plugin. Called once, right after the plugin is loaded and the logger is set up.
    ///
    /// [`with`](Self::with) panics and [`try_with`](Self::try_with) returns `None` until this returns.
    fn new() -> Self;

    /// Called once per game frame, from the game's main thread, with the frame time in seconds.
    fn on_update(&mut self, _dt: f32) {}

    /// Called once NVN is initialized. The `forge::nvn::with_*` functions can be used from here on.
    #[cfg(feature = "nvn")]
    fn on_nvn_ready(&mut self) {}

    /// Called after the game finished rendering a frame, before it's presented.
    ///
    /// ImGui functions must not be used here.
    #[cfg(feature = "nvn")]
    fn on_nvn_render(&mut self, _window: &mut Window, _texture_index: i32) {}

    /// Called to draw ImGui widgets inside the main forge window.
    #[cfg(feature = "imgui")]
    fn on_imgui_render(&mut self) {}

    /// Called to draw ImGui UI outside of the main forge window. Requires your own begin/end calls.
    #[cfg(feature = "imgui")]
    fn on_imgui_free_render(&mut self) {}

    /// Implemented by `#[forge::plugin]`.
    #[doc(hidden)]
    fn __instance() -> &'static __private::Instance<Self>;

    /// Runs `f` with exclusive access to the plugin.
    ///
    /// # Panics
    /// Panics if called before [`new`](Self::new) returned, or if the current thread already has
    /// access to the plugin (inside a hook or another `with`).
    #[track_caller]
    fn with<R>(f: impl FnOnce(&mut Self) -> R) -> R {
        Self::__instance().with(f)
    }

    /// Runs `f` with exclusive access to the plugin, unless that isn't possible right now.
    ///
    /// Returns `None` without blocking if [`new`](Self::new) hasn't returned yet or the plugin is
    /// currently in use, by any thread including the current one.
    fn try_with<R>(f: impl FnOnce(&mut Self) -> R) -> Option<R> {
        Self::__instance().try_with(f)
    }
}

/// Used by the code `#[forge::plugin]` generates. Not public API.
#[doc(hidden)]
pub mod __private {
    use super::*;
    use crate::os::sync::OnceLock;
    use sys::init::PluginInitParams;

    pub struct Instance<T>(OnceLock<Mutex<T>>);

    impl<T> Instance<T> {
        #[allow(clippy::new_without_default)]
        pub const fn new() -> Self {
            Self(OnceLock::new())
        }

        #[track_caller]
        pub(super) fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            self.0.get().expect("Plugin accessed before Plugin::new returned").with(f)
        }

        pub(super) fn try_with<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
            self.0.get()?.try_lock().map(|mut guard| f(&mut guard))
        }
    }

    /// # Safety
    /// `params` must be the pointer the loader passes to `forge_onLoad`.
    pub unsafe fn on_load<T: Plugin>(params: *mut PluginInitParams) {
        let params = unsafe { &mut *params };
        params.required_version = crate::REQUIRED_VERSION;
        write_name(&mut params.name, T::NAME);
    }

    pub fn on_init<T: Plugin>() {
        // Only fails if a logger is already set, in which case logging works anyway.
        let _ = crate::log::init();
        T::__instance().0.get_or_init(|| Mutex::new(T::new()));
    }

    pub fn on_update<T: Plugin>(dt: f32) {
        T::with(|p| p.on_update(dt));
    }

    #[cfg(feature = "nvn")]
    pub fn on_nvn_ready<T: Plugin>() {
        T::with(|p| p.on_nvn_ready());
    }

    #[cfg(feature = "nvn")]
    pub unsafe fn on_nvn_render<T: Plugin>(window: *mut sys::nvn::types::NVNwindow, texture_index: i32) {
        let window = unsafe { Window::from_ptr_mut(window) };
        T::with(|p| p.on_nvn_render(window, texture_index));
    }

    #[cfg(feature = "imgui")]
    pub fn on_imgui_render<T: Plugin>() {
        T::with(|p| p.on_imgui_render());
    }

    #[cfg(feature = "imgui")]
    pub fn on_imgui_free_render<T: Plugin>() {
        T::with(|p| p.on_imgui_free_render());
    }

    /// Copies `name` into `out` as a NUL-terminated string, truncated to a whole UTF-8 character.
    fn write_name(out: &mut [c_char], name: &str) {
        let mut len = name.len().min(out.len() - 1);
        while !name.is_char_boundary(len) {
            len -= 1;
        }

        for (dst, &src) in out.iter_mut().zip(&name.as_bytes()[..len]) {
            *dst = src as c_char;
        }
        out[len] = 0;
    }
}
