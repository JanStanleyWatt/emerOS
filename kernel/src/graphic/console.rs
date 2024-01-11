//! コンソール機能を定義するカーネル部分のモジュール

use core::fmt::Write;

use common_lib::graphic::console::{Console, FontType};

use crate::{FRAME_BUFFER_INFO, TEXT_BUFFER, TEXT_BUFFER_HEIGHT, TEXT_BUFFER_WIDTH};

use super::text_buffer::{TextBuffer, CURSOR_DEFAULT_POSITION};

impl<'a> Console for TextBuffer<'a> {
    fn put_char(&mut self, character: char, font_type: FontType, r_g_b: [u8; 3]) {
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
                let fb_info = FRAME_BUFFER_INFO.get().unwrap();

                if let Some(g) = &glyph {
                    self.write_buffer(g, r_g_b, fb_info);

                    self.cursor.0 += match character {
                        // ASCII文字または半角カタカナの場合はカーソルを横方向'に１つ進める
                        '\u{0}'..='\u{7f}' => 1,
                        '\u{ff61}'..='\u{ff9f}' => 1,
                        // それ以外の場合は横方向に２つ進める
                        _ => 2,
                    };

                    let width = TEXT_BUFFER_WIDTH.get().unwrap().get();
                    if self.cursor.0 >= width {
                        self.new_line();
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn new_line(&mut self) {
        if self.cursor.0 > 0 {
            self.carriage_return()
        }

        if self.cursor.1 >= TEXT_BUFFER_HEIGHT.get().unwrap().get() {
            todo!()
        } else {
            self.cursor.1 += 1
        }
    }

    #[inline(always)]
    fn carriage_return(&mut self) {
        self.cursor.0 = 0;
    }

    fn reset(&mut self) {
        self.clear();
        self.cursor = CURSOR_DEFAULT_POSITION;
    }
}

/// `println!()`などに使う`core::fmt::Write`の実装
impl<'a> core::fmt::Write for TextBuffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.put_char(c, FontType::Text, [255, 255, 255]);
        }
        self.merge_buffer();

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.put_char(c, FontType::Text, [255, 255, 255]);
        self.merge_buffer();

        Ok(())
    }
}

/// 画面に文字列を描画するマクロ。graphic::init()の処理が終わってから使用すること
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphic::console::_print(format_args!($($arg)*)));
}

/// 画面に文字列を描画し、最後に改行を行うマクロ。graphic::init()の処理が終わってから使用すること
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    TEXT_BUFFER.get().unwrap().lock().write_fmt(args).unwrap();
}
