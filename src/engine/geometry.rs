use super::{Coordinate, Matrix};

pub trait GridIncrement: Sized {
    type Width: Into<usize>;
    const WIDTH: Self::Width;

    fn grid_incd(mut self) -> Self {
        self.grid_inc();
        self
    }
    fn grid_inc(&mut self);
}

impl GridIncrement for Coordinate {
    type Width = usize;
    const WIDTH: Self::Width = Matrix::WIDTH;

    fn grid_inc(&mut self) {
        self.x += 1;
        if self.x == Self::WIDTH {
            self.x = 0;
            self.y += 1;
        }
    }
}
