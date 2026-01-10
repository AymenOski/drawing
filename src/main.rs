mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};

use raster::{Color, Image};
use rand::Rng;


fn main() {
    let mut image = Image::blank(1000, 1000);  

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

     let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
     rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    let mut rng = rand::rng();

for _ in 0..15 {
    let shape_type = rng.random_range(1..=2);

    match shape_type {
        0 => {
            let p = gs::Point::random(image.width, image.height);
            p.draw(&mut image);
        }
        1 => {
            let l = gs::Line::random(image.width, image.height);
            l.draw(&mut image);
        }
        2 => {
            let c = gs::Triangle::random(image.width, image.height);
            c.draw(&mut image);
        }
        _ => {}
    }
}

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}