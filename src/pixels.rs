pub struct PixelBuffer<'b> {
    pub height: u32,
    pub buf: &'b mut [u8],
}

impl<'b> PixelBuffer<'b> {
    pub fn new(height: u32, buf: &'b mut [u8]) -> PixelBuffer<'b> {
        Self { height, buf }
    }

    pub fn memset(&mut self, val: u8) {
        self.buf.fill(val);
    }
}
