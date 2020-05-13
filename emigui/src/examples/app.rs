// #![allow(dead_code, unused_variables)] // should be commented out
use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{color::*, containers::*, examples::FractalClock, widgets::*, *};

// ----------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ExampleApp {
    example_window: ExampleWindow,
    open_windows: OpenWindows,
    fractal_clock: FractalClock,
}

impl ExampleApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        show_menu_bar(ui, &mut self.open_windows);
        ui.add(Separator::new());
        self.windows(ui.ctx());
    }

    pub fn windows(&mut self, ctx: &Arc<Context>) {
        // TODO: Make it even simpler to show a window

        // TODO: window manager for automatic positioning?

        let ExampleApp {
            example_window,
            open_windows,
            fractal_clock,
        } = self;

        if ctx.previus_input().web != ctx.input().web {
            let location_hash = ctx
                .input()
                .web
                .as_ref()
                .map(|web| web.location_hash.as_str());

            // #fragment end of URL:
            if location_hash == Some("#clock") {
                *open_windows = OpenWindows {
                    fractal_clock: true,
                    ..OpenWindows::none()
                };
            }
        }

        Window::new("Examples")
            .open(&mut open_windows.examples)
            .default_pos(pos2(32.0, 100.0))
            .default_size(vec2(430.0, 600.0))
            .show(ctx, |ui| {
                example_window.ui(ui);
            });

        Window::new("Settings")
            .open(&mut open_windows.settings)
            .default_pos(pos2(500.0, 100.0))
            .default_size(vec2(350.0, 400.0))
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        Window::new("Inspection")
            .open(&mut open_windows.inspection)
            .default_pos(pos2(500.0, 400.0))
            .default_size(vec2(400.0, 300.0))
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

        Window::new("Memory")
            .open(&mut open_windows.memory)
            .default_pos(pos2(700.0, 350.0))
            .auto_sized()
            .show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        fractal_clock.window(ctx, &mut open_windows.fractal_clock);
    }
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct OpenWindows {
    examples: bool,
    settings: bool,
    inspection: bool,
    memory: bool,
    fractal_clock: bool,
}

impl Default for OpenWindows {
    fn default() -> Self {
        Self {
            examples: true,
            ..OpenWindows::none()
        }
    }
}

impl OpenWindows {
    fn none() -> Self {
        Self {
            examples: false,
            settings: false,
            inspection: false,
            memory: false,
            fractal_clock: false,
        }
    }
}

fn show_menu_bar(ui: &mut Ui, windows: &mut OpenWindows) {
    menu::bar(ui, |ui| {
        menu::menu(ui, "File", |ui| {
            ui.add(Button::new("Do nothing"));
            ui.add(Button::new("Carry on"));
            ui.add(Button::new("Don't Quit"));
        });
        menu::menu(ui, "Windows", |ui| {
            ui.add(Checkbox::new(&mut windows.examples, "Examples"));
            ui.add(Checkbox::new(&mut windows.settings, "Settings"));
            ui.add(Checkbox::new(&mut windows.inspection, "Inspection"));
            ui.add(Checkbox::new(&mut windows.memory, "Memory"));
            ui.add(Checkbox::new(&mut windows.fractal_clock, "Fractal Clock"));
        });
        menu::menu(ui, "About", |ui| {
            ui.add(label!("This is Emigui"));
            ui.add(Hyperlink::new("https://github.com/emilk/emigui/").text("Emigui home page"));
        });
    });
}

// ----------------------------------------------------------------------------

/// Showcase some ui code
#[derive(Deserialize, Serialize)]
pub struct ExampleWindow {
    num_columns: usize,

    widgets: Widgets,
    layout: LayoutExample,
    box_painting: BoxPainting,
    painting: Painting,
}

impl Default for ExampleWindow {
    fn default() -> ExampleWindow {
        ExampleWindow {
            num_columns: 2,

            widgets: Default::default(),
            layout: Default::default(),
            box_painting: Default::default(),
            painting: Default::default(),
        }
    }
}

impl ExampleWindow {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.collapsing("About Emigui", |ui| {
            ui.add(label!(
                "Emigui is an experimental immediate mode GUI written in Rust."
            ));

            ui.horizontal(|ui| {
                ui.add_label("Project home page:");
                ui.add_hyperlink("https://github.com/emilk/emigui/");
            });
        });

        CollapsingHeader::new("Widgets")
            .default_open()
            .show(ui, |ui| {
                self.widgets.ui(ui);
            });

