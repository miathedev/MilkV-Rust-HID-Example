use framebuffer::Framebuffer;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

pub struct FrameBufferDisplay {
    framebuffer: Vec<u8>,
    iface: Framebuffer,
}

impl FrameBufferDisplay {
    // Send buffer to the display
    pub fn flush(&mut self) -> Result<(), ()> {
        //Copy date over to the actual framebuffer implemented in miasbuffer
        self.iface.write_frame(&self.framebuffer);
        Ok(())
    }
}

impl DrawTarget for FrameBufferDisplay {
    //type Color = Rgb888;
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    // Map draw onto the frame buffer
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let xres = self.iface.var_screen_info.xres;
        let yres = self.iface.var_screen_info.yres;

        println!("xres: {:?}", xres);
        println!("yres: {:?}", xres);
        println!(
            "bits_per_pixel: {:?}",
            self.iface.var_screen_info.bits_per_pixel
        );
        //###################
        //RGB888 color
        /*
        let bytespp = self.iface.var_screen_info.bits_per_pixel / 8;

        for Pixel(coord, color) in pixels.into_iter() {
            let x: i32 = coord.x.try_into().unwrap();
            let y: i32 = coord.y.try_into().unwrap();
            if 0 <= x && x < xres as i32 && 0 <= y && y < yres as i32 {
                let index: u32 = (x as u32 + y as u32 * xres) * bytespp;
                self.framebuffer[index as usize] = color.b();
                self.framebuffer[index as usize + 1] = color.g();
                self.framebuffer[index as usize + 2] = color.r();
            }
        }*/

        //##################
        //RGB565 color
        let bytespp = self.iface.var_screen_info.bits_per_pixel / 8; // Should be 2 for RGB565

        for Pixel(coord, color) in pixels.into_iter() {
            let x: i32 = coord.x.try_into().unwrap();
            let y: i32 = coord.y.try_into().unwrap();
            if 0 <= x && x < xres as i32 && 0 <= y && y < yres as i32 {
                let index: u32 = (x as u32 + y as u32 * xres) * bytespp;

                let r5: u16 = (color.b() as u16>> 3) & 0b11111; // Take top 5 bits of red
                let g6: u16 = (color.r() as u16 >> 2) & 0b111111; // Take top 6 bits of green
                let b5: u16 = (color.g() as u16 >> 3) & 0b11111; // Take top 5 bits of blue

                let rgb565: u16 = (r5 << 11) | (g6 << 5) | b5;

                // Convert the 16-bit integer to two bytes and store them in the framebuffer
                self.framebuffer[index as usize] = ((rgb565 >> 8) & 0xFF) as u8;
                self.framebuffer[index as usize + 1] = (rgb565 & 0xFF) as u8;
            }
        }
        Ok(())
    }
}

impl OriginDimensions for FrameBufferDisplay {
    fn size(&self) -> Size {
        Size::new(
            self.iface.var_screen_info.xres,
            self.iface.var_screen_info.yres,
        )
    }
}

impl FrameBufferDisplay {
    pub fn new() -> FrameBufferDisplay {
        let framebuffer = Framebuffer::new("/dev/fb0").unwrap();
        let h = framebuffer.var_screen_info.yres;
        let line_length = framebuffer.fix_screen_info.line_length;
        println!("h: {:?}", h);
        println!("line_length: {:?}", line_length);
        FrameBufferDisplay {
            framebuffer: vec![0u8; (line_length * h) as usize],
            iface: Framebuffer::new("/dev/fb0").unwrap(),
        }
    }
}