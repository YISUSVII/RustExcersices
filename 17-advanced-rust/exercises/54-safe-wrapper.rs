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
        let _ = (self, idx);
        unimplemented!("Implement get_unchecked using pointer offset or get_unchecked")
    }

    fn get(&self, idx: usize) -> Option<u8> {
        // TODO: bounds check then call get_unchecked in an unsafe block
        let _ = (self, idx);
        unimplemented!("Implement get")
    }
}

fn main() {
    let buf = SafeBuf::new(vec![10, 20, 30]);
    println!("{:?}", buf.get(1));
    println!("{:?}", buf.get(9));
}