        CollapsingHeader::new("Layout")
            .default_open()
            .show(ui, |ui| self.layout.ui(ui));

        ui.collapsing("Columns", |ui| {
            ui.add(Slider::usize(&mut self.num_columns, 1..=10).text("Columns"));
            ui.columns(self.num_columns, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    col.add(label!("Column {} out of {}", i + 1, self.num_columns));
                    if i + 1 == self.num_columns && col.add(Button::new("Delete this")).clicked {
                        self.num_columns -= 1;
                    }
                }
            });
        });

        ui.collapsing("Test box rendering", |ui| self.box_painting.ui(ui));

        CollapsingHeader::new("Scroll area")
            // .default_open()
            .show(ui, |ui| {
                ScrollArea::default().show(ui, |ui| {
                    ui.add_label(LOREM_IPSUM);
                });
            });

        CollapsingHeader::new("Painting")
            // .default_open()
            .show(ui, |ui| self.painting.ui(ui));

        CollapsingHeader::new("Resize")
            // .default_open()
            .show(ui, |ui| {
                Resize::default()
                    .default_height(200.0)
                    // .as_wide_as_possible()
                    .auto_shrink_height(false)
                    .show(ui, |ui| {
                        ui.add(label!("This ui can be resized!"));
                        ui.add(label!("Just pull the handle on the bottom right"));
                    });
            });

        ui.collapsing("Name clash example", |ui| {
            ui.add_label("\
                Widgets that store state require unique identifiers so we can track their state between frames. \
                Identifiers are normally derived from the titles of the widget.");

            ui.add_label("\
                For instance, collapsable headers needs to store wether or not they are open. \
                If you fail to give them unique names then clicking one will open both. \
                To help you debug this, an error message is printed on screen:");

            ui.collapsing("Collapsing header", |ui| {
                ui.add_label("Contents of first folddable ui");
            });
            ui.collapsing("Collapsing header", |ui| {
                ui.add_label("Contents of second folddable ui");
            });

            ui.add_label("\
                Most widgets don't need unique names, but are tracked \
                based on their position on screen. For instance, buttons:");
            ui.add(Button::new("Button"));
            ui.add(Button::new("Button"));
        });
    }
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct Widgets {
    checked: bool,
    count: usize,
    radio: usize,
    slider_value: usize,
    text_inputs: [String; 3],
}

impl Default for Widgets {
    fn default() -> Self {
        Self {
            checked: true,
            radio: 0,
            count: 0,
            slider_value: 100,
            text_inputs: ["Hello".to_string(), "World".to_string(), "".to_string()],
        }
    }
}

impl Widgets {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
                ui.add(label!("Text can have").text_color(srgba(110, 255, 110, 255)));
                ui.add(label!("color").text_color(srgba(128, 140, 255, 255)));
                ui.add(label!("and tooltips (hover me)")).tooltip_text(
                    "This is a multiline tooltip that demonstrates that you can easily add tooltips to any element.\nThis is the second line.\nThis is the third.",
                );
            });

        ui.add(Checkbox::new(&mut self.checked, "checkbox"));

        ui.horizontal(|ui| {
            if ui.add(radio(self.radio == 0, "First")).clicked {
                self.radio = 0;
            }
            if ui.add(radio(self.radio == 1, "Second")).clicked {
                self.radio = 1;
            }
            if ui.add(radio(self.radio == 2, "Final")).clicked {
                self.radio = 2;
            }
        });

        ui.horizontal(|ui| {
            if ui
                .add(Button::new("Click me"))
                .tooltip_text("This will just increase a counter.")
                .clicked
            {
                self.count += 1;
            }
            ui.add(label!("The button has been clicked {} times", self.count));
        });

        ui.add(Slider::usize(&mut self.slider_value, 1..=1000).text("value"));
        if ui.add(Button::new("Double it")).clicked {
            self.slider_value *= 2;
        }

        for (i, text) in self.text_inputs.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add(label!("Text input {}: ", i));
                ui.add(TextEdit::new(text).id(i));
            }); // TODO: .tooltip_text("Enter text to edit me")
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct BoxPainting {
    size: Vec2,
    corner_radius: f32,
    stroke_width: f32,
    num_boxes: usize,
}

impl Default for BoxPainting {
    fn default() -> Self {
        Self {
            size: vec2(100.0, 50.0),
            corner_radius: 5.0,
            stroke_width: 2.0,
            num_boxes: 1,
        }
    }
}

