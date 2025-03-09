use core::f32;
use approx::{assert_relative_eq, relative_eq};
use mockall::{predicate, Sequence};
use super::*;


const STROKE: Stroke = Stroke { color: Color{ red: 255, green: 255, blue: 255, alpha: 255 }, thickness: 1.0 };
const BG_COLOR: Color = Color{ red: 0, green: 0, blue: 0, alpha: 255 };


struct FakePaintObject {
    under_mouse: bool,
    selected: bool,
    bounding_rect: Rectangle,
    p1: Option<Vector2>,
    p2: Option<Vector2>,
}

impl Default for FakePaintObject {
    fn default() -> Self {
        Self {
            under_mouse: false,
            selected: false,
            bounding_rect: Rectangle { p1: Vector2::zero(), p2: Vector2::zero() },
            p1: None,
            p2: None,
        }
    }
}

impl PaintObject<MockScreenPainter> for FakePaintObject {
    fn update(&mut self, input: &UserInput) {
        match input {
            UserInput::MouseClick { position, .. } => {
                self.under_mouse = self.bounding_rect.contains_point(*position);
            },
            UserInput::MouseMove { position, .. } => {
                self.under_mouse = self.bounding_rect.contains_point(*position);
            },
            _ => {/* do nothing */},
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, MockScreenPainter>, camera: &Camera) {
        if let Some(p1) = self.p1 {
            if let Some(p2) = self.p2 {
                painter.draw_line(p1, p2, STROKE, camera);
            }
            else {
            painter.draw_circle(p1, 1.0, STROKE, camera);
            }
        }
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
        self.bounding_rect
    }
}


#[test]
fn object_draw_order() {
    // First the background color is drawn then
    // the methods of PaintObjects are called in the same order as the objects were created.

    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut seq = Sequence::new();

    let object1 = FakePaintObject { p1: Some(Vector2::zero()), ..Default::default() };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject { p1: Some(Vector2::zero()), p2: Some(Vector2::zero()), ..Default::default() };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().once().in_sequence(&mut seq).return_const(()); // background color
    painter.expect_draw_circle().once().in_sequence(&mut seq).return_const(());           // object1
    painter.expect_draw_line().once().in_sequence(&mut seq).return_const(());             // object2

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn tools_iterator_empty() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut tools_iter = engine.tools_iter();
    assert_eq!(tools_iter.next(), None);
}

#[test]
fn tools_iterator() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut tool1 = MockTool::new();
    tool1.expect_display_name().once().return_const(String::from("tool1"));
    engine.tools.push(Box::new(tool1));

    let mut tool2 = MockTool::new();
    tool2.expect_display_name().once().return_const(String::from("tool2"));
    engine.tools.push(Box::new(tool2));

    let mut tools_iter = engine.tools_iter();
    assert_eq!(tools_iter.next(), Some(String::from("tool1")));
    assert_eq!(tools_iter.next(), Some(String::from("tool2")));
    assert_eq!(tools_iter.next(), None);
}

