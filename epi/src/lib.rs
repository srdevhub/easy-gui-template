//! Backend-agnostic interface for writing apps using Egui.
//!
//! Egui is a GUI library, which can be plugged in to e.g. a game engine.
//!
//! This crate provides a common interface for programming an app, using Egui,
//! so you can then easily plug it in to a backend such as `egui_web` or `egui_glium`.
//!
//! This crate is primarily used by the `egui_web` and `egui_glium` crates.

#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::dbg_macro,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::filter_map_next,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::match_on_vec_items,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::verbose_file_reads,
    future_incompatible,
    missing_crate_level_docs,
    missing_doc_code_examples,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unused_doc_comments,
)]

pub use egui; // Re-export for user convenience

// ----------------------------------------------------------------------------

/// Implement this trait to write apps that can be compiled both natively using the [`egui_glium`](https://crates.io/crates/egui_glium) crate,
/// and deployed as a web site using the [`egui_web`](https://crates.io/crates/egui_web) crate.
pub trait App {
    /// The name of your App.
    fn name(&self) -> &str;

    /// Background color for the app, e.g. what is sent to `gl.clearColor`.
    /// This is the background of your windows if you don't set a central panel.
    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        egui::Srgba::from_rgb(12, 12, 12).into()
    }

    /// Called once on start. Allows you to restore state.
    fn load(&mut self, _storage: &dyn Storage) {}

    /// Called on shutdown, and perhaps at regular intervals. Allows you to save state.
    fn save(&mut self, _storage: &mut dyn Storage) {}

    /// Time between automatic calls to `save()`
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    /// Called once on shutdown (before or after `save()`)
    fn on_exit(&mut self) {}

    /// Called once before the first frame.
    /// Allows you to do setup code and to call `ctx.set_fonts()`.
    /// Optional.
    fn setup(&mut self, _ctx: &egui::CtxRef) {}

    /// Returns true if this app window should be resizable.
    fn is_resizable(&self) -> bool {
        true
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(&mut self, ctx: &egui::CtxRef, integration_context: &mut IntegrationContext<'_>);
}

pub struct IntegrationContext<'a> {
    /// Information about the integration.
    pub info: IntegrationInfo,
    /// A way to allocate textures (on integrations that support it).
    pub tex_allocator: Option<&'a mut dyn TextureAllocator>,
    /// Where the app can issue commands back to the integration.
    pub output: AppOutput,
    /// If you need to request a repaint from another thread, clone this and send it to that other thread.
    pub repaint_signal: std::sync::Arc<dyn RepaintSignal>,
}

#[derive(Clone, Debug)]
pub struct WebInfo {
    /// e.g. "#fragment" part of "www.example.com/index.html#fragment"
    pub web_location_hash: String,
}

/// Information about the integration passed to the use app each frame.
#[derive(Clone, Debug)]
pub struct IntegrationInfo {
    /// If the app is running in a Web context, this returns information about the environment.
    pub web_info: Option<WebInfo>,

    /// Seconds of cpu usage (in seconds) of UI code on the previous frame.
    /// `None` if this is the first frame.
    pub cpu_usage: Option<f32>,

    /// Local time. Used for the clock in the demo app.
    /// Set to `None` if you don't know.
    pub seconds_since_midnight: Option<f64>,

    /// The OS native pixels-per-point
    pub native_pixels_per_point: Option<f32>,
}

/// Action that can be taken by the user app.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AppOutput {
    /// Set to `true` to stop the app.
    /// This does nothing for web apps.
    pub quit: bool,

    /// Set to some size to resize the outer window (e.g. glium window) to this size.
    pub window_size: Option<egui::Vec2>,

    /// If the app sets this, change the `pixels_per_point` of Egui to this next frame.
    pub pixels_per_point: Option<f32>,
}

pub trait TextureAllocator {
    /// A.locate a new user texture.
    fn alloc(&mut self) -> egui::TextureId;

    /// Set or change the pixels of a user texture.
    fn set_srgba_premultiplied(
        &mut self,
        id: egui::TextureId,
        size: (usize, usize),
        srgba_pixels: &[egui::Srgba],
    );

    /// Free the given texture.
    fn free(&mut self, id: egui::TextureId);
}

pub trait RepaintSignal: Send + Sync {
    /// This signals the Egui integration that a repaint is required.
    /// This is meant to be called when a background process finishes in an async context and/or background thread.
    fn request_repaint(&self);
}

// ----------------------------------------------------------------------------

/// A place where you can store custom data in a way that persists when you restart the app.
///
/// On the web this is backed by [local storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
/// On desktop this is backed by the file system.
pub trait Storage {
    fn get_string(&self, key: &str) -> Option<String>;
    fn set_string(&mut self, key: &str, value: String);

    /// write-to-disk or similar
    fn flush(&mut self);
}

/// Stores nothing.
#[derive(Clone, Default)]
pub struct DummyStorage {}

impl Storage for DummyStorage {
    fn get_string(&self, _key: &str) -> Option<String> {
        None
    }
    fn set_string(&mut self, _key: &str, _value: String) {}
    fn flush(&mut self) {}
}

#[cfg(feature = "serde_json")]
pub fn get_value<T: serde::de::DeserializeOwned>(storage: &dyn Storage, key: &str) -> Option<T> {
    storage
        .get_string(key)
        .and_then(|value| serde_json::from_str(&value).ok())
}

#[cfg(feature = "serde_json")]
pub fn set_value<T: serde::Serialize>(storage: &mut dyn Storage, key: &str, value: &T) {
    storage.set_string(key, serde_json::to_string(value).unwrap());
}

/// storage key used for app
pub const APP_KEY: &str = "app";

// ----------------------------------------------------------------------------

pub mod http {
    pub struct Request {
        /// "GET", …
        pub method: String,
        /// https://…
        pub url: String,
    }

    impl Request {
        pub fn get(url: String) -> Self {
            Self {
                method: "GET".to_owned(),
                url,
            }
        }
    }

    /// Response from an HTTP request for a very simple HTTP fetch API in `eframe`.
    pub struct Response {
        /// The URL we ended up at. This can differ from the request url when we have followed redirects.
        pub url: String,
        pub ok: bool,
        pub status: u16,
        pub status_text: String,

        /// Content-Type header, or empty string if missing.
        pub header_content_type: String,

        /// The raw bytes.
        pub bytes: Vec<u8>,

        /// UTF-8 decoded version of bytes.
        /// ONLY if `header_content_type` starts with "text" and bytes is UTF-8.
        pub text: Option<String>,
    }
}
