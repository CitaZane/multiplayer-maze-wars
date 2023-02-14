use crate::{Map, SCREEN_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use ggez::{
    glam::Vec2,
    graphics::{
        self, Color, DrawMode, DrawParam, Drawable, Image, Mesh, PxScale, Rect, Text, TextAlign,
        TextFragment, TextLayout,
    },
    Context, GameResult,
};
const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;
pub enum ViewType {
    Game(Map),
    MainMenu,
    JoinGame,
    CreateGame,
}

pub struct View {
    pub current_view: ViewType,
    pub elements: Vec<Element>,
    button_dimensions: Button,
    input_dimensions: Input,
    pub ip_input_active: bool,
    pub name_input_active: bool,
    pub ip_address: Text,
    pub name: Text,
    back_arrow_image: Image,
}

pub struct Button {
    width: f32,
    height: f32,
    horizontal_offset: f32,
}

pub struct Input {
    width: f32,
    height: f32,
    padding: f32,
    horizontal_offset: f32,
}

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub rect: Rect,
}
impl View {
    pub fn new(ctx: &mut Context, view_type: ViewType) -> GameResult<View> {
        let button_dimensions = Button {
            width: 125.0,
            height: 35.0,
            horizontal_offset: (SCREEN_WIDTH - 125.0) * 0.5,
        };

        let input_dimensions = Input {
            width: 200.0,
            height: 30.0,
            padding: 10.0,
            horizontal_offset: (SCREEN_WIDTH - 200.0) * 0.5,
        };

        let elements = match view_type {
            ViewType::Game(_) => Vec::new(),
            ViewType::MainMenu => View::get_main_menu_elements(&button_dimensions),
            ViewType::JoinGame => {
                View::get_join_game_elements(&button_dimensions, &input_dimensions)
            }
            ViewType::CreateGame => {
                View::get_create_game_elements(&button_dimensions, &input_dimensions)
            }
        };
        Ok(View {
            current_view: view_type,
            elements,
            button_dimensions,
            input_dimensions,
            ip_input_active: false,
            name_input_active: false,
            ip_address: Text::new(""),
            name: Text::new(""),
            back_arrow_image: Image::from_path(ctx, "/back-arrow.png")?,
        })
    }

