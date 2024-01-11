//! フレームバッファにテキストを描画するモジュール
//!
//! I/Oに強く関与しているためか、`bootloader_api`クレートに強く依存しているクレートである。
//! そのためブートローダを変更するか、あるいは他のフレームバッファを画面描画に利用する場合は大幅に書き直さなければならない

use ab_glyph::{point, Font, FontRef, OutlinedGlyph};
use alloc::vec;
use alloc::vec::Vec;
use bootloader_api::info::FrameBufferInfo;

use super::color;
use crate::FRAME_BUFFER;

/// カーソルの初期座標
pub(super) const CURSOR_DEFAULT_POSITION: (usize, usize) = (0, 1);

pub struct TextBuffer<'a> {
    pub(super) font_text: FontRef<'a>,
    pub(super) font_bold: FontRef<'a>,
    pub(super) scale: f32,

    text_buffer: Vec<u8>,

    /// 左上を(0,0)とした物理座標。cursor.0がx座標で、cursor.1がy座標
    pub(super) cursor: (usize, usize),
}

impl<'a> TextBuffer<'a> {
    /// フレームバッファを初期化する
    pub const fn new(font_text: FontRef<'a>, font_bold: FontRef<'a>, scale: f32) -> Self {
        TextBuffer {
            font_text,
            font_bold,
            scale,
            text_buffer: Vec::new(),
            cursor: CURSOR_DEFAULT_POSITION,
        }
    }

    #[cold]
    fn init_textbuffer(&mut self, byte_len: usize) {
        self.text_buffer = vec![0; byte_len];
    }

    pub(super) fn get_glyph(&self, character: char, font: &FontRef) -> Option<OutlinedGlyph> {
        // put_char(&mut self)で呼び出した際の文字幅を調節するため、0.9をself.scaleに掛けている
        let m = self.scale * 0.9;
        let x = self.cursor.0 as f32 * (m / 2.0);
        let y = self.cursor.1 as f32 * m;

        let glyph = font
            .glyph_id(character)
            .with_scale_and_position(self.scale, point(x, y));

        font.outline_glyph(glyph)
    }

    pub(super) fn write_buffer(
        &mut self,
        glyph: &OutlinedGlyph,
        red_green_blue: [u8; 3],
        info: &FrameBufferInfo,
    ) {
        if self.text_buffer.len() != info.byte_len {
            self.init_textbuffer(info.byte_len);
        }

        let min_x = glyph.px_bounds().min.x as u32;
        let min_y = glyph.px_bounds().min.y as u32;
        let stride = info.stride as u32;
        let bytes_per_pixel = info.bytes_per_pixel as u32;
        let color = color::encode(red_green_blue, info.pixel_format);

        // 描画
        glyph.draw(move |dx, dy, c| {
            let color = color.map(|n| (n as f32 * c) as u8);
            let x = min_x + dx;
            let y = min_y + dy;

            // 1pixelあたりのデータ量が三色+パディング分の4byte、またはグレースケールの1byteのみであると仮定（決め打ち）した処理
            // フレームバッファのフォーマットをbootloader_api::info::PixelFormat以外に変えた場合はまずこの部分を見直す事
            let buf_index = ((y * stride + x) * bytes_per_pixel) as usize;
            let range = if bytes_per_pixel == 4 {
                buf_index..buf_index + 3
            } else {
                buf_index..buf_index + 1
            };

            // self.frame_buffer.lock()[range].copy_from_slice(&color);
            self.text_buffer[range].copy_from_slice(&color);
        });
    }

    #[inline(always)]
    pub(super) fn merge_buffer(&mut self) {
        FRAME_BUFFER
            .get()
            .unwrap()
            .lock()
            .copy_from_slice(&self.text_buffer);
    }

    pub(super) fn clear(&mut self) {
        FRAME_BUFFER.get().unwrap().lock().fill(0);
    }
}
