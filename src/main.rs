use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{
        PrimitiveStyle, Rectangle,
    },
};

mod frame_buffer_display;
use frame_buffer_display::FrameBufferDisplay;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display = FrameBufferDisplay::new();

    // Create styles used by the drawing operations
    let fill_white = PrimitiveStyle::with_fill(Rgb565::WHITE);
    let fill_black = PrimitiveStyle::with_fill(Rgb565::BLACK);

    let fill_red = PrimitiveStyle::with_fill(Rgb565::RED);
    let fill_green = PrimitiveStyle::with_fill(Rgb565::GREEN);
    let fill_blue = PrimitiveStyle::with_fill(Rgb565::BLUE);

    let yoffset = 16;

    // Draw a filled square
    Rectangle::new(Point::new(0 * 16, yoffset), Size::new(16, 16))
        .into_styled(fill_white)
        .draw(&mut display)?;

    Rectangle::new(Point::new(1 * 16, yoffset), Size::new(16, 16))
        .into_styled(fill_black)
        .draw(&mut display)?;

    Rectangle::new(Point::new(2 * 16, 2*yoffset), Size::new(16, 16))
        .into_styled(fill_red)
        .draw(&mut display)?;

    Rectangle::new(Point::new(3 * 16, 3*yoffset), Size::new(16, 16))
        .into_styled(fill_green)
        .draw(&mut display)?;

    Rectangle::new(Point::new(4 * 16, 4*yoffset), Size::new(16, 16))
        .into_styled(fill_blue)
        .draw(&mut display)?;
    
    let _ = display.flush();
    Ok(())
}
