extern crate piston_window;
extern crate image as im;

mod raycasting;

use piston_window::*;
use raycasting::*;

fn main() {
    let mut position = Point::new(100, 100);
    //let mut direction = Point::new(-1, 0);
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;
    
    let opengl = OpenGL::V3_2;
    let (width, height) = (WINDOW_SIZE*MAP_ATOM, MAP_ATOM*WINDOW_SIZE);
    let mut window: PistonWindow =
        WindowSettings::new("rajkastnicz", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &canvas,
            &TextureSettings::new()
        ).unwrap();
    
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        };
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Up {position.move_up()}
            if key == Key::Left {position.move_left()}
            if key == Key::Down {position.move_down()}
            if key == Key::Right {position.move_right()}

            put_pixel(&mut canvas, &position);
        };
    }
}

fn put_pixel(canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>, position: &Point){
    for i in 0..MAP_ATOM {
        for j in 0..MAP_ATOM {
            canvas.put_pixel(position.x + i, position.y + j, im::Rgba([100, 0, 0, 255]));
        }
    }
}