#[test]
fn nothing_is_drawn_if_no_tool_is_selected() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // draw background color

    let mut tool1 = MockTool::new();
    tool1.expect_update().never().return_const(());
    tool1.expect_draw().never().return_const(());
    engine.tools.push(Box::new(tool1));

    let mut tool2 = MockTool::new();
    tool2.expect_update().never().return_const(());
    tool2.expect_draw().never().return_const(());
    engine.tools.push(Box::new(tool2));

    engine.select_tool(None);

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn only_the_selected_tool_is_used() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // draw background color

    let mut seq = Sequence::new();

    let mut tool1 = MockTool::new();
    tool1.expect_update().once().in_sequence(&mut seq).return_const(());
    tool1.expect_draw().once().in_sequence(&mut seq).return_const(());
    engine.tools.push(Box::new(tool1));

    let mut tool2 = MockTool::new();
    tool2.expect_update().once().in_sequence(&mut seq).return_const(());
    tool2.expect_draw().once().in_sequence(&mut seq).return_const(());
    engine.tools.push(Box::new(tool2));

    engine.select_tool(Some(0));
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.select_tool(Some(1));
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn zooming_centered_around_camera_position() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // Place some circles equally spaced along a circle around some arbitrary point (`middle`).
    let middle = Vector2 { x: 100.0, y: 50.0 };
    let objects_count = 10;
    let angle_step = 2.0 * f32::consts::PI / (objects_count as f32);
    let big_r = 50.0;
    let little_r = 1.0;
    for i in 0..objects_count {
        let angle = (i as f32) * angle_step;
        let object = FakePaintObject {
            p1: Some(Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            }),
            ..Default::default()
        };
        engine.objects.push(Box::new(object));
    }

    // Using a camera centered at `middle` and with zoom 1.0,
    // all objects are drawn at equal distance from the origin in screen-space,
    // the same distance as their distance from `middle` in world-space.
    // Their radius is also the same as in world-space.
    let mut painter = MockScreenPainter::default();
    painter.expect_draw_rectangle_filled().return_const(()); // drawing the background color
    painter.expect_draw_circle().with(predicate::function(move |center: &Vector2| relative_eq!(center.length(), big_r)),
                                      predicate::eq(little_r),
                                      predicate::always())
                                .times(objects_count)
                                .return_const(());
    engine.camera = Camera{ position: middle, zoom: 1.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    // With zoom == 2.0, they are all twice as far away, and their radius are also twice as big.
    let mut painter = MockScreenPainter::default();
    painter.expect_draw_rectangle_filled().return_const(()); // drawing the background color
    painter.expect_draw_circle().with(predicate::function(move |center: &Vector2| relative_eq!(center.length(), 2.0 * big_r)),
                                      predicate::eq(2.0 * little_r),
                                      predicate::always())
                                .times(objects_count)
                                .return_const(());
    engine.camera = Camera{ position: middle, zoom: 2.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    // With zoom == 0.5, they are all half as far away, and their radius are also half as big.
    let mut painter = MockScreenPainter::default();
    painter.expect_draw_rectangle_filled().return_const(()); // drawing the background color
    painter.expect_draw_circle().with(predicate::function(move |center: &Vector2| relative_eq!(center.length(), 0.5 * big_r)),
                                      predicate::eq(0.5 * little_r),
                                      predicate::always())
                                .times(objects_count)
                                .return_const(());
    engine.camera = Camera{ position: middle, zoom: 0.5 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}


#[test]
fn zooming_not_centered_around_camera_position() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // Place some circles equally spaced along a circle around some arbitrary point (`middle`).
    let middle = Vector2 { x: 100.0, y: 50.0 };
    let objects_count = 10;
    let angle_step = 2.0 * f32::consts::PI / (objects_count as f32);
    let big_r = 50.0;
    let little_r = 1.0;
    for i in 0..objects_count {
        let angle = (i as f32) * angle_step;
        let object = FakePaintObject {
            p1: Some(Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            }),
            ..Default::default()
        };
        engine.objects.push(Box::new(object));
    }

    let camera_from_middle = Vector2 { x: 100.0, y: 200.0 };

    // Using a camera *not* centered at `middle` and with zoom 1.0,
    // their radius is the same as in world-space.
    // We can't say anything about their position.
    let mut painter = MockScreenPainter::default();
    painter.expect_draw_rectangle_filled().return_const(()); // drawing the background color
    painter.expect_draw_circle().with(predicate::always(),
                                      predicate::eq(little_r),
                                      predicate::always())
                                .times(objects_count)
                                .return_const(());
    engine.camera = Camera{ position: middle + camera_from_middle, zoom: 1.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    // With zoom == 2.0, their radius is twice as big.
    let mut painter = MockScreenPainter::default();
    painter.expect_draw_rectangle_filled().return_const(()); // drawing the background color
    painter.expect_draw_circle().with(predicate::always(),
                                      predicate::eq(2.0 * little_r),
                                      predicate::always())
                                .times(objects_count)
                                .return_const(());
    engine.camera = Camera{ position: middle, zoom: 2.0 };
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}


#[test]
fn user_input_zoom() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Zoom { delta: 0.0 }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom + 0.0);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Zoom { delta: 1.0 }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom + 1.0);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Zoom { delta: -1.0 }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom - 1.0);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Zoom { delta: 10.0 }, STROKE, BG_COLOR);
    engine.update(UserInput::Zoom { delta: -10.0 }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 50.0 } }, STROKE, BG_COLOR);
    engine.update(UserInput::Zoom { delta: 10.0 }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom + 10.0);

    let old_zoom = engine.camera.zoom;
    engine.update(UserInput::Zoom { delta: 10.0 }, STROKE, BG_COLOR);
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 50.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.zoom, old_zoom + 10.0);
}