impl BoxPainting {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::f32(&mut self.size.x, 0.0..=500.0).text("width"));
        ui.add(Slider::f32(&mut self.size.y, 0.0..=500.0).text("height"));
        ui.add(Slider::f32(&mut self.corner_radius, 0.0..=50.0).text("corner_radius"));
        ui.add(Slider::f32(&mut self.stroke_width, 0.0..=10.0).text("stroke_width"));
        ui.add(Slider::usize(&mut self.num_boxes, 0..=5).text("num_boxes"));

        let pos = ui
            .reserve_space(
                vec2(self.size.x * (self.num_boxes as f32), self.size.y),
                None,
            )
            .rect
            .min;

        let mut cmds = vec![];
        for i in 0..self.num_boxes {
            cmds.push(PaintCmd::Rect {
                corner_radius: self.corner_radius,
                fill_color: Some(gray(136, 255)),
                rect: Rect::from_min_size(
                    pos2(10.0 + pos.x + (i as f32) * (self.size.x * 1.1), pos.y),
                    self.size,
                ),
                outline: Some(Outline::new(self.stroke_width, gray(255, 255))),
            });
        }
        ui.add_paint_cmds(cmds);
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
struct Painting {
    lines: Vec<Vec<Vec2>>,
}

impl Painting {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.add_label("Draw with your mouse to paint");
        if ui.add(Button::new("Clear")).clicked {
            self.lines.clear();
        }

        ui.add_custom_contents(vec2(f32::INFINITY, 200.0), |ui| {
            let interact = ui.reserve_space(ui.available_finite().size(), Some(ui.id()));
            let rect = interact.rect;
            ui.set_clip_rect(ui.clip_rect().intersect(rect)); // Make sure we don't paint out of bounds

            if self.lines.is_empty() {
                self.lines.push(vec![]);
            }

            let current_line = self.lines.last_mut().unwrap();

            if interact.active {
                if let Some(mouse_pos) = ui.input().mouse_pos {
                    let canvas_pos = mouse_pos - rect.min;
                    if current_line.last() != Some(&canvas_pos) {
                        current_line.push(canvas_pos);
                    }
                }
            } else if !current_line.is_empty() {
                self.lines.push(vec![]);
            }

            for line in &self.lines {
                if line.len() >= 2 {
                    ui.add_paint_cmd(PaintCmd::LinePath {
                        points: line.iter().map(|p| rect.min + *p).collect(),
                        color: LIGHT_GRAY,
                        width: 2.0,
                    });
                }
            }

            // Frame it:
            ui.add_paint_cmd(PaintCmd::Rect {
                rect: ui.rect(),
                corner_radius: 0.0,
                fill_color: None,
                outline: Some(Outline::new(1.0, WHITE)),
            });
        });
    }
}

// ----------------------------------------------------------------------------

use crate::layout::*;

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct LayoutExample {
    dir: Direction,
    align: Option<Align>, // None == jusitifed
}

impl Default for LayoutExample {
    fn default() -> Self {
        Self {
            dir: Direction::Vertical,
            align: Some(Align::Center),
        }
    }
}

impl LayoutExample {
    pub fn ui(&mut self, ui: &mut Ui) {
        Resize::default()
            .default_size(vec2(200.0, 200.0))
            .show(ui, |ui| self.contents_ui(ui));
    }

    pub fn contents_ui(&mut self, ui: &mut Ui) {
        ui.set_layout(Layout::from_dir_align(self.dir, self.align));

        ui.add(label!("Available space: {:?}", ui.available().size()));
        if ui.add(Button::new("Reset")).clicked {
            *self = Default::default();
        }
        ui.add(Separator::new());
        ui.add(label!("Direction:"));

        // TODO: enum iter

        for &dir in &[Direction::Horizontal, Direction::Vertical] {
            if ui
                .add(RadioButton::new(self.dir == dir, format!("{:?}", dir)))
                .clicked
            {
                self.dir = dir;
            }
        }

        ui.add(Separator::new());

        ui.add(label!("Align:"));

        for &align in &[Align::Min, Align::Center, Align::Max] {
            if ui
                .add(RadioButton::new(
                    self.align == Some(align),
                    format!("{:?}", align),
                ))
                .clicked
            {
                self.align = Some(align);
            }
        }
        if ui
            .add(RadioButton::new(self.align == None, "Justified"))
            .tooltip_text("Try to fill full width/heigth (e.g. buttons)")
            .clicked
        {
            self.align = None;
        }
    }
}

// ----------------------------------------------------------------------------

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris. Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.";
