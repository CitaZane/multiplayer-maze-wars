use crate::{SCREEN_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use ggez::{
    glam::Vec2,
    graphics::{
        self, Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text, TextAlign,
        TextFragment, TextLayout,
    },
    Context, GameResult,
};
const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;
pub enum ViewType {
    Game,
    MainMenu,
    JoinGame,
    CreateGame,
}

pub struct View {
    pub current_view: ViewType,
    pub elements: Vec<Element>,
    button_dimensions: Button,
}

pub struct Button {
    width: f32,
    height: f32,
    horizontal_offset: f32,
}

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub rect: Rect,
}
impl View {
    pub fn new() -> Self {
        let button_width = 125.0;
        let button_horizontal_offset = (SCREEN_WIDTH - button_width) * 0.5;
        let button_dimensions = Button {
            width: 125.0,
            height: 35.0,
            horizontal_offset: button_horizontal_offset,
        };

        View {
            current_view: ViewType::JoinGame,
            elements: View::get_join_game_elements(&button_dimensions),
            button_dimensions,
        }
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
    pub fn get_join_game_elements(button_dimensions: &Button) -> Vec<Element> {
        let mut elems = Vec::new();
        let input_width = 200.0;
        let input_height = 30.0;
        let input_horizontal_offset = (SCREEN_WIDTH - input_width) * 0.5;
        let input_gap = 75.0;
        elems.push(Element {
            name: "IP_INPUT".to_string(),
            rect: graphics::Rect::new(input_horizontal_offset, 200.0, input_width, input_height),
        });
        elems.push(Element {
            name: "NAME_INPUT".to_string(),
            rect: graphics::Rect::new(
                input_horizontal_offset,
                200.0 + input_gap,
                input_width,
                input_height,
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

        elems
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.draw_frame(canvas, ctx)?;
        Ok(())
    }
    fn draw_frame(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        match &self.current_view {
            ViewType::Game => View::draw_game(canvas, ctx),
            ViewType::MainMenu => self.draw_main_menu(canvas, ctx),
            ViewType::JoinGame => self.draw_join_game(canvas, ctx),
            ViewType::CreateGame => View::draw_create_game(canvas, ctx),
        }
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

        // button and inputs rects
        for elem in &self.elements {
            let elem =
                graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), elem.rect, Color::BLACK)?;

            canvas.draw(&elem, DrawParam::default())
        }

        // button text
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

        // input
        let input_width = 200.0;
        let input_height = 30.0;
        let input_horizontal_offset = (SCREEN_WIDTH - input_width) * 0.5;
        let input_gap = 75.0;
        // name input text
        let mut name_input_label = Text::new("NAME");
        name_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });

        // ip input text
        let mut ip_input_label = Text::new("IP ADDRESS");
        ip_input_label.set_layout(TextLayout {
            v_align: TextAlign::End,
            h_align: TextAlign::Middle,
        });

        canvas.draw(
            &ip_input_label,
            DrawParam::from(Vec2::new(
                input_horizontal_offset + input_width / 2.0,
                200.0 + input_gap - 5.0,
            ))
            .color(Color::BLACK),
        );
        canvas.draw(
            &name_input_label,
            DrawParam::from(Vec2::new(
                input_horizontal_offset + input_width / 2.0,
                200.0 - 5.0,
            ))
            .color(Color::BLACK),
        );
        // let btn =
        //     graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), button.rect, Color::BLACK)?;

        // canvas.draw(&btn, DrawParam::default());
        Ok(())
    }

    fn draw_create_game(canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw_game(canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
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
