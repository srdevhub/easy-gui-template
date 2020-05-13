#[derive(Clone, Default)]
pub struct Texture {
    /// e.g. a hash of the data. Use this to detect changes!
    pub id: u64, // TODO
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl std::ops::Index<(usize, usize)> for Texture {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Texture {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }
}

/// A texture pixels, used for fonts.
#[derive(Clone, Default)]
pub struct TextureAtlas {
    texture: Texture,

    /// Used for when adding new rects
    cursor: (usize, usize),
    row_height: usize,
}

impl TextureAtlas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            texture: Texture {
                id: 0,
                width,
                height,
                pixels: vec![0; width * height],
            },
            ..Default::default()
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut Texture {
        self.texture.id += 1;
        &mut self.texture
    }

    pub fn clear(&mut self) {
        self.cursor = (0, 0);
        self.row_height = 0;
    }

    /// Returns the coordinates of where the rect ended up.
    pub fn allocate(&mut self, (w, h): (usize, usize)) -> (usize, usize) {
        assert!(w <= self.texture.width);
        if self.cursor.0 + w > self.texture.width {
            // New row:
            self.cursor.0 = 0;
            self.cursor.1 += self.row_height;
            self.row_height = 0;
        }

        self.row_height = self.row_height.max(h);
        while self.cursor.1 + self.row_height >= self.texture.height {
            self.texture.height *= 2;
        }

        if self.texture.width * self.texture.height > self.texture.pixels.len() {
            self.texture
                .pixels
                .resize(self.texture.width * self.texture.height, 0);
        }

        let pos = self.cursor;
        self.cursor.0 += w;
        self.texture.id += 1;
        (pos.0 as usize, pos.1 as usize)
    }
}

impl Texture {
    pub fn ui(&self, ui: &mut crate::Ui) {
        use crate::{
            color::WHITE, containers::show_tooltip, label, math::*, Mesh, PaintCmd, Vertex,
        };

        ui.add(label!(
            "Texture size: {} x {} (hover to zoom)",
            self.width,
            self.height
        ));
        let mut size = vec2(self.width as f32, self.height as f32);
        if size.x > ui.available().width() {
            size *= ui.available().width() / size.x;
        }
        let interact = ui.reserve_space(size, None);
        let rect = interact.rect;
        let top_left = Vertex {
            pos: rect.min,
            uv: (0, 0),
            color: WHITE,
        };
        let bottom_right = Vertex {
            pos: rect.max,
            uv: (self.width as u16 - 1, self.height as u16 - 1),
            color: WHITE,
        };
        let mut mesh = Mesh::default();
        mesh.add_rect(top_left, bottom_right);
        ui.add_paint_cmd(PaintCmd::Mesh(mesh));

        if interact.hovered {
            show_tooltip(ui.ctx(), |ui| {
                let pos = ui.top_left();
                let zoom_rect = ui.reserve_space(vec2(128.0, 128.0), None).rect;
                let u = remap_clamp(pos.x, rect.range_x(), 0.0..=self.width as f32 - 1.0).round();
                let v = remap_clamp(pos.y, rect.range_y(), 0.0..=self.height as f32 - 1.0).round();

                let texel_radius = 32.0;
                let u = clamp(u, texel_radius..=self.width as f32 - 1.0 - texel_radius);
                let v = clamp(v, texel_radius..=self.height as f32 - 1.0 - texel_radius);

                let top_left = Vertex {
                    pos: zoom_rect.min,
                    uv: ((u - texel_radius) as u16, (v - texel_radius) as u16),
                    color: WHITE,
                };
                let bottom_right = Vertex {
                    pos: zoom_rect.max,
                    uv: ((u + texel_radius) as u16, (v + texel_radius) as u16),
                    color: WHITE,
                };
                let mut mesh = Mesh::default();
                mesh.add_rect(top_left, bottom_right);
                ui.add_paint_cmd(PaintCmd::Mesh(mesh));
            });
        }
    }
}
