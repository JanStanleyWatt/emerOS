use bootloader_api::info::PixelFormat;

pub(super) const fn encode(red_green_blue: [u8; 3], pixel_format: PixelFormat) -> [u8; 3] {
    let red = red_green_blue[0];
    let green = red_green_blue[1];
    let blue = red_green_blue[2];

    match pixel_format {
        PixelFormat::Rgb => red_green_blue,
        PixelFormat::Bgr => [blue, green, red],
        PixelFormat::U8 => [(red + green + blue) / 3, 0, 0],
        PixelFormat::Unknown {
            red_position,
            green_position,
            blue_position,
        } => [red_position, green_position, blue_position],
        _ => panic!("Unknown pixel format"),
    }
}
