//! NSH on HarmonyOS and OpenHarmony: ArkUI native views, one engine per
//! `NshSurface`, in a standard UIAbility host. A platform built outside NSH
//! on its extension points (NSH's `docs/dev/platforms.md`): apps see it as
//! `platform.name === "harmonyos"` on `platform.os === "android"`.
//! Each rendering surface owns one engine and ArkUI tree.
//!
//! The engine lays out text off the UI thread and transfers owned native drawing
//! commands with selection geometry. Native views
//! consume one mount batch per wake, with no per-node ArkTS calls.
#![cfg(target_env = "ohos")]

mod accessibility;
mod app;
mod input;
mod interop;
mod linked;
mod media;
mod navigation;
mod render;
mod scroll;
mod services;
mod text;

/// The platform's name: `Environment::platform`, `platform.name` in apps
/// and `--target` on the command line.
pub const PLATFORM: &str = "harmonyos";

pub use app::{
    NshCapabilities, NshEnvironment, NshOptions, NshPackage, NshPackageInfo, NshSurface, NshTrust,
    get_capabilities, init_logging,
};
pub use interop::NshNativeProperty;
pub use linked::LinkedApp;

#[doc(hidden)]
pub mod __private {
    /// `nsh/base`'s UI set, for a TSX app's runner.
    pub fn nsh_base_registry() -> nsh_core::element::Registry {
        nsh_base::registry()
    }

    /// Referenced by the runner so the linker keeps this library's objects
    /// and their N-API registrations.
    #[inline(never)]
    pub fn link() {
        std::hint::black_box(crate::app::get_capabilities as *const ());
    }
}
