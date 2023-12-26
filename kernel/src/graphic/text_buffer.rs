use ab_glyph::{point, Font, FontRef, Glyph};

use super::{color::Color, FRONT_FRAME_BUFFER};

pub struct TextBuffer<'a> {
    font_text: FontRef<'a>,
    font_bold: FontRef<'a>,
    scale: f32,
}

impl<'a> TextBuffer<'a> {
    pub const fn new(font_text: FontRef<'a>, font_bold: FontRef<'a>, scale: f32) -> Self {
        TextBuffer {
            font_text,
            font_bold,
            scale,
        }
    }

    pub fn write_char(&self, character: char, font_type: FontType, color: Color, x: f32, y: f32) {
        let font = match font_type {
            FontType::Text => &self.font_text,
            FontType::Bold => &self.font_bold,
        };

        let q_glyph: Glyph = font
            .glyph_id(character)
            .with_scale_and_position(self.scale, point(x, y));
        if let Some(q) = font.outline_glyph(q_glyph) {
            let info = FRONT_FRAME_BUFFER.get().unwrap().info;
            let min_x = q.px_bounds().min.x as u32;
            let min_y = q.px_bounds().min.y as u32;
            let stride = info.stride as u32;
            let bit_per_pixel = info.bytes_per_pixel as u32;
            let color = color.encode(info.pixel_format);
            let mut buf = FRONT_FRAME_BUFFER.get().unwrap().frame_buffer.lock();

            q.draw(move |dx, dy, c| {
                let color = [
                    color[0] as f32 * c,
                    color[1] as f32 * c,
                    color[2] as f32 * c,
                ];
                let color = [color[0] as u8, color[1] as u8, color[2] as u8];
                let x = min_x + dx;
                let y = min_y + dy;
                let buf_index = ((y * stride + x) * bit_per_pixel) as usize;

                buf[buf_index..buf_index + 3].copy_from_slice(&color);
            });
        }
    }
}

pub enum FontType {
    Text,
    Bold,
}
