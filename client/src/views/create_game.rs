use std::process::Command;
use std::thread;
use std::{collections::HashMap, net::UdpSocket};
use local_ip_address::local_ip;

use ggez::{
    graphics::{self, Rect, Text},
    Context, GameResult,
};

use crate::{
    drawer::{Button, Drawer, Input},
    View,
};

use super::{game::GameStruct, main_menu::MainMenuStruct};

pub struct CreateGameStruct {
    pub element_rects: HashMap<String, Rect>, // holds text input and button rects
    pub name_input_active: bool,
    pub name: Text,
    pub drawer: Drawer,
}

impl CreateGameStruct {
    pub fn new(ctx: &mut Context) -> GameResult<CreateGameStruct> {
        let drawer = Drawer::new(ctx)?;
        Ok(CreateGameStruct {
            element_rects: Self::get_elements(&drawer.button_dimensions, &drawer.input_dimensions),
            name_input_active: false,
            name: Text::new(""),
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
                if name == "NAME_INPUT" {
                    self.name_input_active = true;
                } else if name == "CREATE_GAME" {
                    let output = Command::new("./start_server.sh").output();
                    println!("{:?}", output);
                    //_ = self.connect();
                    new_view = Some(View::Game(GameStruct::new(ctx).unwrap()));
                } else if name == "BACK_ARROW_IMG" {
                    new_view = Some(View::MainMenu(MainMenuStruct::new(ctx).unwrap()));
                }
            }
        }

        new_view
    }

    pub fn connect(&self) -> Result<UdpSocket, std::io::Error>{
        let my_local_ip = local_ip().unwrap();

        let socket = UdpSocket::bind(my_local_ip.to_string().to_owned() + ":0")?;

        // here we need to send to server address
        socket
            .send_to("client connected".as_bytes(), my_local_ip.to_string() + ":34254")
            .expect("Error on send");

        // create buffer to save the socket message to
        let mut buf = [0; 2048];

        // load the message from the server to buffer and panic if any error happens
        socket.recv_from(&mut buf).expect("Didnt receive any data");

        Ok(socket)
    }

    // pub fn server(&self) -> std::io::Result<()> {
    //     let my_local_ip = local_ip().unwrap();
    //     let socket = UdpSocket::bind(my_local_ip.to_string() + ":34254")?; // for UDP4/6

    //     let mut buf = [0; 2048];
    //     println!("Server started at: {}", my_local_ip.to_string() + ":34254");
    //     loop {
    //         // Receives a single datagram message on the socket.
    //         // If `buf` is too small to hold
    //         // the message, it will be cut off.
    //         let (amt, src) = socket.recv_from(&mut buf)?;
    //         let echo = std::str::from_utf8(&buf[..amt]).unwrap();
    //         println!("Message: {}", echo);
    //         // Redeclare `buf` as slice of the received data
    //         // and send data back to origin.
    //         let buf = &mut buf[..amt];
    //         socket.send_to(buf, &src)?;
    //     }
    // }
}