#[test]
fn user_input_pan() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    engine.camera.zoom = 1.0;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 0.0, y: 0.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 0.0);
    assert_relative_eq!(engine.camera.position.y, 0.0);

    engine.camera.zoom = 1.0;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 10.0);
    assert_relative_eq!(engine.camera.position.y, 1.0);

    engine.camera.zoom = 1.0;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 20.0);
    assert_relative_eq!(engine.camera.position.y, 2.0);

    // The interesting part begins here:
    // The pan delta is received in screen coordinates,
    // but the camera position is in world coordinates.
    // The more zoomed in the camera is, the less a given delta moves it.

    engine.camera.zoom = 10.0;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 100.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 1.0);
    assert_relative_eq!(engine.camera.position.y, 10.0);

    engine.camera.zoom = 0.5;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 20.0);
    assert_relative_eq!(engine.camera.position.y, 2.0);

    engine.camera.zoom = 0.5;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 40.0);
    assert_relative_eq!(engine.camera.position.y, 4.0);

    engine.camera.zoom = 1.0;
    engine.camera.position = Vector2 { x: 0.0, y: 0.0 };
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    engine.camera.zoom = 0.5;
    engine.update(UserInput::Pan { delta: Vector2 { x: 10.0, y: 1.0 } }, STROKE, BG_COLOR);
    assert_relative_eq!(engine.camera.position.x, 30.0);
    assert_relative_eq!(engine.camera.position.y, 3.0);
}

#[test]
fn no_selection_without_user_input() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // None of the following user inputs should cause any objects to be selected.

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::MouseMove { position: Vector2{x: 5.0, y: 5.4}, button: MouseButton::None, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::Pan { delta: Vector2{x: 1.0, y: 0.1 } }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::MouseClick { button: MouseButton::Right, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn no_selection_if_a_tool_is_selected() {
    let mut tool1 = MockTool::new();
    tool1.expect_update().return_const(());
    tool1.expect_draw().return_const(());
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(vec![Box::new(tool1)], view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    engine.select_tool(Some(0));

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn single_selection() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click neither
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn selection_with_shift() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // While shift is being held down, all objects are selected that the user clicks,
    // even if they click somewhere where there is no object.
    // BUT if they click a selected object it will be deselected again.

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click neither
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click neither
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn selection_remains_if_no_input() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // do nothing
    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), true);

    // irrelevant input
    engine.update(UserInput::MouseMove { button: MouseButton::None, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), true);
}

#[test]
fn selection_with_shift_except_first() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // This is exactly the same as the `selection_with_shift` test,
    // except that shift is NOT held down during the FIRST click.
    // Everything should work the same.

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click neither
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click neither
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click object #2
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn single_selection_after_multiple_selection() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // When an object is clicked without shift, that object is selected
    // but all other objects are deselected.

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // select all
    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click object #1
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn clicking_elsewhere_deselects_all() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // When the user clicks someshere where there are no objects without shift,
    // all objects are deselected.

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // select all
    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // click elsewhere
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // select some
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    // click elsewhere
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn select_all_input_selects_all() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    // It even works when we start with only some of the objects selected.

    // (first we click somewhere else to deselect all objects then click on #1 to select it)
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 100.0, y: 100.0}, is_shift_down: false }, STROKE, BG_COLOR);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);
}

