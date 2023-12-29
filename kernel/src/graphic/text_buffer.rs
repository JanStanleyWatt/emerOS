//! 画面にテキストを描画するモジュール
//!
//! I/Oに強く関与しているためか、`bootloader_api`クレートに強く依存しているクレートである。
//! そのためブートローダを変更するか、あるいは他のフレームバッファを画面描画に利用する場合は大幅に書き直さなければならない。
//! 出力できる文字は [PlemolJP Console Nerds](https://github.com/yuru7/PlemolJP) に収録されたもののうち、
//!  TextとBoldの二種類の太さのうちどれか一つに限る
//!
//! 画面左端にカーソルが達した場合は、`FrameBuffer::new_line(&mut self)`を呼び出して改行を行う
//!
//! ## LISENCE
//! Copyright (c) 2021, Yuko OTAWARA. with Reserved Font Name "PlemolJP"
//!
//! This Font Software is licensed under the SIL Open Font License, Version 1.1.
//! This license is copied below, and is also available with a FAQ at:
//! https://scripts.sil.org/OFL

use ab_glyph::{point, Font, FontRef, OutlinedGlyph};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use common_lib::locked::Locked;

/// カーソルの初期座標。
/// 原因は不明だがY値の初期値を1にしないと上から一段目の文字と二段目の文字が重なる
const CURSOR_DEFAULT_POSITION: (usize, usize) = (0, 1);

pub struct TextBuffer<'a> {
    /// 通常フォント
    pub(super) font_text: FontRef<'a>,
    /// 太字フォント
    pub(super) font_bold: FontRef<'a>,
    /// フォントの大きさ
    pub(super) scale: f32,
    /// フレームバッファの情報
    pub(super) info: &'a FrameBufferInfo,
    /// 左上を(0,0)とした物理座標。cursor.0がx座標で、cursor.1がy座標
    pub(super) cursor: (usize, usize),

    frame_buffer: &'a Locked<&'static mut [u8]>,
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

    /// 入力文字とフォントファイルからフォント情報を抽出するメソッド
    pub(super) fn get_glyph(&self, character: char, font: &FontRef) -> Option<OutlinedGlyph> {
        // put_char(&mut self)で呼び出した際の文字幅を調節するため、0.9をself.scaleに掛けている
        let x = self.cursor.0 as f32 * (self.scale * 0.9 / 2.0);
        let y = self.cursor.1 as f32 * self.scale * 0.9;

        let glyph = font
            .glyph_id(character)
            .with_scale_and_position(self.scale, point(x, y));

        font.outline_glyph(glyph)
    }

    pub(super) fn write_buffer(&self, glyph: &OutlinedGlyph, r_g_b: [u8; 3]) {
        let min_x = glyph.px_bounds().min.x as u32;
        let min_y = glyph.px_bounds().min.y as u32;
        let stride = self.info.stride as u32;
        let bit_per_pixel = self.info.bytes_per_pixel as u32;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [r_g_b[0], r_g_b[1], r_g_b[2]],
            PixelFormat::Bgr => [r_g_b[2], r_g_b[1], r_g_b[0]],
            PixelFormat::U8 => panic!("The format is not supported by this struct"),
            // Unknownなので決め打ち
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => [red_position, green_position, blue_position],
            _ => panic!("Unknown pixel format"),
        };

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
