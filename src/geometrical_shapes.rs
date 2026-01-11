#![allow(dead_code)]

use rand::Rng;
use raster::Color;

pub trait Drawable {
    fn draw(&self, img: &mut impl Displayable);

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color::rgb(rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>())
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

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

pub struct Rectangle {
    p1: Point,
    p2: Point,
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(max_w: i32, max_h: i32) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(0..max_w),
            y: rng.random_range(0..max_h),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut impl Displayable) {
        img.display(self.x, self.y, self.color());
    }
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self { p1: *p1, p2: *p2 }
    }

    pub fn draw_with_color(&self, img: &mut impl Displayable, color: &Color) {
        let dx = (self.p2.x - self.p1.x).abs();
        let dy = (self.p2.y - self.p1.y).abs();
        let steps = dx.max(dy);

        for i in 0..=steps {
            let t = (i as f32) / (steps as f32);
            let x = self.p1.x + ((((self.p2.x - self.p1.x) as f32) * t) as i32);
            let y = self.p1.y + ((((self.p2.y - self.p1.y) as f32) * t) as i32);
            img.display(x, y, color.clone());
        }
    }

    pub fn random(w: i32, h: i32) -> Self {
        Self {
            p1: Point::random(w, h),
            p2: Point::random(w, h),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut impl Displayable) {
        let color = self.color();
        self.draw_with_color(img, &color);
    }
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut impl Displayable) {
        let color = self.color();
        Line::new(&self.p1, &self.p2).draw_with_color(img, &color);
        Line::new(&self.p2, &self.p3).draw_with_color(img, &color);
        Line::new(&self.p3, &self.p1).draw_with_color(img, &color);
    }
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self { p1: *p1, p2: *p2 }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut impl Displayable) {
        let p3 = Point::new(self.p1.x, self.p2.y);
        let p4 = Point::new(self.p2.x, self.p1.y);

        let color = self.color();
        Line::new(&self.p1, &p3).draw_with_color(img, &color);
        Line::new(&p3, &self.p2).draw_with_color(img, &color);
        Line::new(&self.p2, &p4).draw_with_color(img, &color);
        Line::new(&p4, &self.p1).draw_with_color(img, &color);
    }
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }

    pub fn random(w: i32, h: i32) -> Self {
        let center = Point::random(w, h);
        let edge = Point::random(w, h);

        let dx = edge.x - center.x;
        let dy = edge.y - center.y;
        let radius = ((dx * dx + dy * dy) as f32).sqrt() as i32;

        Self::new(&center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut impl Displayable) {
        use std::f32::consts::PI;

        let color = self.color();
        let steps = (2.0 * PI * (self.radius as f32)) as i32;

        for i in 0..steps {
            let theta = ((i as f32) / (steps as f32)) * 2.0 * PI;
            let x = self.center.x + (((self.radius as f32) * theta.cos()) as i32);
            let y = self.center.y + (((self.radius as f32) * theta.sin()) as i32);
            img.display(x, y, color.clone());
        }
    }
}