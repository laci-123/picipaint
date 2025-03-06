use core::f32;
use approx::{assert_relative_eq, relative_eq};
use mockall::{predicate, Sequence};
use super::*;


const STROKE: Stroke = Stroke { color: Color{ red: 255, green: 255, blue: 255, alpha: 255 }, thickness: 1.0 };
const BG_COLOR: Color = Color{ red: 0, green: 0, blue: 0, alpha: 255 };


#[test]
fn object_draw_order() {
    // First the background color is drawn then
    // the methods of PaintObjects are called in the same order as the objects were created.

    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut seq = Sequence::new();

    let mut object1 = MockPaintObject::new();
    object1.expect_update().once().return_const(());
    object1.expect_draw().once().returning(|painter, camera| {
        painter.draw_circle(Vector2::zero(), 1.0, STROKE, camera);
    });
    object1.expect_set_selected().return_const(());
    engine.objects.push(Box::new(object1));

    let mut object2 = MockPaintObject::new();
    object2.expect_update().once().in_sequence(&mut seq).return_const(());
    object2.expect_draw().once().returning(|painter, camera| {
        painter.draw_line(Vector2::zero(), Vector2::zero(), STROKE, camera);
    });
    object2.expect_set_selected().return_const(());
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().once().in_sequence(&mut seq).return_const(()); // background color
    painter.expect_draw_circle().once().in_sequence(&mut seq).return_const(());           // object1
    painter.expect_draw_line().once().in_sequence(&mut seq).return_const(());             // object2

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);
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
fn tool_selection() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut tool1 = MockTool::new();
    tool1.expect_before_deactivate().once().return_const(());
    engine.tools.push(Box::new(tool1));

    let mut tool2 = MockTool::new();
    tool2.expect_before_deactivate().never().return_const(());
    engine.tools.push(Box::new(tool2));

    assert_eq!(engine.selected_tool_index, 0);
    engine.select_tool(1);
    assert_eq!(engine.selected_tool_index, 1);
    engine.select_tool(1);
    assert_eq!(engine.selected_tool_index, 1);
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
    tool1.expect_before_deactivate().once().in_sequence(&mut seq).return_const(());
    engine.tools.push(Box::new(tool1));

    let mut tool2 = MockTool::new();
    tool2.expect_update().once().in_sequence(&mut seq).return_const(());
    tool2.expect_draw().once().in_sequence(&mut seq).return_const(());
    tool2.expect_before_deactivate().never().return_const(());
    engine.tools.push(Box::new(tool2));

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.select_tool(1);

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
        let mut object = MockPaintObject::new();
        object.expect_update().return_const(());
        object.expect_draw().returning(move |painter, camera| {
            let center = Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            };
            painter.draw_circle(center, little_r, STROKE, camera);
        });
        object.expect_set_selected().return_const(());
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
        let mut object = MockPaintObject::new();
        object.expect_update().return_const(());
        object.expect_draw().returning(move |painter, camera| {
            let center = Vector2{
                x: middle.x + big_r * angle.cos(),
                y: middle.y + big_r * angle.sin()
            };
            painter.draw_circle(center, little_r, STROKE, camera);
        });
        object.expect_set_selected().return_const(());
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

    // no relevant user input so it should never be selected
    let mut object1 = MockPaintObject::new();
    object1.expect_update().return_const(());
    object1.expect_draw().return_const(());
    object1.expect_is_under_mouse().returning(|| true);
    object1.expect_set_selected().times(1..).with(predicate::eq(false)).return_const(());
    engine.objects.push(Box::new(object1));

    // no relevant user input so it should never be selected
    let mut object2 = MockPaintObject::new();
    object2.expect_update().return_const(());
    object2.expect_draw().return_const(());
    object2.expect_is_under_mouse().returning(|| false);
    object2.expect_set_selected().times(1..).with(predicate::eq(false)).return_const(());
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color

    engine.update(UserInput::Nothing, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::MouseMove { position: Vector2{x: 10.0, y: 23.4}, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::Pan { delta: Vector2{x: 1.0, y: 0.1 } }, STROKE, BG_COLOR);
    engine.draw(&mut painter);

    engine.update(UserInput::MouseClick { button: MouseButton::Right, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn single_selection() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    // always under mouse so it should be selected
    let mut object1 = MockPaintObject::new();
    object1.expect_update().return_const(());
    object1.expect_draw().return_const(());
    object1.expect_is_under_mouse().returning(|| true);
    object1.expect_set_selected().times(1..).with(predicate::eq(true)).return_const(());
    engine.objects.push(Box::new(object1));

    // never under mouse so it should never be selected
    let mut object2 = MockPaintObject::new();
    object2.expect_update().return_const(());
    object2.expect_draw().return_const(());
    object2.expect_is_under_mouse().returning(|| false);
    object2.expect_set_selected().times(1..).with(predicate::eq(false)).return_const(());
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: false }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn single_selection_with_shift() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut seq = Sequence::new();

    // when shift is held down and the same object is clicked multiple times,
    // it should be alternately selected and deselected
    // (mock: `is_selected` always returns the last value set in `set_selected`)
    let mut object1 = MockPaintObject::new();
    object1.expect_update().return_const(());
    object1.expect_draw().return_const(());
    object1.expect_is_under_mouse().returning(|| true);
    object1.expect_is_selected() .once().in_sequence(&mut seq).returning(|| false);
    object1.expect_set_selected().once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());
    object1.expect_is_selected() .once().in_sequence(&mut seq).returning(|| true);
    object1.expect_set_selected().once().in_sequence(&mut seq).with(predicate::eq(false)).return_const(());
    object1.expect_is_selected() .once().in_sequence(&mut seq).returning(|| false);
    object1.expect_set_selected().once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());
    object1.expect_is_selected() .once().in_sequence(&mut seq).returning(|| true);
    object1.expect_set_selected().once().in_sequence(&mut seq).with(predicate::eq(false)).return_const(());
    engine.objects.push(Box::new(object1));

    // never under mouse so it should never be selected
    let mut object2 = MockPaintObject::new();
    object2.expect_update().return_const(());
    object2.expect_draw().return_const(());
    object2.expect_is_under_mouse().returning(|| false);
    object2.expect_is_selected().returning(|| false);
    object2.expect_set_selected().times(1..).with(predicate::eq(false)).return_const(());
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}

