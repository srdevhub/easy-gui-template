use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{
    color::Color,
    font::Galley,
    fonts::TextStyle,
    math::{Pos2, Rect},
    mesher::{Mesh, Path},
    Context, Ui,
};

// ----------------------------------------------------------------------------

#[derive(Clone, Default, Serialize)]
pub struct Output {
    pub cursor_icon: CursorIcon,

    /// If set, open this url.
    pub open_url: Option<String>,

    /// Response to Event::Copy or Event::Cut. Ignore if empty.
    pub copied_text: String,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CursorIcon {
    Default,
    /// Pointing hand, used for e.g. web links
    PointingHand,
    ResizeNwSe,
    Text,
}

impl Default for CursorIcon {
    fn default() -> Self {
        Self::Default
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct InteractInfo {
    /// The mouse is hovering above this thing
    pub hovered: bool,

    /// The mouse pressed this thing ealier, and now released on this thing too.
    pub clicked: bool,

    /// The mouse is interacting with this thing (e.g. dragging it or holding it)
    pub active: bool,

    /// The region of the screen we are talking about
    pub rect: Rect,
}

impl InteractInfo {
    pub fn union(self, other: Self) -> Self {
        Self {
            hovered: self.hovered || other.hovered,
            clicked: self.clicked || other.clicked,
            active: self.active || other.active,
            rect: self.rect.union(other.rect),
        }
    }
}

// ----------------------------------------------------------------------------

// TODO: rename GuiResponse
pub struct GuiResponse {
    /// The mouse is hovering above this
    pub hovered: bool,

    /// The mouse clicked this thing this frame
    pub clicked: bool,

    /// The mouse is interacting with this thing (e.g. dragging it)
    pub active: bool,

    /// The area of the screen we are talking about
    pub rect: Rect,

    /// Used for optionally showing a tooltip
    pub ctx: Arc<Context>,
}

impl GuiResponse {
    /// Show some stuff if the item was hovered
    pub fn tooltip(&mut self, add_contents: impl FnOnce(&mut Ui)) -> &mut Self {
        if self.hovered {
            crate::containers::show_tooltip(&self.ctx, add_contents);
        }
        self
    }

    /// Show this text if the item was hovered
    pub fn tooltip_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.tooltip(|popup| {
            popup.add(crate::widgets::Label::new(text));
        })
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Outline {
    pub width: f32,
    pub color: Color,
}

impl Outline {
    pub fn new(width: impl Into<f32>, color: impl Into<Color>) -> Self {
        Self {
            width: width.into(),
            color: color.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PaintCmd {
    Circle {
        center: Pos2,
        fill_color: Option<Color>,
        outline: Option<Outline>,
        radius: f32,
    },
    LineSegment {
        points: [Pos2; 2],
        color: Color,
        width: f32,
    },
    LinePath {
        points: Vec<Pos2>,
        color: Color,
        width: f32,
    },
    Path {
        path: Path,
        closed: bool,
        fill_color: Option<Color>,
        outline: Option<Outline>,
    },
    Rect {
        rect: Rect,
        corner_radius: f32,
        fill_color: Option<Color>,
        outline: Option<Outline>,
    },
    /// Paint a single line of text
    Text {
        /// Top left corner of the first character.
        pos: Pos2,
        /// The layed out text
        galley: Galley,
        text_style: TextStyle, // TODO: Font?
        color: Color,
    },
    /// Low-level triangle mesh
    Mesh(Mesh),
}

impl PaintCmd {
    pub fn line_segment(points: [Pos2; 2], color: Color, width: f32) -> Self {
        Self::LineSegment {
            points,
            color,
            width,
        }
    }
}
