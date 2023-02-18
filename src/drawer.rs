use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use ggez::{
    glam::Vec2,
    graphics::{
        self, Color, DrawMode, DrawParam, Drawable, Image, PxScale, Rect, Text, TextAlign,
        TextFragment, TextLayout,
    },
    Context, GameResult, mint,
};

pub struct Button {
    pub width: f32,
    pub height: f32,
    pub horizontal_offset: f32,
}

pub struct Input {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub horizontal_offset: f32,
}
pub struct Drawer {
    pub input_dimensions: Input,
    pub button_dimensions: Button,
    pub back_arrow_image: Image,
}

impl Drawer {
    pub fn new(ctx: &mut Context) -> GameResult<Drawer> {
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

        Ok(Drawer {
            input_dimensions,
            button_dimensions,
            back_arrow_image: Image::from_path(ctx, "/back-arrow.png")?,
        })
    }

    pub fn draw_name_input(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
        name_input_text: String,
        name_input_active: bool,
        name_input_rect: Rect,
    ) -> GameResult {
        let name_input_draw_mode = if name_input_active {
            DrawMode::stroke(3.0)
        } else {
            DrawMode::stroke(1.0)
        };
        // input box
        let name_input = graphics::Mesh::new_rectangle(
            ctx,
            name_input_draw_mode,
            name_input_rect,
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
                y - 5.0,
            ))
            .color(Color::BLACK),
        );

        // input box text
        let mut name_input_text = Text::new(name_input_text);
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

    pub fn draw_join_game_button(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
        join_game_rect: Rect,
    ) -> GameResult {
        // button rect
        let join_game_btn = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            join_game_rect,
            Color::BLACK,
        )?;
        canvas.draw(&join_game_btn, DrawParam::default());

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
                y + self.button_dimensions.height / 2.0,
            ))
            .color(Color::BLACK),
        );

        Ok(())
    }

    pub fn draw_create_game_button(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
        create_game_rect: Rect,
    ) -> GameResult {
        // button rect
        let create_game_btn = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            create_game_rect,
            Color::BLACK,
        )?;
        canvas.draw(&create_game_btn, DrawParam::default());

        // button text
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

        Ok(())
    }

    pub fn draw_back_arrow_img(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        back_arrow_rect: Rect,
    ) -> GameResult {
        let img = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            back_arrow_rect,
            Color::BLACK,
        )?;
        canvas.draw(&img, DrawParam::default());
        canvas.draw(
            &self.back_arrow_image,
            DrawParam::from(Vec2::new(100.0, 100.0)).scale([0.1, 0.1]),
        );

        Ok(())
    }

    pub fn draw_ip_input(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        y: f32,
        ip_input_active: bool,
        ip_address: String,
        ip_input_rect: Rect,
    ) -> GameResult {
        let ip_input_draw_mode = if ip_input_active {
            DrawMode::stroke(3.0)
        } else {
            DrawMode::stroke(1.0)
        };
        // input box
        let ip_input =
            graphics::Mesh::new_rectangle(ctx, ip_input_draw_mode, ip_input_rect, Color::BLACK)?;
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
        //input box text
        let mut ip_input = Text::new(ip_address);
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
    pub fn draw_eye(&self, canvas: &mut graphics::Canvas, ctx: &mut Context)->GameResult{
        let img = ggez::graphics::Image::from_path(ctx, "/eye-front.png")?;
        let dest = mint::Point2{ x: SCREEN_WIDTH/2.0 - 80.0, y: SCREEN_HEIGHT - 150.0 };
        canvas.draw(&img, DrawParam::new()
        .dest(dest)
    );
        Ok(())
    }
}
