use std::collections::HashMap;

use ggez::{
    graphics::{self, Rect, Text},
    Context, GameResult,
};

use crate::{
    drawer::{Button, Drawer, Input},
    views::create_game::CreateGameStruct,
    View,
};

use super::{game::GameStruct, main_menu::MainMenuStruct};

pub struct JoinGameStruct {
    pub element_rects: HashMap<String, Rect>, // holds text input and button rects
    pub ip_input_active: bool,
    pub name_input_active: bool,
    pub ip_address: Text,
    pub name: Text,
    pub drawer: Drawer,
}

impl JoinGameStruct {
    pub fn new(ctx: &mut Context) -> GameResult<JoinGameStruct> {
        let drawer = Drawer::new(ctx)?;
        Ok(JoinGameStruct {
            element_rects: Self::get_elements(&drawer.button_dimensions, &drawer.input_dimensions),
            ip_input_active: false,
            name_input_active: false,
            ip_address: Text::new(""),
            name: Text::new(""),
            drawer,
        })
    }

    pub fn get_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        let input_gap = 75.0;

        elems.insert(
            "NAME_INPUT".to_string(),
            Rect::new(
                input_dimensions.horizontal_offset,
                200.0,
                input_dimensions.width,
                input_dimensions.height,
            ),
        );
        elems.insert(
            "IP_INPUT".to_string(),
            Rect::new(
                input_dimensions.horizontal_offset,
                200.0 + input_gap,
                input_dimensions.width,
                input_dimensions.height,
            ),
        );
        elems.insert(
            "JOIN_GAME".to_string(),
            Rect::new(
                button_dimensions.horizontal_offset,
                200.0 + (input_gap * 2.0),
                button_dimensions.width,
                button_dimensions.height,
            ),
        );

        elems.insert(
            "BACK_ARROW_IMG".to_string(),
            Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        );

        elems
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.drawer.draw_title(canvas, ctx)?;
        self.drawer.draw_name_input(
            canvas,
            ctx,
            200.0,
            self.name.contents(),
            self.name_input_active,
            *self.element_rects.get("NAME_INPUT").unwrap(),
        )?;

        self.drawer.draw_ip_input(
            canvas,
            ctx,
            275.0,
            self.ip_input_active,
            self.ip_address.contents(),
            *self.element_rects.get("IP_INPUT").unwrap(),
        )?;

        self.drawer.draw_back_arrow_img(
            canvas,
            ctx,
            *self.element_rects.get("BACK_ARROW_IMG").unwrap(),
        )?;

        self.drawer.draw_join_game_button(
            canvas,
            ctx,
            350.0,
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
                if name == "IP_INPUT" {
                    self.ip_input_active = true;
                    if self.name_input_active {
                        self.name_input_active = false;
                    }
                } else if name == "NAME_INPUT" {
                    self.name_input_active = true;
                    if self.ip_input_active {
                        self.ip_input_active = false;
                    }
                } else if name == "JOIN_GAME" {
                    new_view = Some(View::Game(GameStruct::new(ctx).unwrap()));
                    let name = self.name.contents();
                    let server_ip = self.ip_address.contents();

                    CreateGameStruct::connect(server_ip, name).unwrap();

                    // println!("IP address {}", ip_address);
                    // println!("Name {}", name);
                } else if name == "BACK_ARROW_IMG" {
                    new_view = Some(View::MainMenu(MainMenuStruct::new(ctx).unwrap()));
                }
            }
        }
        new_view
    }
}
