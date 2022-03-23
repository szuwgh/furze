pub trait Clear {
    fn reset(&mut self);
}

pub type Bytes = Vec<u8>;

impl Clear for Bytes {
    fn reset(&mut self) {
        self.clear()
    }
}
