use rand::Rng;
use raster::Color;

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

// Implementations

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(max_width: i32, max_height: i32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(0..max_width),
            y: rng.gen_range(0..max_height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        // Each shape must be drawn in a different color ; so I have chosen white for Point
        Color::hex("#2600ffff").unwrap()
    }
}