    pub fn get_main_menu_elements(button_dimensions: &Button) -> Vec<Element> {
        let mut elems = Vec::new();
        let buttons_gap = 75.0;

        elems.push(Element {
            name: "CREATE_GAME".to_string(),
            rect: graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0,
                button_dimensions.width,
                button_dimensions.height,
            ),
        });
        elems.push(Element {
            name: "JOIN_GAME".to_string(),
            rect: graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0 + buttons_gap,
                button_dimensions.width,
                button_dimensions.height,
            ),
        });

        elems
    }
    pub fn get_create_game_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> Vec<Element> {
        let mut elems = Vec::new();

        elems.push(Element {
            name: "CREATE_GAME".to_string(),
            rect: graphics::Rect::new(
                button_dimensions.horizontal_offset,
                275.0,
                button_dimensions.width,
                button_dimensions.height,
            ),
        });
        elems.push(Element {
            name: "NAME_INPUT".to_string(),
            rect: graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0,
                input_dimensions.width,
                input_dimensions.height,
            ),
        });

        elems.push(Element {
            name: "BACK_ARROW_IMG".to_string(),
            rect: graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        });

        elems
    }
    pub fn get_join_game_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> Vec<Element> {
        let mut elems = Vec::new();
        let input_gap = 75.0;

        elems.push(Element {
            name: "NAME_INPUT".to_string(),
            rect: graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0,
                input_dimensions.width,
                input_dimensions.height,
            ),
        });
        elems.push(Element {
            name: "IP_INPUT".to_string(),
            rect: graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0 + input_gap,
                input_dimensions.width,
                input_dimensions.height,
            ),
        });
        elems.push(Element {
            name: "JOIN_GAME".to_string(),
            rect: graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0 + (input_gap * 2.0),
                button_dimensions.width,
                button_dimensions.height,
            ),
        });

        elems.push(Element {
            name: "BACK_ARROW_IMG".to_string(),
            rect: graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        });

        elems
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        match &self.current_view {
            ViewType::Game(map) => View::draw_game(canvas, ctx, map)?,
            ViewType::MainMenu => self.draw_main_menu(canvas, ctx)?,
            ViewType::JoinGame => self.draw_join_game(canvas, ctx)?,
            ViewType::CreateGame => self.draw_create_game(canvas, ctx)?,
        };
        Ok(())
    }

    fn draw_join_game(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        // title
        let title = Text::new(TextFragment {
            text: "Maze Wars".to_string(),
            scale: Some(PxScale::from(35.0)),
            color: Some(Color::BLACK),
            ..TextFragment::default()
        });
        let title_horizontal_offset = (SCREEN_WIDTH - title.dimensions(ctx).unwrap().w) * 0.5;
        canvas.draw(
            &title,
            DrawParam::from(Vec2::new(title_horizontal_offset, 100.0)),
        );

        // -- BUTTON TEXTS --
        // join game text
        let mut join_game_text = Text::new("Join game");
        join_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &join_game_text,
            DrawParam::from(Vec2::new(
                self.button_dimensions.horizontal_offset + self.button_dimensions.width / 2.0,
                350.0 + self.button_dimensions.height / 2.0,
            ))
            .color(Color::BLACK),
        );

        // -- INPUTS --
        let input_gap = 75.0;

        // name input label
        let mut name_input_label = Text::new("NAME");
        name_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &name_input_label,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.width / 2.0,
                200.0 - 5.0,
            ))
            .color(Color::BLACK),
        );

        // name input
        let mut name_input = Text::new(self.name.contents());
        name_input.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Begin,
        });

        canvas.draw(
            &name_input,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.padding,
                200.0 + (self.input_dimensions.height / 2.0),
            ))
            .color(Color::BLACK),
        );
        // ip input label
        let mut ip_input_label = Text::new("IP ADDRESS");
        ip_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &ip_input_label,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.width / 2.0,
                200.0 + input_gap - 5.0,
            ))
            .color(Color::BLACK),
        );
        //ip input
        let mut ip_input = Text::new(self.ip_address.contents());
        ip_input.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Begin,
        });

        canvas.draw(
            &ip_input,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.padding,
                200.0 + input_gap + (self.input_dimensions.height / 2.0),
            ))
            .color(Color::BLACK),
        );

        canvas.draw(
            &self.back_arrow_image,
            DrawParam::from(Vec2::new(100.0, 100.0)).scale([0.1, 0.1]),
        );

        // button and inputs rects
        for elem in &self.elements {
            let mut draw_mode = DrawMode::stroke(1.0);
            if elem.name == "IP_INPUT" && self.ip_input_active
                || elem.name == "NAME_INPUT" && self.name_input_active
            {
                draw_mode = DrawMode::stroke(3.0)
            }

            let elem = graphics::Mesh::new_rectangle(ctx, draw_mode, elem.rect, Color::BLACK)?;

            canvas.draw(&elem, DrawParam::default())
        }

        Ok(())
    }

    fn draw_create_game(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        // title
        let title = Text::new(TextFragment {
            text: "Maze Wars".to_string(),
            scale: Some(PxScale::from(35.0)),
            color: Some(Color::BLACK),
            ..TextFragment::default()
        });
        let title_horizontal_offset = (SCREEN_WIDTH - title.dimensions(ctx).unwrap().w) * 0.5;
        canvas.draw(
            &title,
            DrawParam::from(Vec2::new(title_horizontal_offset, 100.0)),
        );
        // create game text
        let mut create_game_text = Text::new("Create game");
        create_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &create_game_text,
            DrawParam::from(Vec2::new(
                self.button_dimensions.horizontal_offset + self.button_dimensions.width / 2.0,
                275.0 + self.button_dimensions.height / 2.0,
            ))
            .color(Color::BLACK),
        );

        // name input label
        let mut name_input_label = Text::new("NAME");
        name_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &name_input_label,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.width / 2.0,
                200.0 - 5.0,
            ))
            .color(Color::BLACK),
        );

        // name input
        let mut name_input = Text::new(self.name.contents());
        name_input.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Begin,
        });

        canvas.draw(
            &name_input,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.padding,
                200.0 + (self.input_dimensions.height / 2.0),
            ))
            .color(Color::BLACK),
        );

        // button and inputs rects
        for elem in &self.elements {
            let mut draw_mode = DrawMode::stroke(1.0);
            if elem.name == "IP_INPUT" && self.ip_input_active
                || elem.name == "NAME_INPUT" && self.name_input_active
            {
                draw_mode = DrawMode::stroke(3.0)
            }

            let elem = graphics::Mesh::new_rectangle(ctx, draw_mode, elem.rect, Color::BLACK)?;

            canvas.draw(&elem, DrawParam::default())
        }
        canvas.draw(
            &self.back_arrow_image,
            DrawParam::from(Vec2::new(100.0, 100.0)).scale([0.1, 0.1]),
        );
        Ok(())
    }

    fn draw_game(canvas: &mut graphics::Canvas, ctx: &mut Context, map: &Map) -> GameResult {
        map.draw(canvas, ctx);
        let frame = graphics::Rect::new(X, Y, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            frame,
            Color::from_rgb(0, 0, 0),
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }

    fn draw_main_menu(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        // title
        let title = Text::new(TextFragment {
            text: "Maze Wars".to_string(),
            scale: Some(PxScale::from(35.0)),
            color: Some(Color::BLACK),
            ..TextFragment::default()
        });
        let title_horizontal_offset = (SCREEN_WIDTH - title.dimensions(ctx).unwrap().w) * 0.5;

        // button text
        let mut create_game_text = Text::new("Create game");
        create_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });
        let mut join_game_text = Text::new("Join game");
        join_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });

        let button_width = 125.0;
        let button_horizontal_offset = (SCREEN_WIDTH - button_width) * 0.5;
        for button in &self.elements {
            let btn = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(1.0),
                button.rect,
                Color::BLACK,
            )?;

            canvas.draw(&btn, DrawParam::default())
        }
        // draw
        canvas.draw(
            &title,
            DrawParam::from(Vec2::new(title_horizontal_offset, 100.0)),
        );

        // join game text
        canvas.draw(
            &join_game_text,
            DrawParam::from(Vec2::new(
                button_horizontal_offset + button_width / 2.0,
                275.0 + 17.5,
            ))
            .color(Color::BLACK),
        );
        // start game text
        canvas.draw(
            &create_game_text,
            DrawParam::from(Vec2::new(
                button_horizontal_offset + button_width / 2.0,
                200.0 + 17.5,
            ))
            .color(Color::BLACK),
        );

        Ok(())
    }
}
