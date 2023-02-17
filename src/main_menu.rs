use std::collections::HashMap;

use ggez::{
    graphics::{self, Rect},
    Context, GameResult,
};
use crate::view::View2;

use crate::{
    drawer::{Button, Drawer},
    
};

use super::{create_game::CreateGameStruct, join_game::JoinGameStruct};

pub struct MainMenuStruct {
    pub element_rects: HashMap<String, Rect>, // holds text input and button rects
    pub drawer: Drawer,
}

impl MainMenuStruct {
    pub fn new(ctx: &mut Context) -> GameResult<MainMenuStruct> {
        let drawer = Drawer::new(ctx)?;
        Ok(MainMenuStruct {
            element_rects: Self::get_elements(&drawer.button_dimensions),
            drawer,
        })
    }

    pub fn get_elements(button_dimensions: &Button) -> HashMap<String, Rect> {
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

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.drawer.draw_title(canvas, ctx)?;

        self.drawer.draw_create_game_button(
            canvas,
            ctx,
            200.0,
            *self.element_rects.get("CREATE_GAME").unwrap(),
        )?;

        self.drawer.draw_join_game_button(
            canvas,
            ctx,
            275.0,
            *self.element_rects.get("JOIN_GAME").unwrap(),
        )?;

        Ok(())
    }

    pub fn check_mouse_click(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        ctx: &mut Context,
    ) -> Option<View> {
        let mut new_view = None;
        for (name, elem_rect) in &self.element_rects {
            if mouse_x > elem_rect.x
                && mouse_x < elem_rect.x + elem_rect.w
                && mouse_y > elem_rect.y
                && mouse_y < elem_rect.y + elem_rect.h
            {
                if name == "CREATE_GAME" {
                    new_view = Some(View::CreateGame(CreateGameStruct::new(ctx).unwrap()));
                } else if name == "JOIN_GAME" {
                    new_view = Some(View::JoinGame(JoinGameStruct::new(ctx).unwrap()));
                }
            }
        }
        new_view
    }
}
