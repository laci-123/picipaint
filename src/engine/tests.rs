use core::f32;
use approx::assert_relative_eq;
use super::*;


const STROKE: Stroke = Stroke { color: Color{ red: 255, green: 255, blue: 255, alpha: 255 }, thickness: 1.0 };
const BG_COLOR: Color = Color{ red: 0, green: 0, blue: 0, alpha: 255 };


#[derive(Default)]
struct MockPainter {
    centers: Vec<Vector2>,
    radii: Vec<f32>,
}

impl ScreenPainter for MockPainter {
    fn draw_circle(&mut self, center: Vector2, radius: f32, _stroke: Stroke) {
        self.centers.push(center);
        self.radii.push(radius);
    }

    fn draw_line(&mut self, _start: Vector2, _end: Vector2, _stroke: Stroke) {}
    fn draw_rectangle(&mut self, _rectangle: Rectangle, _stroke: Stroke) {}
    fn draw_rectangle_filled(&mut self, _rectangle: Rectangle, _color: Color, _stroke: Option<Stroke>) {}
}


struct MockCircle {
    center: Vector2,
    radius: f32,
    under_mouse: bool,
    selected: bool,
}

impl PaintObject<MockPainter> for MockCircle {
    fn update(&mut self, _input: &UserInput, _camera: &Camera) {}
    
    fn draw(&self, painter: &mut WorldPainter<MockPainter>, camera: &Camera) {
        painter.draw_circle(self.center, self.radius, STROKE, camera);
    }
    
    fn is_selected(&self) -> bool {
        self.selected
    }
    
    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    
    fn is_under_mouse(&self) -> bool {
        self.under_mouse
    }
    
    fn get_bounding_rect(&self) -> Rectangle {
        todo!()
    }
}


#[test]
fn zooming_centered_around_camera_position() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockPainter>::new(tools, view_width, view_height);

    // Place some objects equally spaced along a circle around some arbitrary point (`middle`).
    let middle = Vector2 { x: 100.0, y: 50.0 };
    let objects_count = 10;
    let angle_step = 2.0 * f32::consts::PI / (objects_count as f32);
    let big_r = 50.0;
    let little_r = 1.0;
    for i in 0..objects_count {
        let angle = (i as f32) * angle_step;
        engine.objects.push(Box::new(MockCircle {
            center: Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            },
            radius: little_r,
            selected: false,
            under_mouse: false
        }));
    }

    // Using a camera centered at `middle` and with zoom 1.0,
    // all objects are drawn at equal distance from the origin in screen-space,
    // the same distance as their distance from `middle` in world-space.
    // Their radius is also the same as in world-space.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 1.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for center in painter.centers {
        assert_relative_eq!(center.length(), big_r);
    }
    for radius in painter.radii {
        debug_assert_eq!(radius, little_r);
    }

    // With zoom == 2.0, they are all twice as far away, and their radius are also twice as big.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 2.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for center in painter.centers {
        assert_relative_eq!(center.length(), 2.0 * big_r);
    }
    for radius in painter.radii {
        debug_assert_eq!(radius, 2.0 * little_r);
    }

    // With zoom == 0.5, they are all half as far away, and their radius are also half as big.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 0.5 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for center in painter.centers {
        assert_relative_eq!(center.length(), 0.5 * big_r);
    }
    for radius in painter.radii {
        assert_eq!(radius, 0.5 * little_r);
    }
}


#[test]
fn zooming_not_centered_around_camera_position() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockPainter>::new(tools, view_width, view_height);

    // Place some objects equally spaced along a circle around some arbitrary point (`middle`).
    let middle = Vector2 { x: 100.0, y: 50.0 };
    let objects_count = 10;
    let angle_step = 2.0 * f32::consts::PI / (objects_count as f32);
    let big_r = 50.0;
    let little_r = 1.0;
    for i in 0..objects_count {
        let angle = (i as f32) * angle_step;
        engine.objects.push(Box::new(MockCircle {
            center: Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            },
            radius: little_r,
            selected: false,
            under_mouse: false
        }));
    }

    let camera_from_middle = Vector2 { x: 100.0, y: 200.0 };
    let mut objects_distance_at_no_zoom = Vec::new();

    // Using a camera *not* centered at `middle` and with zoom 1.0,
    // their radius is the same as in world-space.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 1.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for radius in painter.radii {
        assert_eq!(radius, little_r);
    }
    for center in painter.centers {
        objects_distance_at_no_zoom.push(center.length());
    }

    // With zoom == 2.0, each object is twice as far away from the origin than with zoom == 1.0,
    // and its radius are also twice as big.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 2.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for radius in painter.radii {
        assert_eq!(radius, 2.0 * little_r);
    }
    for (center, distance_at_no_zoom) in painter.centers.iter().zip(objects_distance_at_no_zoom.iter()) {
        assert_relative_eq!(center.length(), 2.0 * distance_at_no_zoom);
    }

    // With zoom == 0.5, each object is half as far away from the origin than with zoom == 1.0,
    // and its radius are also half as big.
    let mut painter = MockPainter::default();
    engine.camera = Camera{ position: middle, zoom: 0.5 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    for radius in painter.radii {
        assert_eq!(radius, 0.5 * little_r);
    }
    for (center, distance_at_no_zoom) in painter.centers.iter().zip(objects_distance_at_no_zoom.iter()) {
        assert_relative_eq!(center.length(), 0.5 * distance_at_no_zoom);
    }
}
