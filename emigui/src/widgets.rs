#![allow(clippy::new_without_default)]

use crate::{layout::Direction, GuiResponse, *};

mod slider;
pub mod text_edit;

pub use {paint::*, slider::*, text_edit::*};

// ----------------------------------------------------------------------------

/// Anything implementing Widget can be added to a Ui with `Ui::add`
pub trait Widget {
    fn ui(self, ui: &mut Ui) -> GuiResponse;
}

// ----------------------------------------------------------------------------

pub struct Label {
    // TODO: not pub
    pub(crate) text: String,
    pub(crate) multiline: bool,
    auto_shrink: bool,
    pub(crate) text_style: TextStyle, // TODO: Option<TextStyle>, where None means "use the default for the ui"
    pub(crate) text_color: Option<Color>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            multiline: true,
            auto_shrink: false,
            text_style: TextStyle::Body,
            text_color: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// If true, will word wrap to `ui.available_finite().width()`.
    /// If false (default), will word wrap to `ui.available().width()`.
    /// This only makes a difference for auto-sized parents.
    pub fn auto_shrink(mut self) -> Self {
        self.auto_shrink = true;
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn layout(&self, ui: &Ui) -> font::Galley {
        let max_width = if self.auto_shrink {
            ui.available_finite().width()
        } else {
            ui.available().width()
        };
        self.layout_width(ui, max_width)
    }

    pub fn layout_width(&self, ui: &Ui, max_width: f32) -> font::Galley {
        let font = &ui.fonts()[self.text_style];
        if self.multiline {
            font.layout_multiline(self.text.clone(), max_width) // TODO: avoid clone
        } else {
            font.layout_single_line(self.text.clone()) // TODO: avoid clone
        }
    }

    pub fn font_height(&self, ui: &Ui) -> f32 {
        ui.fonts()[self.text_style].height()
    }

    // TODO: this should return a LabelLayout which has a paint method.
    // We can then split Widget::Ui in two: layout + allocating space, and painting.
    // this allows us to assemble lables, THEN detect interaction, THEN chose color style based on that.
    // pub fn layout(self, ui: &mut ui) -> LabelLayout { }

    // TODO: a paint method for painting anywhere in a ui.
    // This should be the easiest method of putting text anywhere.

    pub fn paint_galley(&self, ui: &mut Ui, pos: Pos2, galley: font::Galley) {
        ui.add_galley(pos, galley, self.text_style, self.text_color);
    }
}

/// Usage:  label!("Foo: {}", bar)
#[macro_export]
macro_rules! label {
    ($fmt:expr) => ($crate::widgets::Label::new($fmt));
    ($fmt:expr, $($arg:tt)*) => ($crate::widgets::Label::new(format!($fmt, $($arg)*)));
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let galley = self.layout(ui);
        let interact = ui.reserve_space(galley.size, None);
        self.paint_galley(ui, interact.rect.min, galley);
        ui.response(interact)
    }
}

impl Into<Label> for &str {
    fn into(self) -> Label {
        Label::new(self)
    }
}

impl Into<Label> for String {
    fn into(self) -> Label {
        Label::new(self)
    }
}

// ----------------------------------------------------------------------------

pub struct Hyperlink {
    url: String,
    text: String,
}

impl Hyperlink {
    pub fn new(url: impl Into<String>) -> Self {
        let url = url.into();
        Self {
            text: url.clone(),
            url,
        }
    }

    /// Show some other text than the url
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl Widget for Hyperlink {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let Hyperlink { url, text } = self;

        let color = color::LIGHT_BLUE;
        let text_style = TextStyle::Body;
        let id = ui.make_child_id(&url);
        let font = &ui.fonts()[text_style];
        let galley = font.layout_multiline(text, ui.available().width());
        let interact = ui.reserve_space(galley.size, Some(id));
        if interact.hovered {
            ui.ctx().output().cursor_icon = CursorIcon::PointingHand;
        }
        if interact.clicked {
            ui.ctx().output().open_url = Some(url);
        }

        if interact.hovered {
            // Underline:
            for line in &galley.lines {
                let pos = interact.rect.min;
                let y = pos.y + line.y_max;
                let y = ui.round_to_pixel(y);
                let min_x = pos.x + line.min_x();
                let max_x = pos.x + line.max_x();
                ui.add_paint_cmd(PaintCmd::line_segment(
                    [pos2(min_x, y), pos2(max_x, y)],
                    color,
                    ui.style().line_width,
                ));
            }
        }

        ui.add_galley(interact.rect.min, galley, text_style, Some(color));

        ui.response(interact)
    }
}

// ----------------------------------------------------------------------------

pub struct Button {
    text: String,
    text_color: Option<Color>,
    text_style: TextStyle,
    /// None means default for interact
    fill_color: Option<Color>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            text_color: None,
            text_style: TextStyle::Button,
            fill_color: None,
        }
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self
    }

    pub fn fill_color(mut self, fill_color: Option<Color>) -> Self {
        self.fill_color = fill_color;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let Button {
            text,
            text_color,
            text_style,
            fill_color,
        } = self;

        let id = ui.make_position_id();
        let font = &ui.fonts()[text_style];
        let galley = font.layout_multiline(text, ui.available().width());
        let padding = ui.style().button_padding;
        let mut size = galley.size + 2.0 * padding;
        size.y = size.y.max(ui.style().clickable_diameter);
        let interact = ui.reserve_space(size, Some(id));
        let text_cursor = interact.rect.left_center() + vec2(padding.x, -0.5 * galley.size.y);
        let bg_fill_color = fill_color.or(ui.style().interact(&interact).bg_fill_color);
        ui.add_paint_cmd(PaintCmd::Rect {
            corner_radius: ui.style().interact(&interact).corner_radius,
            fill_color: bg_fill_color,
            outline: ui.style().interact(&interact).rect_outline,
            rect: interact.rect,
        });
        let stroke_color = ui.style().interact(&interact).stroke_color;
        let text_color = text_color.unwrap_or(stroke_color);
        ui.add_galley(text_cursor, galley, text_style, Some(text_color));
        ui.response(interact)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    text: String,
    text_color: Option<Color>,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool, text: impl Into<String>) -> Self {
        Checkbox {
            checked,
            text: text.into(),
            text_color: None,
        }
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }
}

