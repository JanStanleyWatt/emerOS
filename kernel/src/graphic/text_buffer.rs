//! 画面にテキストを描画するモジュール
//!
//! I/Oに強く関与しているためか、`bootloader_api`クレートに強く依存しているクレートである。
//! そのためブートローダを変更するか、あるいは他のフレームバッファを画面描画に利用する場合は大幅に書き直さなければならない

use core::fmt;

use super::color::Color;
use ab_glyph::{point, Font, FontRef, OutlinedGlyph};
use bootloader_api::info::FrameBufferInfo;
use common_lib::locked::Locked;

/// カーソルの初期座標。
/// 原因は不明だがY値の初期値を1にしないと上から一段目の文字と二段目の文字が重なる
const CURSOR_DEFAULT_POSITION: (usize, usize) = (0, 1);

pub struct TextBuffer<'a> {
    font_text: FontRef<'a>,
    font_bold: FontRef<'a>,
    scale: f32,
    frame_buffer: &'a Locked<&'static mut [u8]>,
    info: &'a FrameBufferInfo,

    /// 左上を(0,0)とした物理座標。cursor.0がx座標で、cursor.1がy座標
    cursor: (usize, usize),
}

impl<'a> TextBuffer<'a> {
    /// フレームバッファを初期化する
    pub const fn new(
        font_text: FontRef<'a>,
        font_bold: FontRef<'a>,
        scale: f32,
        frame_buffer: &'a Locked<&'static mut [u8]>,
        info: &'a FrameBufferInfo,
    ) -> Self {
        TextBuffer {
            font_text,
            font_bold,
            scale,
            info,
            frame_buffer,
            cursor: CURSOR_DEFAULT_POSITION,
        }
    }

    /// フレームバッファに文字を一文字出力するメソッド
    ///
    /// 出力できる文字は [PlemolJP Console Nerds](https://github.com/yuru7/PlemolJP) に収録されたもののうち、
    /// TextとBoldの二種類の太さのうちどれか一つに限る
    ///
    /// 画面左端にカーソルが達した場合は、`FrameBuffer::new_line(&mut self)`を呼び出して改行を行う
    ///
    /// ## LISENCE
    /// Copyright (c) 2021, Yuko OTAWARA. with Reserved Font Name "PlemolJP"
    ///
    /// This Font Software is licensed under the SIL Open Font License, Version 1.1.
    /// This license is copied below, and is also available with a FAQ at:
    /// https://scripts.sil.org/OFL
    pub fn put_char(&mut self, character: char, font_type: FontType, color: Color) {
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

                let glyph = self.get_glyph(character, font, self.scale);

                if let Some(g) = &glyph {
                    self.write_buffer(g, color);

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

    /// `FrameBuffer::put_char(&self)`の簡易版メソッド。
    ///
    /// **マクロ**`println!()`**を整備したら削除する予定なので、多用しないように**
    #[inline(always)]
    pub fn put_char_plain(&mut self, character: char) {
        self.put_char(character, FontType::Text, Color::new(255, 255, 255))
    }

    /// 改行を行うメソッド。
    /// カーソルが先頭以外の場合は行頭復帰も同時に行う
    ///
    /// ## TODO
    /// カーソルが画面下端に達した場合の処理を記述すること
    #[inline(always)]
    pub const fn new_line(&mut self) {
        if self.cursor.0 > 0 {
            self.carriage_return()
        }
        if self.cursor.1 >= self.height() {
            todo!()
        } else {
            self.cursor.1 += 1
        }
    }

    /// 行頭復帰を行うメソッド
    #[inline(always)]
    pub const fn carriage_return(&mut self) {
        self.cursor.0 = 0;
    }

    /// 画面の横幅を基準とした、1行当たりに収まる最大の文字数を表すメソッド
    #[inline(always)]
    pub const fn width(&self) -> usize {
        (self.info.stride as f32 / (self.scale / 2.0)) as usize
    }

    /// 画面に収まる最大の行数を表すメソッド
    #[inline(always)]
    pub const fn height(&self) -> usize {
        (self.info.height as f32 / self.scale) as usize
    }

    fn get_glyph(&self, character: char, font: &FontRef, scale: f32) -> Option<OutlinedGlyph> {
        // put_char(&mut self)で呼び出した際の文字幅を調節するため、0.9をself.scaleに掛けている
        let x = self.cursor.0 as f32 * (self.scale * 0.9 / 2.0);
        let y = self.cursor.1 as f32 * self.scale * 0.9;

        let glyph = font
            .glyph_id(character)
            .with_scale_and_position(scale, point(x, y));

        font.outline_glyph(glyph)
    }

    fn write_buffer(&self, glyph: &OutlinedGlyph, color: Color) {
        let min_x = glyph.px_bounds().min.x as u32;
        let min_y = glyph.px_bounds().min.y as u32;
        let stride = self.info.stride as u32;
        let bit_per_pixel = self.info.bytes_per_pixel as u32;
        let color = color.encode(self.info.pixel_format);

        // 描画
        glyph.draw(move |dx, dy, c| {
            let color = [
                color[0] as f32 * c,
                color[1] as f32 * c,
                color[2] as f32 * c,
            ];
            let color = [color[0] as u8, color[1] as u8, color[2] as u8];
            let x = min_x + dx;
            let y = min_y + dy;

            // 1pixelあたりのデータ量が三色+パディング分の4byte、またはグレースケールの1byteのみであると仮定（決め打ち）した処理
            // フレームバッファのフォーマットをbootloader_api::info::PixelFormat以外に変えた場合はまずこの部分を見直す事
            let buf_index = ((y * stride + x) * bit_per_pixel) as usize;
            let range = if bit_per_pixel == 4 {
                buf_index..buf_index + 3
            } else {
                buf_index..buf_index + 1
            };

            self.frame_buffer.lock()[range].copy_from_slice(&color);
        });
    }
}

impl<'a> fmt::Write for TextBuffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.put_char_plain(c);
        }

        Ok(())
    }
}

/// フォントの種類を定義する
pub enum FontType {
    /// 通常
    Text,

    /// 太字
    Bold,
}
