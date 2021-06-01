#[derive(Debug, Clone, Default)]
pub struct Turtle {
    x: isize,
    y: isize,
}

impl Turtle {
    pub fn at(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn up(&mut self) {
        self.y -= 1;
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}
