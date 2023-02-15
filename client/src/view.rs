use std::collections::HashMap;

use crate::{Game, Map, SCREEN_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
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
    pub element_rects: HashMap<String, Rect>, // holds text input and button rects
    pub ip_input_active: bool,
    pub name_input_active: bool,
    pub ip_address: Text,
    pub name: Text,
    back_arrow_image: Image,
    button_dimensions: Button,
    input_dimensions: Input,
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

        let element_rects = match view_type {
            ViewType::Game(_) => HashMap::new(),
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
            element_rects,
            button_dimensions,
            input_dimensions,
            ip_input_active: false,
            name_input_active: false,
            ip_address: Text::new(""),
            name: Text::new(""),
            back_arrow_image: Image::from_path(ctx, "/back-arrow.png")?,
        })
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

    pub fn get_main_menu_elements(button_dimensions: &Button) -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        let buttons_gap = 75.0;

        elems.insert(
            "CREATE_GAME".to_string(),
            graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0,
                button_dimensions.width,
                button_dimensions.height,
            ),
        );
        elems.insert(
            "JOIN_GAME".to_string(),
            graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0 + buttons_gap,
                button_dimensions.width,
                button_dimensions.height,
            ),
        );

        elems
    }

    pub fn get_create_game_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        elems.insert(
            "CREATE_GAME".to_string(),
            graphics::Rect::new(
                button_dimensions.horizontal_offset,
                275.0,
                button_dimensions.width,
                button_dimensions.height,
            ),
        );
        elems.insert(
            "NAME_INPUT".to_string(),
            graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0,
                input_dimensions.width,
                input_dimensions.height,
            ),
        );
        elems.insert(
            "BACK_ARROW_IMG".to_string(),
            graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        );

        elems
    }
    pub fn get_join_game_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        let input_gap = 75.0;

        elems.insert(
            "NAME_INPUT".to_string(),
            graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0,
                input_dimensions.width,
                input_dimensions.height,
            ),
        );
        elems.insert(
            "IP_INPUT".to_string(),
            graphics::Rect::new(
                input_dimensions.horizontal_offset,
                200.0 + input_gap,
                input_dimensions.width,
                input_dimensions.height,
            ),
        );
        elems.insert(
            "JOIN_GAME".to_string(),
            graphics::Rect::new(
                button_dimensions.horizontal_offset,
                200.0 + (input_gap * 2.0),
                button_dimensions.width,
                button_dimensions.height,
            ),
        );

        elems.insert(
            "BACK_ARROW_IMG".to_string(),
            graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        );

        elems
    }

    fn draw_join_game(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        // title
        self.draw_title(canvas, ctx)?;

        // inputs
        self.draw_name_input(canvas, ctx, 200.0)?;
        self.draw_ip_input(canvas, ctx, 275.0)?;

        // back arrow
        self.draw_back_arrow_img(canvas, ctx)?;

        // buttons
        self.draw_join_game_button(canvas, ctx, 350.0)?;

        Ok(())
    }

    fn draw_create_game(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        // title
        self.draw_title(canvas, ctx)?;
        // name input
        self.draw_name_input(canvas, ctx, 200.0 - 5.0)?;
        // back navigation
        self.draw_back_arrow_img(canvas, ctx)?;
        // btn
        self.draw_create_game_button(canvas, ctx, 275.0)?;

        Ok(())
    }

    fn draw_game(canvas: &mut graphics::Canvas, ctx: &mut Context, map: &Map) -> GameResult {
        map.draw(canvas, ctx)?;
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
        self.draw_title(canvas, ctx)?;
        self.draw_create_game_button(canvas, ctx, 200.0)?;
        self.draw_join_game_button(canvas, ctx, 275.0)?;

        Ok(())
    }

    pub fn draw_title(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
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

        Ok(())
    }

    pub fn draw_name_input(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
    ) -> GameResult {
        let name_input_draw_mode = if self.name_input_active {
            DrawMode::stroke(3.0)
        } else {
            DrawMode::stroke(1.0)
        };

        let name_input = graphics::Mesh::new_rectangle(
            ctx,
            name_input_draw_mode,
            *self.element_rects.get("NAME_INPUT").unwrap(),
            Color::BLACK,
        )?;

        canvas.draw(&name_input, DrawParam::default());

        // label
        let mut name_input_label = Text::new("NAME");
        name_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &name_input_label,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.width / 2.0,
                y - 5.0, // 200.
            ))
            .color(Color::BLACK),
        );

        //text
        let mut name_input_text = Text::new(self.name.contents());
        name_input_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Begin,
        });

        canvas.draw(
            &name_input_text,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.padding,
                y + (self.input_dimensions.height / 2.0),
            ))
            .color(Color::BLACK),
        );

        Ok(())
    }

    pub fn draw_ip_input(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
    ) -> GameResult {
        let input_gap = 75.0;
        let ip_input_draw_mode = if self.ip_input_active {
            DrawMode::stroke(3.0)
        } else {
            DrawMode::stroke(1.0)
        };
        // input
        let ip_input = graphics::Mesh::new_rectangle(
            ctx,
            ip_input_draw_mode,
            *self.element_rects.get("IP_INPUT").unwrap(),
            Color::BLACK,
        )?;

        canvas.draw(&ip_input, DrawParam::default());
        // label
        let mut ip_input_label = Text::new("IP ADDRESS");
        ip_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &ip_input_label,
            DrawParam::from(Vec2::new(
                self.input_dimensions.horizontal_offset + self.input_dimensions.width / 2.0,
                y - 5.0,
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
                y + (self.input_dimensions.height / 2.0),
            ))
            .color(Color::BLACK),
        );

        Ok(())
    }

    pub fn draw_join_game_button(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
    ) -> GameResult {
        let mut join_game_text = Text::new("Join game");
        join_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &join_game_text,
            DrawParam::from(Vec2::new(
                self.button_dimensions.horizontal_offset + self.button_dimensions.width / 2.0,
                y + self.button_dimensions.height / 2.0,
            ))
            .color(Color::BLACK),
        );

        let join_game_btn = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            *self.element_rects.get("JOIN_GAME").unwrap(),
            Color::BLACK,
        )?;

        canvas.draw(&join_game_btn, DrawParam::default());

        Ok(())
    }

    pub fn draw_create_game_button(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
    ) -> GameResult {
        let mut create_game_text = Text::new("Create game");
        create_game_text.set_layout(TextLayout {
            v_align: TextAlign::Middle,
            h_align: TextAlign::Middle,
        });
        canvas.draw(
            &create_game_text,
            DrawParam::from(Vec2::new(
                self.button_dimensions.horizontal_offset + self.button_dimensions.width / 2.0,
                y + self.button_dimensions.height / 2.0,
            ))
            .color(Color::BLACK),
        );

        let create_game_btn = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            *self.element_rects.get("CREATE_GAME").unwrap(),
            Color::BLACK,
        )?;

        canvas.draw(&create_game_btn, DrawParam::default());

        Ok(())
    }

    pub fn draw_back_arrow_img(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
    ) -> GameResult {
        let img_box = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            *self.element_rects.get("BACK_ARROW_IMG").unwrap(),
            Color::BLACK,
        )?;
        canvas.draw(&img_box, DrawParam::default());
        canvas.draw(
            &self.back_arrow_image,
            DrawParam::from(Vec2::new(100.0, 100.0)).scale([0.1, 0.1]),
        );

        Ok(())
    }
}
