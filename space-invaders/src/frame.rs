 
    pub struct Frame<'a>{
        frame: &'a mut [u8],
        width: u32,
        height: u32,
    }

    impl<'a> Frame<'a> {
        pub fn new(frame: &'a mut [u8], width: u32, height: u32) -> Self {
            Frame { frame, width, height}
        }

        pub fn put_pixel(&mut self, x: u32, y: u32, rgba: &[u8; 4]) {
            if x < self.width*4 && y < self.height*4 {
                let offset = (x+y*self.width*4) as usize;
                self.frame[offset..offset+4].copy_from_slice(rgba);
            }
        }
    }