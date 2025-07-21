use std::io::Read;

pub struct RotDecoder<R: Read> {
    pub(crate) input: R,
    pub(crate) rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.input.read(buf);
        for x in buf.iter_mut() {
            *x = if x.is_ascii_alphabetic() {
                if *x + self.rot > 'z' as u8 || *x + self.rot > 'Z' as u8 && *x <= 'Z' as u8 { *x - ('z' as u8 - 'a' as u8) + self.rot - 1 }
                else { *x + self.rot }
            } else { *x }
        }
        size
    }
}