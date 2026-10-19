#[derive(PartialEq)]
pub struct Square(u8);

impl Square {
    pub const fn index(&self) -> u8 {
        self.0
    }

    pub const fn rank(&self) -> u8 {
        self.0.div_euclid(8) + 1
    }

    pub const fn file(&self) -> u8 {
        self.0 % 8
    }

    pub const fn new(index: u8) -> Self {
        assert!(index < 64);
        Square(index)
    }
}
