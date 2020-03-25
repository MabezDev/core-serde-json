


pub struct Slice<'a>{
    buf: &'a mut [u8],
    index: usize
}

impl<'a> Slice<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Slice { buf, index: 0 }
    }

    pub fn push(&mut self, b: u8) -> Result<(), ()>{
        if self.index >= self.buf.len() {
            return Err(())
        }

        self.buf[self.index] = b;
        self.index += 1;
        Ok(())
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) -> Result<(), ()> {
        if self.index + slice.len() >= self.buf.len() {
            return Err(());
        }
        self.buf[self.index..self.index + slice.len()].copy_from_slice(slice);
        self.index += slice.len();
        Ok(())
    }

    pub fn release(self) -> &'a mut [u8] {
        let (used, _unused) = self.buf.split_at_mut(self.index);
        
        used
    }
}

// use heapless::Vec;

// pub struct VecSlice<B> {
//     buf: Vec<u8, B>
// }