impl<'a> Widget for Checkbox<'a> {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let Checkbox {
            checked,
            text,
            text_color,
        } = self;

        let id = ui.make_position_id();
        let text_style = TextStyle::Button;
        let font = &ui.fonts()[text_style];
        let galley = font.layout_single_line(text);
        let interact = ui.reserve_space(
            ui.style().button_padding
                + vec2(ui.style().start_icon_width, 0.0)
                + galley.size
                + ui.style().button_padding,
            Some(id),
        );
        let text_cursor =
            interact.rect.min + ui.style().button_padding + vec2(ui.style().start_icon_width, 0.0);
        if interact.clicked {
            *checked = !*checked;
        }
        let (small_icon_rect, big_icon_rect) = ui.style().icon_rectangles(interact.rect);
        ui.add_paint_cmd(PaintCmd::Rect {
            corner_radius: ui.style().interact(&interact).corner_radius,
            fill_color: ui.style().interact(&interact).bg_fill_color,
            outline: ui.style().interact(&interact).rect_outline,
            rect: big_icon_rect,
        });

        let stroke_color = ui.style().interact(&interact).stroke_color;

        if *checked {
            ui.add_paint_cmd(PaintCmd::LinePath {
                points: vec![
                    pos2(small_icon_rect.left(), small_icon_rect.center().y),
                    pos2(small_icon_rect.center().x, small_icon_rect.bottom()),
                    pos2(small_icon_rect.right(), small_icon_rect.top()),
                ],
                color: stroke_color,
                width: ui.style().line_width,
            });
        }

        let text_color = text_color.unwrap_or(stroke_color);
        ui.add_galley(text_cursor, galley, text_style, Some(text_color));
        ui.response(interact)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct RadioButton {
    checked: bool,
    text: String,
    text_color: Option<Color>,
}

impl RadioButton {
    pub fn new(checked: bool, text: impl Into<String>) -> Self {
        Self {
            checked,
            text: text.into(),
            text_color: None,
        }
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }
}

pub fn radio(checked: bool, text: impl Into<String>) -> RadioButton {
    RadioButton::new(checked, text)
}

impl Widget for RadioButton {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let RadioButton {
            checked,
            text,
            text_color,
        } = self;
        let id = ui.make_position_id();
        let text_style = TextStyle::Button;
        let font = &ui.fonts()[text_style];
        let galley = font.layout_multiline(text, ui.available().width());
        let interact = ui.reserve_space(
            ui.style().button_padding
                + vec2(ui.style().start_icon_width, 0.0)
                + galley.size
                + ui.style().button_padding,
            Some(id),
        );
        let text_cursor =
            interact.rect.min + ui.style().button_padding + vec2(ui.style().start_icon_width, 0.0);

        let bg_fill_color = ui.style().interact(&interact).bg_fill_color;
        let stroke_color = ui.style().interact(&interact).stroke_color;

        let (small_icon_rect, big_icon_rect) = ui.style().icon_rectangles(interact.rect);

        ui.add_paint_cmd(PaintCmd::Circle {
            center: big_icon_rect.center(),
            fill_color: bg_fill_color,
            outline: ui.style().interact(&interact).rect_outline, // TODO
            radius: big_icon_rect.width() / 2.0,
        });

        if checked {
            ui.add_paint_cmd(PaintCmd::Circle {
                center: small_icon_rect.center(),
                fill_color: Some(stroke_color),
                outline: None,
                radius: small_icon_rect.width() / 3.0,
            });
        }

        let text_color = text_color.unwrap_or(stroke_color);
        ui.add_galley(text_cursor, galley, text_style, Some(text_color));
        ui.response(interact)
    }
}

// ----------------------------------------------------------------------------

pub struct Separator {
    line_width: Option<f32>,
    min_spacing: f32,
    extra: f32,
    color: Color,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            line_width: None,
            min_spacing: 6.0,
            extra: 0.0,
            color: color::WHITE,
        }
    }

    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn min_spacing(mut self, min_spacing: f32) -> Self {
        self.min_spacing = min_spacing;
        self
    }

    /// Draw this much longer on each side
    pub fn extra(mut self, extra: f32) -> Self {
        self.extra = extra;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let Separator {
            line_width,
            min_spacing,
            extra,
            color,
        } = self;

        let line_width = line_width.unwrap_or_else(|| ui.style().line_width);

        let available_space = ui.available_finite().size();

        let (points, interact) = match ui.layout().dir() {
            Direction::Horizontal => {
                let interact = ui.reserve_space(vec2(min_spacing, available_space.y), None);
                let r = &interact.rect;
                (
                    [
                        pos2(r.center().x, r.top() - extra),
                        pos2(r.center().x, r.bottom() + extra),
                    ],
                    interact,
                )
            }
            Direction::Vertical => {
                let interact = ui.reserve_space(vec2(available_space.x, min_spacing), None);
                let r = &interact.rect;
                (
                    [
                        pos2(r.left() - extra, r.center().y),
                        pos2(r.right() + extra, r.center().y),
                    ],
                    interact,
                )
            }
        };
        ui.add_paint_cmd(PaintCmd::LineSegment {
            points,
            color: color,
            width: line_width,
        });
        ui.response(interact)
    }
}
