use paint_object::*;
use paint_object::straight_line::*;
use macroquad::prelude::*;


#[macroquad::main("Picipaint")]
async fn main() {
    let mut objects = Vec::<Box<dyn PaintObject>>::new();
    let mut line_maker = StraightLineMaker::new(GREEN, 2.0);

    loop {
        clear_background(BLACK);

        if let Some(line) = line_maker.update_and_draw(Vec2::from(mouse_position())) {
            objects.push(Box::new(line));
            line_maker.reset();
        }

        for object in objects.iter() {
            object.draw();
        }

        next_frame().await;
    }
}


mod paint_object;
