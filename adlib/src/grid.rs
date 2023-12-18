use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    Right,
    Up,
    Left,
    Down,
}

pub use Dir::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl Dir {
    pub fn reverse(&self) -> Dir {
        match self {
            Right => Left,
            Up => Down,
            Left => Right,
            Down => Up,
        }
    }

    pub fn turn_left(&self) -> Dir {
        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }

    pub fn turn_right(&self) -> Dir {
        match self {
            Right => Down,
            Up => Right,
            Left => Up,
            Down => Left,
        }
    }

    pub fn dr(&self) -> isize {
        match self {
            Right => 0,
            Up => -1,
            Left => 0,
            Down => 1,
        }
    }

    pub fn dc(&self) -> isize {
        match self {
            Right => 1,
            Up => 0,
            Left => -1,
            Down => 0,
        }
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, dir: Dir) -> Point {
        Point {
            row: (self.row as isize + dir.dr()) as usize,
            col: (self.col as isize + dir.dc()) as usize,
        }
    }
}

impl AddAssign<Dir> for Point {
    fn add_assign(&mut self, dir: Dir) {
        *self = *self + dir;
    }
}

impl<T> Grid<T> {
    pub fn num_rows(&self) -> usize { self.data.len() }

    pub fn num_cols(&self) -> usize { self.data[0].len() }

    pub fn has(&self, p: Point) -> bool {
        p.row < self.num_rows() && p.col < self.num_cols()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &T {
        &self.data[p.row][p.col]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        &mut self.data[p.row][p.col]
    }
}
