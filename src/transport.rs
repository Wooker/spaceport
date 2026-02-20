pub trait Transport {
    type Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

