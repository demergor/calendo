struct Terminal {
    width: u16,
    height: u16,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        todo!("Implement raw terminal destructor");
    }
}
