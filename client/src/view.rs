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
pub enum View {
    Game,
    MainMenu(Vec<Element>),
    JoinGame,
    CreateGame,
}

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub rect: Rect,
}
impl View {
    pub fn new() -> Self {
        View::create_main_menu()
    }

    pub fn create_main_menu() -> Self {
        let mut elems = Vec::new();

        let button_width = 125.0;
        let button_height = 35.0;
        let button_horizontal_offset = (SCREEN_WIDTH - button_width) * 0.5;
        let buttons_gap = 75.0;

        elems.push(Element {
            name: "CREATE_GAME".to_string(),
            rect: graphics::Rect::new(button_horizontal_offset, 200.0, button_width, button_height),
        });
        elems.push(Element {
            name: "JOIN_GAME".to_string(),
            rect: graphics::Rect::new(
                button_horizontal_offset,
                200.0 + buttons_gap,
                button_width,
                button_height,
            ),
        });

        View::MainMenu(elems)
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.draw_frame(canvas, ctx)?;
        Ok(())
    }
    fn draw_frame(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        match &self {
            View::Game => View::draw_game(canvas, ctx),
            View::MainMenu(data) => View::draw_main_menu(canvas, ctx, data),
            View::JoinGame => View::draw_join_game(canvas, ctx),
            View::CreateGame => View::draw_create_game(canvas, ctx),
        }
    }

    fn draw_join_game(canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
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

    fn draw_main_menu(
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        data: &Vec<Element>,
    ) -> GameResult {
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
        for button in data {
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
