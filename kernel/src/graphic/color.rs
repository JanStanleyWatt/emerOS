//! bootloader_api::info::PixelFormatに準拠した色表現の内、`PixelFormat::Rgb`と`PixelFormat::Bgr`の部分を定義するモジュール
//!
//! ## Panic
//! グレースケール(`bootloader_api::info::PixelFormat::U8`)のフレームバッファで`Color::encode(&self)`を呼び出すとパニックを起こす
use bootloader_api::info::PixelFormat;

/// bootloader_api::info::PixelFormatに準拠した色表現の内、`PixelFormat::Rgb`と`PixelFormat::Bgr`の部分を定義する
#[derive(Copy, Clone, Debug, Hash, Default)]
pub(crate) struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl Color {
    /// 構造体`Color`の新規作成
    #[inline(always)]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    /// Color->配列への変換を行うメソッド
    ///
    /// ## Panic
    /// グレースケール(`bootloader_api::info::PixelFormat::U8`)のフレームバッファでこのメソッドを呼び出すとパニックを起こす
    ///
    /// 2023年12月28日現在、グレースケールは未実装かつ実装すべきかどうかもまだわからないことから、`Color::encode(&self)`で
    /// "The format is not supported by this struct"というメッセージと共にパニックした場合は真っ先に上記を疑う事
    ///
    /// 言うまでもなく、`bootloader_api::info::PixelFormat`に定義されていないフォーマットの場合でもパニックを起こす
    #[inline]
    pub const fn encode(&self, pixel_format: PixelFormat) -> [u8; 3] {
        match pixel_format {
            PixelFormat::Rgb => [self.red, self.green, self.blue],
            PixelFormat::Bgr => [self.blue, self.green, self.red],
            PixelFormat::U8 => panic!("The format is not supported by this struct"),
            // Unknownなので決め打ち
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => [red_position, green_position, blue_position],
            _ => panic!("Unknown pixel format"),
        }
    }

    // #[inline]
    // pub const fn decode(pixel: [u8; 3], pixel_format: PixelFormat) -> Self {
    //     match pixel_format {
    //         PixelFormat::Rgb => Color::new(pixel[0], pixel[1], pixel[2]),
    //         PixelFormat::Bgr => Color::new(pixel[2], pixel[1], pixel[0]),
    //         PixelFormat::U8 => todo!(),
    //         PixelFormat::Unknown {
    //             red_position,
    //             green_position,
    //             blue_position,
    //         } => Color::new(red_position, green_position, blue_position),
    //         _ => panic!("Unknown pixel format"),
    //     }
    // }
}
