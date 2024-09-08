use crate::utils::lsystem::LSystem;
use iced::Font;
use iced::widget::{
    Column, TextInput, slider, row, text, container, column, button, tooltip,
};
use iced::{Center, Element, Task, Theme};


pub fn main_result() -> iced::Result {
    iced::application("Lindenmayer viewer", Lindenmayer::update, Lindenmayer::view)
        .theme(|_| Theme::Nord)
        .centered()
        .default_font(Font::MONOSPACE)
        .run()
}

fn get_vertices_from_system(n: usize, rule: &str, angle: f64) -> Vec<Vec<(f64, f64)>> {
    let mut system = LSystem::new(n, rule, "X", angle, 80.0);
    system.fill_string();
    system.fill_vertices();
    system.vertices
}

#[derive(Debug, Clone)]
enum Preset {
    Fern,
    Bricks,
    Hilbert,
    Dragon,
    Koch,
    Sierpinski,
}


struct Lindenmayer {
    system: LSystem,
    drawing: drawing::LindenmayerDrawing,
    n_iterations: u32,
    angle: f64,
    rule: String,
    axiom: String,
    scale: f32,
    vertices: Vec<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone)]
enum Message {
    RuleChanged(String),
    AxiomChanged(String),
    IterationsChanged(u32),
    AngleChanged(String),
    ScaleChanged(f32),
    Drawing(drawing::Message),
    UpdateSystem,
    LoadPreset(Preset),
}

impl Preset {
    pub fn parameters(&self) -> (&str, &str, f64) {
        match &self {
            Preset::Fern => ("X=>F-[[X]+X]+F[+FX]-X, F=>FF", "X", 22.5),
            Preset::Hilbert => ("A=>+BF-AFA-FB+, B=>-AF+BFB+FA-", "A", 90.0),
            Preset::Koch => ("F=>F+F--F+F", "F", 60.0),
            Preset::Bricks => ("F=>FF+F-F+F+FF", "F+F+F+F", 90.0),
            Preset::Sierpinski => ("F=>G-F-G, G=>F+G+F", "F", 60.0),
            Preset::Dragon => ("F=>F+X, X=>F-X", "F", 90.0),
        }
    }
}

