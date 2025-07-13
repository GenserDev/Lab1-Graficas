use crate::framebuffer::Framebuffer;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub fn fill_polygon(fb: &mut Framebuffer, vertices: &[Point]) {
    if vertices.len() < 3 {
        return;
    }

    //Lmites en Y
    let min_y = vertices.iter().map(|p| p.y).min().unwrap();
    let max_y = vertices.iter().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        //Intersecciones
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let v1 = vertices[i];
            let v2 = vertices[j];

            if v1.y == v2.y {
                continue;
            }

            if (v1.y <= y && y < v2.y) || (v2.y <= y && y < v1.y) {
                let x = v1.x + ((y - v1.y) * (v2.x - v1.x)) / (v2.y - v1.y);
                intersections.push(x);
            }
        }
        intersections.sort();

        //Rellenar
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                
                for x in x_start..=x_end {
                    fb.set_pixel(x, y);
                }
            }
        }
    }
}

pub fn draw_line(fb: &mut Framebuffer, p1: Point, p2: Point) {
    let mut x0 = p1.x;
    let mut y0 = p1.y;
    let x1 = p2.x;
    let y1 = p2.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        fb.set_pixel(x0, y0);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}