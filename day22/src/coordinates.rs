pub struct Xyz(i32, i32, i32);

impl Xyz {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn z(&self) -> i32 {
        self.2
    }
}
