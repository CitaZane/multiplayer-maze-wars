use ggez::graphics::{Color, DrawParam, TextAlign, TextLayout};
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::net::{TcpListener, UdpSocket};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::drawer::{Button, Drawer, Input};
use crate::server::{Message, Server};
use crate::view::View;
use ggez::{
    graphics::{self, Rect, Text},
    Context, GameResult,
};

use super::{game::GameStruct, main_menu::MainMenuStruct};

pub struct CreateGameStruct {
    pub element_rects: HashMap<String, Rect>, // holds text input and button rects
    pub name_input_active: bool,
    pub name: Text,
    pub drawer: Drawer,
    pub display_error: bool,
}

impl CreateGameStruct {
    pub fn new(ctx: &mut Context) -> GameResult<CreateGameStruct> {
        let drawer = Drawer::new(ctx)?;
        Ok(CreateGameStruct {
            element_rects: Self::get_elements(&drawer.button_dimensions, &drawer.input_dimensions),
            name_input_active: false,
            name: Text::new(""),
            display_error: false,
            drawer,
        })
    }

    pub fn get_elements(
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

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        if self.display_error {
            let mut text = Text::new("Server already running on this network");
            let create_game_btn_rect = self.element_rects.get("CREATE_GAME").unwrap();
            let text_x = create_game_btn_rect.x + create_game_btn_rect.w / 2.0;
            let text_y = create_game_btn_rect.y + create_game_btn_rect.h + 10.0;
            text.set_layout(TextLayout {
                v_align: TextAlign::Begin,
                h_align: TextAlign::Middle,
            });

            canvas.draw(&text, DrawParam::from([text_x, text_y]).color(Color::RED));
        }
        self.drawer.draw_title(canvas, ctx)?;
        self.drawer.draw_name_input(
            canvas,
            ctx,
            200.0,
            self.name.contents(),
            self.name_input_active,
            *self.element_rects.get("NAME_INPUT").unwrap(),
        )?;

        self.drawer.draw_back_arrow_img(
            canvas,
            ctx,
            *self.element_rects.get("BACK_ARROW_IMG").unwrap(),
        )?;

        self.drawer.draw_create_game_button(
            canvas,
            ctx,
            275.0,
            *self.element_rects.get("CREATE_GAME").unwrap(),
        )?;
        self.drawer
            .draw_fps_counter(canvas, ctx)
            .expect("Cant draw fps counter.");
        Ok(())
    }

    pub fn check_mouse_click(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        ctx: &mut Context,
        send_ch: Sender<Message>,
    ) -> Option<View> {
        let mut new_view = None;

        for (name, elem_rect) in &self.element_rects {
            if mouse_x > elem_rect.x && mouse_x < elem_rect.x + elem_rect.w && mouse_y > elem_rect.y
            {
                if name == "NAME_INPUT" {
                    self.name_input_active = true;
                } else if name == "CREATE_GAME" {
                    match UdpSocket::bind("192.168.1.126:35353") {
                        Ok(_) => {
                            println!("blabla");
                            new_view = Some(View::Game(GameStruct::new(ctx).unwrap()));
                        }
                        Err(_) => {
                            self.display_error = true;
                            println!("port in use");
                        }
                    }

                    // match portpicker::is_free(35353) {
                    //     true => {
                    //         println!("blabla");
                    //         new_view = Some(View::Game(GameStruct::new(ctx).unwrap()));
                    //     }
                    //     false => {
                    //         println!("port in use");
                    //     }
                    // }
                    break;
                } else if name == "BACK_ARROW_IMG" {
                    new_view = Some(View::MainMenu(MainMenuStruct::new(ctx).unwrap()));
                }
            }
        }

        new_view
    }
}
