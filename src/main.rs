use macroquad::prelude::*;


#[macroquad::main("Picipaint")]
async fn main() {
    let mut position = Vec2::new(10.0, 20.0);
    let mut velocity = Vec2::new(150.0, 100.0);
    let radius = 10.0;

    loop {
        clear_background(BLACK);

        let new_position = position + velocity * get_frame_time();
        let mut bumped_into_wall = false;
        if new_position.x - radius < 0.0 || screen_width() < new_position.x + radius {
            velocity.x *= -1.0;
            bumped_into_wall = true;
        }
        if new_position.y - radius < 0.0 || screen_height() < new_position.y + radius {
            velocity.y *= -1.0;
            bumped_into_wall = true;
        }
        if !bumped_into_wall {
            position = new_position;
        }

        draw_circle(position.x, position.y, radius, GREEN);

        next_frame().await;
    }
}