#[test]
fn deselect_all_input_deselects_all() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    // When we start with no objects selected it has no effect.

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // It works with some objects selected.

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), false);

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);

    // And with all objects selected.

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 15.0, y: 15.0}, is_shift_down: true }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
    assert_eq!(engine.objects[1].is_selected(), true);

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    assert_eq!(engine.objects[1].is_selected(), false);
}

#[test]
fn zoom_and_pan_do_not_affect_selection() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    // control tests
    assert_eq!(engine.objects[0].is_selected(), false);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 11.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);

    // reset
    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);

    engine.update(UserInput::Zoom { delta: 20.0 }, STROKE, BG_COLOR);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);

    // reset
    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), false);

    engine.update(UserInput::Pan { delta: Vector2 { x: 100.0, y: -500.0 } }, STROKE, BG_COLOR);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 5.0, y: 5.0}, is_shift_down: false }, STROKE, BG_COLOR);
    assert_eq!(engine.objects[0].is_selected(), true);
}

#[test]
fn no_selection_marker_when_no_objects_selected() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color
    painter.expect_draw_rectangle().never(); // As long as nothing is selected, nothing should draw unfilled rectangles.

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    // click somewhere else
    engine.update(UserInput::MouseClick { position: Vector2 { x: 100.0, y: 100.0 }, button: MouseButton::Left, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    // click on object #1 with right click
    engine.update(UserInput::MouseClick { position: Vector2 { x: 5.0, y: 5.0 }, button: MouseButton::Right, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Zoom { delta: 0.1 }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Pan { delta: Vector2 { x: -1.0, y: -10.0 } }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}


#[test]
fn selection_marker_around_selected_object() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color
    // There should be a rectangle drawn around the selected object.
    painter.expect_draw_rectangle()
           .once()
           .with(predicate::eq(Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } }), predicate::always())
           .return_const(());

    // select object #1
    engine.update(UserInput::MouseClick { position: Vector2 { x: 5.0, y: 5.0 }, button: MouseButton::Left, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn selection_markers_with_multiple_selected_objects() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color
    // There should be a rectangle drawn around each selected object.
    painter.expect_draw_rectangle()
           .once()
           .with(predicate::eq(Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } }), predicate::always())
           .return_const(());
    painter.expect_draw_rectangle()
           .once()
           .with(predicate::eq(Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } }), predicate::always())
           .return_const(());

    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::DeselectAll, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn delete_input_does_nothing_with_nothing_selected() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        p1: Some(Vector2{x: 1.0, y: 2.0}),
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        p1: Some(Vector2{x: 11.0, y: 12.0}),
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color
    // draw a circle for each object before the delete input and after the delete input
    painter.expect_draw_circle()
           .times(4)
           .return_const(());

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Delete, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn delete_input_deletes_all_selected_objects() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let object1 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 0.0, y: 0.0 }, p2: Vector2 { x: 10.0, y: 10.0 } },
        p1: Some(Vector2{x: 1.0, y: 2.0}),
        ..Default::default()
    };
    engine.objects.push(Box::new(object1));

    let object2 = FakePaintObject {
        bounding_rect: Rectangle { p1: Vector2 { x: 10.0, y: 10.0 }, p2: Vector2 { x: 20.0, y: 20.0 } },
        p1: Some(Vector2{x: 11.0, y: 12.0}),
        ..Default::default()
    };
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color
    painter.expect_draw_rectangle().return_const(()); // selection markers
    // draw a circle for each object before the delete input but not after it
    painter.expect_draw_circle()
           .times(2)
           .return_const(());

    engine.update(UserInput::SelectAll, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Delete, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}