#[test]
fn multiple_selection_with_shift() {
    let tools = Vec::new();
    let view_width = 1000.0;
    let view_height = 1000.0;
    let mut engine = Engine::<MockScreenPainter>::new(tools, view_width, view_height);

    let mut seq = Sequence::new();

    let mut object1 = MockPaintObject::new();
    object1.expect_update().return_const(());
    object1.expect_draw().return_const(());
    let mut object2 = MockPaintObject::new();
    object2.expect_update().return_const(());
    object2.expect_draw().return_const(());
    object2.expect_is_under_mouse().returning(|| true);
    object2.expect_set_selected().times(1..).with(predicate::eq(true)).return_const(());

    // while shift is held down, once an object is clicked it is always selected
    // (mock: `is_selected` always returns the last value set in `set_selected`)

    // 1st click
    object1.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| true);
    object1.expect_is_selected()   .once().in_sequence(&mut seq).returning(|| false);
    object1.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(()); 
    object2.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| false);
    object2.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(false)).return_const(());
    // 2nd click
    object1.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| false);
    object1.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());
    object2.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| true);
    object2.expect_is_selected()   .once().in_sequence(&mut seq).returning(|| false);
    object2.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());
    // 3rd click
    object1.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| false);
    object1.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());
    object2.expect_is_under_mouse().once().in_sequence(&mut seq).returning(|| false);
    object2.expect_set_selected()  .once().in_sequence(&mut seq).with(predicate::eq(true)).return_const(());

    engine.objects.push(Box::new(object1));
    engine.objects.push(Box::new(object2));

    let mut painter = MockScreenPainter::new();
    painter.expect_draw_rectangle_filled().return_const(()); // background color

    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
    engine.update(UserInput::MouseClick { button: MouseButton::Left, position: Vector2{x: 10.0, y: 23.4}, is_shift_down: true }, STROKE, BG_COLOR);
    engine.draw(&mut painter);
}
