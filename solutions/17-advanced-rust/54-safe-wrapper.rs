struct SafeBuf {
    data: Vec<u8>,
}

impl SafeBuf {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// # Safety
    /// `idx` must be < data length.
    unsafe fn get_unchecked(&self, idx: usize) -> u8 {
        *self.data.get_unchecked(idx)
    }

    fn get(&self, idx: usize) -> Option<u8> {
        if idx < self.data.len() {
            // SAFETY: idx checked against length above.
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }
}

fn main() {
    let buf = SafeBuf::new(vec![10, 20, 30]);
    println!("{:?}", buf.get(1));
    println!("{:?}", buf.get(9));
}
