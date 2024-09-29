use rand::Rng;
use std::cmp::min;
use raster::{Color, Image};

// ============================ GLOBAL METHODS ====================
pub fn random_rgb_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    Color::rgb(r, g, b)
}

// ============================ TRAITS ============================
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        random_rgb_color()
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[allow(dead_code)]
impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

// ============================ STRUCT POINT ============================
#[derive(Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Point { x, y }
    }
}

#[allow(dead_code)]
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        // Color::rgb(0, 255, 0)
        random_rgb_color()
    }
}

// ============================ STRUCT LINE ============================
#[derive(Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let start = Point::random(width, height);
        let end = Point::random(width, height);
        Line { start, end }
    }
}

#[allow(dead_code)]
impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = self.start.x;
        let mut y = self.start.y;
        let color = self.color();

        loop {
            image.display(x, y, color.clone());
            if x == self.end.x && y == self.end.y { break; }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
        // random_rgb_color()
    }
}

// ============================ STRUCT RECTANGLE ============================
#[derive(Clone)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Rectangle {
            top_left: top_left.clone(),
            bottom_right: bottom_right.clone(),
        }
    }
}

#[allow(dead_code)]
impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        Line::new(&self.top_left, &top_right).draw(image);
        Line::new(&top_right, &self.bottom_right).draw(image);
        Line::new(&self.bottom_right, &bottom_left).draw(image);
        Line::new(&bottom_left, &self.top_left).draw(image);
    }

    fn color(&self) -> Color {
        // Color::rgb(0, 0, 255)
        random_rgb_color()
    }
}

// ============================ STRUCT TRIANGLE ============================
#[derive(Clone)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
        }
    }
}

#[allow(dead_code)]
impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(&self.a, &self.b).draw(image);
        Line::new(&self.b, &self.c).draw(image);
        Line::new(&self.c, &self.a).draw(image);
    }

    fn color(&self) -> Color {
        // Color::rgb(255, 255, 0)
        random_rgb_color()
    }
}

// ============================ STRUCT CIRCLE ============================
#[derive(Clone)]
pub struct Circle {
    center: Point,
    radius: i32,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = rand::thread_rng().gen_range(10..=min(width, height) / 2);
        Circle { center, radius }
    }
}

#[allow(dead_code)]
impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;
        let color = self.color();

        while x >= y {
            image.display(self.center.x + x, self.center.y + y, color.clone());
            image.display(self.center.x + y, self.center.y + x, color.clone());
            image.display(self.center.x - y, self.center.y + x, color.clone());
            image.display(self.center.x - x, self.center.y + y, color.clone());
            image.display(self.center.x - x, self.center.y - y, color.clone());
            image.display(self.center.x - y, self.center.y - x, color.clone());
            image.display(self.center.x + y, self.center.y - x, color.clone());
            image.display(self.center.x + x, self.center.y - y, color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> Color {
        // Color::rgb(255, 0, 0)
        random_rgb_color()
    }
}