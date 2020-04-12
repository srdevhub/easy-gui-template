#![deny(warnings)]

extern crate rusttype;
extern crate serde;

#[macro_use] // TODO: get rid of this
extern crate serde_derive;

pub mod color;
mod emigui;
pub mod example_app;
mod font;
mod fonts;
mod layout;
pub mod math;
pub mod mesher;
mod style;
mod texture_atlas;
mod types;
pub mod widgets;

pub use {
    color::Color,
    emigui::Emigui,
    fonts::{FontDefinitions, Fonts, TextStyle},
    layout::{Align, Region},
    math::*,
    mesher::{Mesh, Vertex},
    style::Style,
    texture_atlas::Texture,
    types::*,
};
