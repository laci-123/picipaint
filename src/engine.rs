use crate::primitives::*;


pub trait ScreenPainter {
    type Texture;
    fn draw_line(&mut self, start: Vector2<ScreenSpace>, end: Vector2<ScreenSpace>, stroke: Stroke<ScreenSpace>);
    fn draw_circle(&mut self, center: Vector2<ScreenSpace>, radius: Number<ScreenSpace>, stroke: Stroke<ScreenSpace>);
    fn draw_rectangle(&mut self, rectangle: Rectangle<ScreenSpace>, stroke: Stroke<ScreenSpace>);
    fn draw_rectangle_filled(&mut self, rectangle: Rectangle<ScreenSpace>, color: Color, stroke: Option<Stroke<ScreenSpace>>);
    fn load_image(&mut self, name: &str, image: &image::DynamicImage) -> Self::Texture;
    fn draw_image(&mut self, frame: Rectangle<ScreenSpace>, texture: &Self::Texture);
}


pub struct WorldPainter<'a, P: ScreenPainter> {
    screen_painter: &'a mut P,
}

impl<'a, P: ScreenPainter> WorldPainter<'a, P> {
    pub fn draw_line(&mut self, start: Vector2<WorldSpace>, end: Vector2<WorldSpace>, stroke: Stroke<WorldSpace>, camera: &Camera) {
        let s = camera.point_to_screen_coordinates(start);
        let e = camera.point_to_screen_coordinates(end);
        self.screen_painter.draw_line(s, e, camera.stroke_to_screen_coordinates(stroke));
    }
    
    // pub fn draw_circle(&mut self, center: Vector2, radius: f32, stroke: Stroke, camera: &Camera) {
    //     let c = camera.convert_to_screen_coordinates(center);
    //     let r = camera.zoom * radius;
    //     self.screen_painter.draw_circle(c, r, stroke.with_scaled_thickness(camera.zoom));
    // }
    
    pub fn draw_rectangle(&mut self, rectangle: Rectangle<WorldSpace>, stroke: Stroke<WorldSpace>, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.point_to_screen_coordinates(rectangle.p1),
            p2: camera.point_to_screen_coordinates(rectangle.p2),
        };
        self.screen_painter.draw_rectangle(rect, camera.stroke_to_screen_coordinates(stroke));
    }
    
    // pub fn draw_rectangle_filled(&mut self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>, camera: &Camera) {
    //     let rect = Rectangle {
    //         p1: camera.convert_to_screen_coordinates(rectangle.p1),
    //         p2: camera.convert_to_screen_coordinates(rectangle.p2),
    //     };
    //     self.screen_painter.draw_rectangle_filled(rect, color, stroke.map(|s| s.with_scaled_thickness(camera.zoom)));
    // }

    pub fn load_image(&mut self, name: &str, image: &image::DynamicImage) -> P::Texture {
        self.screen_painter.load_image(name, image)
    }
    
    pub fn draw_image(&mut self, frame: Rectangle<WorldSpace>, texture: &P::Texture, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.point_to_screen_coordinates(frame.p1),
            p2: camera.point_to_screen_coordinates(frame.p2),
        };
        self.screen_painter.draw_image(rect, texture);
    }
}


#[derive(PartialEq, Debug)]
pub enum MouseButton {
    None,
    Left,
    Right,
}


#[derive(PartialEq, Debug)]
pub enum UserInput {
    Nothing,
    MouseClick {
        position: Vector2<ScreenSpace>,
        button: MouseButton,
        is_shift_down: bool,
    },
    MouseMove {
        position: Vector2<ScreenSpace>,
        delta: Vector2<ScreenSpace>,
        button: MouseButton,
        is_shift_down: bool,
    },
    SelectAll,
    DeselectAll,
    Zoom {
        delta: f32,
    },
    Pan {
        delta: Vector2<ScreenSpace>,
    },
    Delete,
}

impl UserInput {
    pub fn mouse_position(&self) -> Option<Vector2<ScreenSpace>> {
        match self {
            Self::MouseClick { position, .. } => Some(*position),
            Self::MouseMove { position, .. }  => Some(*position),
            _                                 => None,
        }
    }

    pub fn mouse_delta(&self) -> Option<Vector2<ScreenSpace>> {
        match self {
            Self::MouseMove { delta, .. } => Some(*delta),
            _                             => None,
        }
    }

