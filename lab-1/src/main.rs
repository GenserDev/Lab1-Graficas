mod framebuffer;
mod polygon;

use polygon::{Color, Point, fill_polygon, draw_line};
use framebuffer::Framebuffer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    
    //Colores
    fb.set_background_color(Color::new(50, 50, 100));
    fb.clear();

    // Definir polígonos
    let polygon1 = [
        Point::new(165, 380), Point::new(185, 360), Point::new(180, 330), Point::new(207, 345),
        Point::new(233, 330), Point::new(230, 360), Point::new(250, 380), Point::new(220, 385),
        Point::new(205, 410), Point::new(193, 383)
    ];

    let polygon2 = [
        Point::new(321, 335), Point::new(288, 286), Point::new(339, 251), Point::new(374, 302)
    ];

    let polygon3 = [
        Point::new(377, 249), Point::new(411, 197), Point::new(436, 249)
    ];

    let polygon4 = [
        Point::new(413, 177), Point::new(448, 159), Point::new(502, 88), Point::new(553, 53),
        Point::new(535, 36), Point::new(676, 37), Point::new(660, 52), Point::new(750, 145),
        Point::new(761, 179), Point::new(672, 192), Point::new(659, 214), Point::new(615, 214),
        Point::new(632, 230), Point::new(580, 230), Point::new(597, 215), Point::new(552, 214),
        Point::new(517, 144), Point::new(466, 180)
    ];

    let polygon5 = [
        Point::new(682, 175), Point::new(708, 120), Point::new(735, 148), Point::new(739, 170)
    ];

    //Rellenar figuras
    fb.set_current_color(Color::new(255, 255, 0)); //Amarillo - Poligono 1
    fill_polygon(&mut fb, &polygon1);

    fb.set_current_color(Color::new(0, 0, 255)); //Azul - Poligono 2
    fill_polygon(&mut fb, &polygon2);

    fb.set_current_color(Color::new(255, 0, 0)); //Rojo - Poligono 3
    fill_polygon(&mut fb, &polygon3);

    fb.set_current_color(Color::new(0, 255, 0)); //Verde - Poligono 4
    fill_polygon(&mut fb, &polygon4);

    //Rellenar agujero de la figura 4
    fb.set_current_color(fb.background_color);
    fill_polygon(&mut fb, &polygon5);

    //Bordes blancos 
    fb.set_current_color(Color::new(255, 255, 255)); 
    draw_polygon_borders(&mut fb, &polygon1);
    draw_polygon_borders(&mut fb, &polygon2);
    draw_polygon_borders(&mut fb, &polygon3);
    draw_polygon_borders(&mut fb, &polygon4);
    draw_polygon_borders(&mut fb, &polygon5);

    //Guardar imagen
    fb.save("out.png")?;
    println!("Imagen guardada como output.png");

    Ok(())
}

fn draw_polygon_borders(fb: &mut Framebuffer, vertices: &[Point]) {
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        draw_line(fb, start, end);
    }
}