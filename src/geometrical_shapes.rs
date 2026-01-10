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
    p1: Point,
    p2: Point,
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
        let mut rng = rand::rng();

        Self {
            x: rng.random_range(0..max_width),
            y: rng.random_range(0..max_height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::hex("#2600ffff").unwrap()
    }
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line { p1: *p1, p2: *p2 }
    }

    pub fn random(max_width: i32, max_height: i32) -> Self {
        Self {
            p1: Point::random(max_width, max_height),
            p2: Point::random(max_width, max_height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut impl Displayable) {
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let steps = dx.max(dy);

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x1 + ((x2 - x1) as f32 * t).round() as i32;
            let y = y1 + ((y2 - y1) as f32 * t).round() as i32;
            image.display(x, y, self.color());
        }
    }

    fn color(&self) -> Color {
        Color::hex("#15ff00ff").unwrap()
    }
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }
    pub fn random(max_width: i32, max_height: i32) -> Self {
        Self {
            p1: Point::random(max_width, max_height),
            p2: Point::random(max_width, max_height),
            p3: Point::random(max_width, max_height),
        }
    }
    
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut impl Displayable) {

        Line::new(&self.p1, &self.p2).draw(image);
        Line::new(&self.p2, &self.p3).draw(image);
        Line::new(&self.p3, &self.p1).draw(image);
    }

    fn color(&self) -> Color {
        Color::hex("#ffffffff").unwrap()
    }
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle { p1: *p1, p2: *p2 }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut impl Displayable) {
        let p3 = Point::new(self.p1.x, self.p2.y);
        let p4 = Point::new(self.p2.x, self.p1.y);
        Line::new(&self.p1, &p3).draw(image);
        Line::new(&p3, &self.p2).draw(image);
        Line::new(&self.p2, &p4).draw(image);
        Line::new(&p4, &self.p1).draw(image);
    }

    fn color(&self) -> Color {
        Color::hex("#ff0000ff").unwrap()
    }
}