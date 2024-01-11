// use core::ops::Range;

// use alloc::vec::Vec;

/// コンソール機能を定義するトレイト
pub trait Console {
    /// フレームバッファに文字を一文字出力するメソッド
    ///
    /// ## Panic
    /// `bootloader_api::info::PixelFormat`に定義されていないフォーマットの場合はパニックを起こす
    fn put_char(&mut self, character: char, font_type: FontType, red_green_blue: [u8; 3]);

    /// 改行を行うメソッド。
    /// カーソルが先頭以外の場合は行頭復帰も同時に行う
    fn new_line(&mut self);

    /// 行頭復帰を行うメソッド
    fn carriage_return(&mut self);

    /// 画面表示をすべて消し、カーソルを初期位置に戻すメソッド
    fn reset(&mut self);
}

/// フォントの種類を定義する
#[derive(Debug, Default, Clone, Copy, Hash)]
pub enum FontType {
    /// 通常
    #[default]
    Text,

    /// 太字
    Bold,
}
