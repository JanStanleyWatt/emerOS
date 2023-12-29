use common_lib::graphic::console::{Console, FontType};

use super::text_buffer::TextBuffer;

impl<'a> Console for TextBuffer<'a> {
    fn new_line(&mut self) {
        if self.cursor.0 > 0 {
            self.carriage_return()
        }
        if self.cursor.1 >= self.height() {
            todo!()
        } else {
            self.cursor.1 += 1
        }
    }

    fn carriage_return(&mut self) {
        self.cursor.0 = 0;
    }

    fn width(&self) -> usize {
        (self.info.stride as f32 / (self.scale / 2.0)) as usize
    }

    fn height(&self) -> usize {
        (self.info.height as f32 / self.scale) as usize
    }

    fn put_char(&mut self, character: char, font_type: FontType, r_d_b: [u8; 3]) {
        match character {
            // 制御文字の場合はそれに従った処理を行う
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            // 制御文字以外はフレームバッファに文字を描画し、カーソルを進める
            _ => {
                let font = match font_type {
                    FontType::Text => &self.font_text,
                    FontType::Bold => &self.font_bold,
                };

                let glyph = self.get_glyph(character, font);

                if let Some(g) = &glyph {
                    self.write_buffer(g, r_d_b);

                    self.cursor.0 += match character {
                        // ASCII文字または半角カタカナの場合はカーソルを横方向に２つ進める
                        '\u{0}'..='\u{7f}' => 1,
                        '\u{ff61}'..='\u{ff9f}' => 1,
                        // それ以外の場合は横方向に１つ進める
                        _ => 2,
                    };

                    if self.cursor.0 >= self.width() {
                        self.new_line();
                    }
                }
            }
        }
    }
}

impl<'a> core::fmt::Write for TextBuffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.put_char(c, FontType::Text, [255, 255, 255]);
        }

        Ok(())
    }
}