impl Lindenmayer {
    fn new() -> Self {

        let v = get_vertices_from_system(3, "X=F[-X][+X]", 30.0);

        Self {
            system: LSystem::default(),
            drawing: drawing::LindenmayerDrawing::new(v.clone()),
            n_iterations: 3,
            angle: 30.0,
            scale: 1.0,
            rule: "X=>F[-X][+X]".to_string(),
            axiom: "X".to_string(),
            vertices: v.clone(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {

        match message {
            Message::Drawing(drawmessage) => {
                self.update_lsystem();

                self.drawing.update(drawing::Message::Draw(self.vertices.clone()));
            }
            Message::UpdateSystem => {
                self.update_lsystem();
            }
            Message::RuleChanged(rule) => {
                self.rule = rule;
            }
            Message::AxiomChanged(axiom) => {
                self.axiom = axiom;
            }
            Message::ScaleChanged(scale) => {
                self.scale = scale.into();
                self.drawing.update(drawing::Message::UpdateScale(scale));
            }
            Message::IterationsChanged(n) => {
                self.n_iterations = n;
            }
            Message::AngleChanged(anglestring) => {
                self.angle = anglestring.parse::<f64>().unwrap_or(0.0);
            }
            Message::LoadPreset(preset) => {
                self.preset(preset.parameters());
            }
        };
        return Task::none();
    }

    fn preset(&mut self, parameters: (&str, &str, f64)) {
        let (rule, axiom, angle) = parameters;
        self.rule = rule.to_string();
        self.axiom = axiom.to_string();
        self.angle = angle;

        self.update_lsystem();
    }

    fn view(&self) -> Element<Message> {

        let ruleinput = column![
            text("Rules"),
            TextInput::new(
                "Rules",
                &self.rule,
            )
            .on_input(Message::RuleChanged)
        ]
        .padding(10);

        let axiominput = column![
            text("Axiom"),
            TextInput::new(
                "X",
                &self.axiom,
            )
            .on_input(Message::AxiomChanged)
        ]
        .padding(10);

        let angleinput = column![
            text("Angle (deg)"),
            TextInput::new(
                "30.0",
                &self.angle.to_string(),
            )
            .on_input(Message::AngleChanged)
        ]
        .padding(10);


        let scale_control = column![
            text("Inverse scale"),
            slider(0.1..=50.0, self.scale, Message::ScaleChanged),
            text!("{0}", self.scale.to_string()),
        ]
        .padding(10)
        .align_x(Center);

        let interations_control = column![
            text("Iterations"),
            slider(0..=15, self.n_iterations, Message::IterationsChanged),
            text!("{0}", self.n_iterations.to_string()),
        ]
        .padding(10)
        .align_x(Center);

        let controls = row![
            ruleinput,
            axiominput,
            angleinput,
            interations_control,
            scale_control,
            action(
                text(">"), 
                "Update and redraw", 
                Some(Message::Drawing(drawing::Message::Draw(self.vertices.clone())))
            ),
        ]
        .align_y(Center)
        .padding(30);

        let content = row![
            view_presets(),
            self.drawing
                .view()
                .map(|message| Message::Drawing(message)),
        ];

        column![
            controls,
            content,
        ]
        .padding(10)
        .spacing(20)
        .align_x(Center)
        .into()

    }

    pub fn update_lsystem(&mut self) {
        self.system = LSystem::default();

        self.system = LSystem::new(
            self.n_iterations.try_into().unwrap(),
            &self.rule,
            &self.axiom,
            self.angle,
            80.0,
        );

        self.system.fill_string();
        self.system.fill_vertices();

        self.vertices = self.system.vertices.clone();

    }
}

fn view_presets<'a>() -> Column<'a, Message> {
    column![
        text("Presets"),
        action(text("0"), "Fern", Some(Message::LoadPreset(Preset::Fern))),
        action(text("1"), "Bricks", Some(Message::LoadPreset(Preset::Bricks))),
        action(text("2"), "Sierpinski triangle", Some(Message::LoadPreset(Preset::Sierpinski))),
        action(text("3"), "Hilbert curve", Some(Message::LoadPreset(Preset::Hilbert))),
        action(text("4"), "Dragon curve", Some(Message::LoadPreset(Preset::Dragon))),
        action(text("5"), "Koch curve", Some(Message::LoadPreset(Preset::Koch))),
    ]
    .spacing(5)
    .padding(10)   
}

impl Default for Lindenmayer {
    fn default() -> Self {
        Self::new()
    }
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(25).padding(5));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

mod drawing {
    use iced::mouse;
    use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke};
    use iced::{Element, Point, Rectangle, Renderer, Theme, Fill};

    #[derive(Debug, Clone)]
    pub enum Message {
        Draw(Vec<Vec<(f64, f64)>>),
        UpdateScale(f32),
    }

    pub struct LindenmayerDrawing {
        cache: Cache,
        vertices: Vec<Vec<(f64, f64)>>,
        drawingscale: f32,
    }

    impl Default for LindenmayerDrawing {
        fn default() -> Self {
            Self {
                cache: Cache::default(),
                vertices: Vec::new(),
                drawingscale: 1.0,
            }
        }
    }

    impl LindenmayerDrawing{
        pub fn request_redraw(&mut self) {
            self.cache.clear();
        }

        pub fn new(v: Vec<Vec<(f64, f64)>>) -> Self {
            Self {
                cache: Cache::default(),
                vertices: v,
                drawingscale: 1.0,
            }
        }

        pub fn view(&self) -> Element<Message> {
            Canvas::new(self).width(Fill).height(Fill).into()
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Draw(vertices) => {
                    self.vertices = vertices;
                    self.request_redraw();
                }
                Message::UpdateScale(scale) => {
                    self.drawingscale = scale;
                    self.request_redraw();
                }

                
            }
        }
    }

    impl canvas::Program<Message> for LindenmayerDrawing {
        type State = ();

        fn draw(
            &self, 
            _state: &(),
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<Geometry> {
            let geometry = self.cache.draw(renderer, bounds.size(), |frame| {

                let palette = theme.palette();

                let start = Point::new(frame.center().x, frame.center().y);

                let lines = Path::new(|b| {
                    b.move_to(start);
                    for smallpath in &self.vertices {
                        let initial = smallpath[0];
                        for point in smallpath {

                            let coordinate = Point::new(
                                start.x - point.0 as f32  / self.drawingscale, 
                                start.y - point.1 as f32  / self.drawingscale,
                            );

                            if point.0 == initial.0 && point.1 == initial.1 {
                                b.move_to(coordinate);
                            } else {
                                b.line_to(coordinate);
                                b.move_to(coordinate);
                            }
                            
                        }
                    }
                });
                frame.stroke(&lines, Stroke::default().with_color(palette.primary));
            });

            vec![geometry]
        }
    }

}