    pub fn mouse_is_up(&self) -> bool {
        match self {
            Self::MouseMove { button: MouseButton::None, .. } => true,
            Self::MouseMove { .. }                            => false,
            Self::MouseClick { .. }                           => false,
            _                                                 => true,
        }
    }
}


pub struct PaintObjectCommon {
    pub is_selected: bool,
}

pub trait PaintObject<P: ScreenPainter> {
    fn base(&self) -> &PaintObjectCommon;
    fn base_mut(&mut self) -> &mut PaintObjectCommon;
    fn update(&mut self, input: &UserInput, camera: &Camera);
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, P>, camera: &Camera);
    fn is_under_mouse(&self) -> bool;
    fn get_bounding_rect(&self) -> Rectangle<WorldSpace>;
    fn shift_with(&mut self, p: Vector2<WorldSpace>);
    fn resize_to(&mut self, new_size: Rectangle<WorldSpace>);
}


pub trait Tool<P: ScreenPainter, IconType> {
    fn update(&mut self, input: &UserInput, stroke: Stroke<WorldSpace>, camera: &Camera) -> Result<Option<Box<dyn PaintObject<P>>>, String>;
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, P>, background_color: Color, camera: &Camera);
    fn display_name(&self) -> &str;
    fn icon(&self) -> IconType;
}


pub struct ToolIterator<'a, P: ScreenPainter, IconType> {
    tools: &'a Vec<Box<dyn Tool<P, IconType>>>,
    index: usize,
}

impl<'a, P: ScreenPainter, IconType> Iterator for ToolIterator<'a, P, IconType> {
    type Item = &'a dyn Tool<P, IconType>;

    fn next(&mut self) -> Option<&'a dyn Tool<P, IconType>> {
        if let Some(tool) = self.tools.get(self.index) {
            self.index += 1;
            Some(tool.as_ref())
        }
        else {
            None
        }
    }
}


pub struct Engine<P: ScreenPainter, IconType> {
    objects: Vec<Box<dyn PaintObject<P>>>,
    tools: Vec<Box<dyn Tool<P, IconType>>>,
    to_be_deleted: Vec<usize>,
    selected_tool_index: Option<usize>,
    view_width: f32,
    view_height: f32,
    camera: Camera,
    objects_are_dragged: bool,
    object_is_resized_by_vertex: Option<RectangleVertex>,
}

impl<P: ScreenPainter, IconType> Engine<P, IconType> {
    const MINIMUM_OBJECT_SIZE: Number<WorldSpace> = Number::<WorldSpace>::new(20.0);
    const SELECTION_MARKER_SIZE: Number<ScreenSpace> = Number::<ScreenSpace>::new(5.0);

    pub fn new(tools: Vec<Box<dyn Tool<P, IconType>>>) -> Self {
        Self {
            objects: Vec::new(),
            tools,
            to_be_deleted: Vec::new(),
            selected_tool_index: None,
            view_width: 0.0,
            view_height: 0.0,
            camera: Camera::default(),
            objects_are_dragged: false,
            object_is_resized_by_vertex: None,
        }
    }

    pub fn add_object(&mut self, object: impl PaintObject<P> + 'static) {
        self.objects.push(Box::new(object));
    }
    
    pub fn update(&mut self, input: UserInput, stroke: Stroke<WorldSpace>, view_width: f32, view_height: f32) -> Result<(), String> {
        self.view_width = view_width;
        self.view_height = view_height;
        self.camera.offset = Vector2::new(view_width / 2.0, view_height / 2.0);

        match input {
            UserInput::Pan { delta } => {
                self.camera.position += self.camera.distance_to_world_coordinates(delta);
            },
            UserInput::Zoom { delta } => {
                self.camera.zoom += delta;
                if self.camera.zoom < 0.0 {
                    self.camera.zoom = 0.0;
                }
            },
            _ => {
                self.update_tools_and_objects(input, stroke)?;
            },
        }

        Ok(())
    }

