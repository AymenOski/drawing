// Traits
pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}


// Structures
#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    p1: Point,
    p2: Point,
}

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

pub struct Circle {
    center: Point,
    radius: i32,
}