use bootloader_api::info::PixelFormat;

#[derive(Copy, Clone, Debug, Hash, Default)]
pub(crate) struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl Color {
    #[inline(always)]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    /// Color->配列
    #[inline]
    pub const fn encode(&self, pixel_format: PixelFormat) -> [u8; 3] {
        match pixel_format {
            PixelFormat::Rgb => [self.red, self.green, self.blue],
            PixelFormat::Bgr => [self.blue, self.green, self.red],
            PixelFormat::U8 => todo!(),
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
