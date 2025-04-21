use eframe::egui::{self, Vec2};
use crate::color_selector::ColorSelector;
use crate::primitives::*;
use crate::engine::*;
use crate::floating_window::FloatingWindow;
use crate::paint_object::{freehand_curve::*, straight_line::*, picture::*};
use crate::egui_painter::*;


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(1000.0, 600.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const UI_SCALE: f32          = 1.5;
pub const NAME: &'static str     = "PiciPaint";


pub struct App {
    engine: Engine<EguiPainter, egui::ImageSource<'static>>,
    stroke: Stroke<WorldSpace>,
    bg_color: Color,
    fg_color_selector: ColorSelector,
    bg_color_selector: ColorSelector,
    error_window: FloatingWindow,
    error_msg: String,
}

impl App {
    pub fn new(_context: &eframe::CreationContext) -> Self {
        Self {
            engine: Engine::new(vec![
                Box::new(FreehandCurveTool::default()),
                Box::new(StraghtLineTool::default()),
                Box::new(PictureTool::default()),
            ]),
            stroke: Stroke::new(Color::from_rgb(0, 0, 200), Number::<WorldSpace>::new(2.0)),
            bg_color: Color::from_rgb(0, 0, 0),
            fg_color_selector: ColorSelector::new("Foreground color"),
            bg_color_selector: ColorSelector::new("Background color"),
            error_window: FloatingWindow::new("error"),
            error_msg: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(UI_SCALE);

        let dropped_pictures = ctx.input(|input| {
            let offset = Vector2::new(10.0, 10.0);
            let mut pictures = Vec::new();
            for (i, dropped_file) in input.raw.dropped_files.iter().enumerate() {
                match Picture::from_dropped_file(dropped_file, offset * (i as f32)) {
                    Ok(Some(picture)) => pictures.push(picture),
                    Ok(None)          => {/*This isn't a picture, just skip it*/},
                    Err(error_msg)    => return Err(error_msg),
                }
            }
            return Ok(pictures);
        });

        match dropped_pictures {
            Ok(pictures) => {
                for picture in pictures {
                    self.engine.add_object(picture);
                }
            },
            Err(error_msg) => {
                self.error_msg = error_msg;
                self.error_window.is_open = true;
            },
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let modal_dialog_is_open = self.error_window.is_open || self.bg_color_selector.window.is_open || self.fg_color_selector.window.is_open;
            if modal_dialog_is_open {
                ui.disable();
            }

            ui.horizontal(|ui| {
                let selected = self.engine.get_selected_tool_index().is_none();
                let image = egui::include_image!("../img/selection_tool.png");
                if ui.add(egui::Button::image(image).frame(selected)).on_hover_ui(|ui| {ui.label("selection");}).clicked() {
                    self.engine.select_tool(None);
                }
                let mut new_selected = None;
                for (i, tool) in self.engine.tools_iter().enumerate() {
                    let selected = self.engine.get_selected_tool_index().is_some_and(|si| si == i);
                    if ui.add(egui::Button::image(tool.icon()).frame(selected)).on_hover_ui(|ui| {ui.label(tool.display_name());}).clicked() {
                        new_selected = Some(i);
                    }
                }
                if new_selected.is_some() {
                    self.engine.select_tool(new_selected);
                }

                ui.separator();

                ui.toggle_value(&mut self.fg_color_selector.window.is_open, "fg color");
                ui.toggle_value(&mut self.bg_color_selector.window.is_open, "bg color");
                ui.add(egui::Slider::new(&mut self.stroke.thickness.value, 0.5..=10.0)).on_hover_ui_at_pointer(|ui| {
                    ui.label("line thickness");
                });
            });

            ui.separator();

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

                let mut p = EguiPainter::new(painter, ctx.clone());

                let user_input = if modal_dialog_is_open {
                    UserInput::Nothing
                }
                else {
                    map_user_input(&response, ui)
                };
                let screen_rect = ui.ctx().input(|input| input.screen_rect);

                if let Err(err) = self.engine.update(user_input, self.stroke, self.bg_color, screen_rect.width(), screen_rect.height()) {
                    self.error_window.is_open = true;
                    self.error_msg = err;
                }

                self.engine.draw(&mut p);
            });

            self.fg_color_selector.update(ctx, &mut self.stroke.color);
            self.bg_color_selector.update(ctx, &mut self.bg_color);
            self.error_window.show(ctx, |ui| {
                ui.heading("Error");
                ui.label(&self.error_msg);
            });
});
    }
}

fn map_user_input(response: &egui::Response, ui: &egui::Ui) -> UserInput {
    let is_shift_down = ui.input(|input| input.modifiers.shift);
    let mouse_wheel_delta = ui.input(|input| input.smooth_scroll_delta.y * 0.001);

    if mouse_wheel_delta != 0.0 {
        return UserInput::Zoom { delta: mouse_wheel_delta };
    }
    if ui.input(|input| input.key_pressed(egui::Key::A) && input.modifiers.command) {
        return UserInput::SelectAll;
    }
    if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        return UserInput::DeselectAll;
    }
    if ui.input(|input| input.key_pressed(egui::Key::Delete)) {
        return UserInput::Delete;
    }
    if response.clicked_by(egui::PointerButton::Primary) {
        if let Some(position) = response.interact_pointer_pos() {
            return UserInput::MouseClick {
                position: Vector2::from(position),
                button: MouseButton::Left,
                is_shift_down
            };
        }
    }
    if response.clicked_by(egui::PointerButton::Secondary) {
        if let Some(position) = response.interact_pointer_pos() {
            return UserInput::MouseClick {
                position: Vector2::from(position),
                button: MouseButton::Right,
                is_shift_down
            };
        }
    }
    if response.dragged_by(egui::PointerButton::Primary) {
        if let Some(position) = response.interact_pointer_pos() {
            return UserInput::MouseMove {
                position: Vector2::from(position),
                delta: Vector2::from(response.drag_delta()),
                button: MouseButton::Left,
                is_shift_down
            };
        }
    }
    if response.dragged_by(egui::PointerButton::Secondary) {
        if let Some(position) = response.interact_pointer_pos() {
            return UserInput::MouseMove {
                position: Vector2::from(position),
                delta: Vector2::from(response.drag_delta()),
                button: MouseButton::Right,
                is_shift_down
            };
        }
    }
    if response.dragged_by(egui::PointerButton::Middle) {
        let delta = response.drag_delta();
        return UserInput::Pan {
            delta: Vector2::new(-1.0 * delta.x, -1.0 * delta.y)
        };
    }
    if response.hovered() {
        if let Some(position) = response.hover_pos() {
            return UserInput::MouseMove {
                position: Vector2::from(position),
                delta: Vector2::from(response.drag_delta()),
                button: MouseButton::None,
                is_shift_down
            };
        }
    }
    return UserInput::Nothing;
}