    fn update_tools_and_objects(&mut self, input: UserInput, stroke: Stroke<WorldSpace>) -> Result<(), String> {
        if let Some(tool_index) = self.selected_tool_index {
            if let Some(tool) = self.tools.get_mut(tool_index) {
                if let Some(new_object) = tool.update(&input, stroke, &self.camera)? {
                    self.objects.push(new_object);
                }
            }
        }

        if input == UserInput::SelectAll {
            self.select_tool(None);
        }
        if input.mouse_is_up() {
            self.objects_are_dragged = false;
            self.object_is_resized_by_vertex = None;
        }

        for (i, object) in self.objects.iter_mut().enumerate() {
            object.update(&input, &self.camera);

            if self.selected_tool_index.is_none() {
                if input == UserInput::SelectAll {
                    object.base_mut().is_selected = true;
                    continue;
                }
                if input == UserInput::DeselectAll {
                    object.base_mut().is_selected = true;
                    continue;
                }
                if input == UserInput::Delete && object.base().is_selected {
                    self.to_be_deleted.push(i);
                    continue;
                }

                let left_click    = matches!(input, UserInput::MouseClick { button: MouseButton::Left, .. });
                let shift_is_down = matches!(input, UserInput::MouseClick { is_shift_down: true, .. });

                if left_click {
                    if object.is_under_mouse() {
                        if shift_is_down {
                            object.base_mut().is_selected = !object.base().is_selected;
                        }
                        else {
                            object.base_mut().is_selected = true;
                        }
                    }
                    else {
                        if !shift_is_down {
                            object.base_mut().is_selected = false;
                        }
                    }
                }
                if object.base().is_selected && object.is_under_mouse() {
                    self.objects_are_dragged = true;
                }
            }
        }

        for object in self.objects.iter_mut() {
            let Some(mouse_delta)    = input.mouse_delta()   .map(|d| self.camera.distance_to_world_coordinates(d)) else {break};
            let Some(mouse_position) = input.mouse_position().map(|p| self.camera.point_to_world_coordinates(p))  else {break};

            if object.base().is_selected {
                let selection_marker_size = self.camera.size_to_world_coordinates(Self::SELECTION_MARKER_SIZE);
                if let Some(vertex) = object.get_bounding_rect().vertex_under_point(mouse_position, selection_marker_size) {
                    self.object_is_resized_by_vertex = Some(vertex);
                }
                if let Some(vertex) = self.object_is_resized_by_vertex {
                    let new_rect = object.get_bounding_rect().resize_by_dragging_vertex(vertex, mouse_delta);
                    if new_rect.width() > Self::MINIMUM_OBJECT_SIZE && new_rect.height() > Self::MINIMUM_OBJECT_SIZE {
                        object.resize_to(new_rect);
                    }
                }
                else if self.objects_are_dragged {
                    object.shift_with(mouse_delta);
                }
            }
        }

        for i in self.to_be_deleted.iter().rev() {
            // going in reverse order to avoid shifting indeces
            self.objects.swap_remove(*i);
        }
        self.to_be_deleted.clear();

        Ok(())
    }

    pub fn draw(&self, screen_painter: &mut P, background_color: Color) {
        screen_painter.draw_rectangle_filled(Rectangle::from_point_and_size(Vector2::zero(), Number::new(self.view_width), Number::new(self.view_height)), background_color, None);

        for object in self.objects.iter() {
            let mut world_painter = WorldPainter { screen_painter };
            object.draw(&mut world_painter, &self.camera);
            if object.base().is_selected {
                let world_rect = object.get_bounding_rect();
                let screen_rect = Rectangle {
                    p1: self.camera.point_to_screen_coordinates(world_rect.p1),
                    p2: self.camera.point_to_screen_coordinates(world_rect.p2),
                };
                let selection_marker_stroke = Stroke::new(background_color.inverse(), Number::<ScreenSpace>::new(2.0));
                screen_painter.draw_rectangle(screen_rect, selection_marker_stroke);
                for vertex in screen_rect.vertices() {
                    screen_painter.draw_circle(vertex, Self::SELECTION_MARKER_SIZE, selection_marker_stroke);
                }
            }
        }

        if let Some(tool_index) = self.selected_tool_index {
            let mut world_painter = WorldPainter { screen_painter };
            if let Some(tool) = self.tools.get(tool_index) {
                tool.draw(&mut world_painter, background_color, &self.camera);
            }
        }
    }

    pub fn tools_iter(&self) -> ToolIterator<P, IconType> {
        ToolIterator { tools: &self.tools, index: 0 }
    }

    pub fn select_tool(&mut self, index: Option<usize>) {
        self.selected_tool_index = index;
        if index.is_some() {
            for object in self.objects.iter_mut() {
                object.base_mut().is_selected = false;
            }
        }
    }

    pub fn get_selected_tool_index(&self) -> Option<usize> {
        self.selected_tool_index
    }
}
