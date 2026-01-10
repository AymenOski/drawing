use rand::Rng;
use raster::Color;

pub trait Drawable {
    fn draw(&self, img: &mut impl Displayable);

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color::rgb(rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>())
    }

    fn draw_with_color(&self, img: &mut impl Displayable, _color: &Color) {
        self.draw(img);
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

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
        let color = self.color();

        let steps = dx.max(dy);

        for i in 0..=steps {
            let t = (i as f32) / (steps as f32);
            let x = x1 + ((((x2 - x1) as f32) * t).round() as i32);
            let y = y1 + ((((y2 - y1) as f32) * t).round() as i32);
            image.display(x, y, color.clone());
        }
    }
    fn draw_with_color(&self, img: &mut impl Displayable, color: &Color) {
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let steps = dx.max(dy);

        for i in 0..=steps {
            let t = (i as f32) / (steps as f32);
            let x = x1 + ((((x2 - x1) as f32) * t).round() as i32);
            let y = y1 + ((((y2 - y1) as f32) * t).round() as i32);
            img.display(x, y, color.clone());
        }
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
        let color = self.color();
        Line::new(&self.p1, &self.p2).draw_with_color(image, &color);
        Line::new(&self.p2, &self.p3).draw_with_color(image, &color);
        Line::new(&self.p3, &self.p1).draw_with_color(image, &color);
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
        let color = self.color();

        Line::new(&self.p1, &p3).draw_with_color(image, &color);
        Line::new(&p3, &self.p2).draw_with_color(image, &color);
        Line::new(&self.p2, &p4).draw_with_color(image, &color);
        Line::new(&p4, &self.p1).draw_with_color(image, &color);
    }
}
impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let edge = Point::random(width, height);
        let radius = (
            ((edge.x - center.x).pow(2) + (edge.y - center.y).pow(2)) as f32
        ).sqrt() as i32;
        Self::new(center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut impl Displayable) {
        use std::f32::consts::PI;

        let color = self.color();
        let cx = self.center.x as f32;
        let cy = self.center.y as f32;
        let r = self.radius as f32;

        let mut steps = (2.0 * PI * r).round() as i32;
        if steps <= 0 {
            steps = 1;
        }

        for i in 0..=steps {
            let theta = ((i as f32) * 2.0 * PI) / (steps as f32);
            let x = cx + r * theta.cos();
            let y = cy + r * theta.sin();

            image.display(x.round() as i32, y.round() as i32, color.clone());
        }
    }
}
