use ggez::{
    graphics::{self, Rect, Text},
    Context, GameResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};

use crate::{
    drawer::{Button, Drawer, Input},
    main_menu::MainMenuStruct,
    state::Map,
    view::View,
};

pub struct CreateMap {
    element_rects: HashMap<String, Rect>,
    pub name_input_active: bool,
    pub name: Text,
    map: Map,
    drawer: Drawer,
}

impl CreateMap {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let drawer = Drawer::new(ctx)?;
        Ok(CreateMap {
            element_rects: Self::get_elements(&drawer.button_dimensions, &drawer.input_dimensions),
            name: Text::new(""),
            name_input_active: false,
            map: Map::empty_map(ctx),
            drawer,
        })
    }
    fn save_map(&self) -> Result<(), Error> {
        let path = format!("maps/{}.txt", self.name.contents());
        let mut output = File::create(path)?;
        for line in self.map.maze.iter() {
            let res = "".to_string();
            let l = line.iter().fold(res, |acc, &num| format!("{acc}{num}"));
            let l = format!("{}", l);
            writeln!(output, "{l}")?;
        }
        Ok(())
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.map.draw(canvas)?;
        self.drawer
            .draw_fps_counter(canvas, ctx)
            .expect("Cant draw fps counter.");
        self.drawer.draw_back_arrow_img(
            canvas,
            ctx,
            *self.element_rects.get("BACK_ARROW_IMG").unwrap(),
        )?;
        self.drawer.draw_name_input(
            canvas,
            ctx,
            200.0,
            self.name.contents(),
            self.name_input_active,
            *self.element_rects.get("NAME_INPUT").unwrap(),
        )?;
        self.drawer.draw_save_map_button(
            canvas,
            ctx,
            275.0,
            *self.element_rects.get("SAVE_MAP").unwrap(),
        )?;
        Ok(())
    }
    pub fn get_elements(
        button_dimensions: &Button,
        input_dimensions: &Input,
    ) -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        elems.insert(
            "BACK_ARROW_IMG".to_string(),
            graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
        );
        elems.insert(
            "SAVE_MAP".to_string(),
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

        elems
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
                } else if name == "BACK_ARROW_IMG" {
                    new_view = Some(View::MainMenu(MainMenuStruct::new(ctx).unwrap()));
                } else if name == "SAVE_MAP" {
                    self.save_map().expect("Cant save map");

                    new_view = Some(View::MainMenu(MainMenuStruct::new(ctx).unwrap()));
                }
            }
        }

        new_view
    }
    pub fn register_click(&mut self, mouse_x: f32, mouse_y: f32, ctx: &mut Context) {
        // bottom left corner x and y
        let (x, y, len) = self.map.get_map_corner_and_len();
        let height = self.map.tile_size * self.map.height;
        if mouse_x > x + len || mouse_x < x {
            return;
        }
        if mouse_y > y || mouse_y < y - height {
            return;
        }
        let map_x = ((mouse_x - self.map.h_offset()) / self.map.tile_size) as usize;
        let map_y = ((mouse_y - self.map.v_offset()) / self.map.tile_size) as usize;
        if map_x == 0
            || map_y == 0
            || map_x == self.map.maze[0].len() - 1
            || map_y == self.map.maze.len() - 1
        {
            return;
        }
        if self.map.maze[map_y][map_x] == 0 {
            self.map.maze[map_y][map_x] = 1
        } else {
            self.map.maze[map_y][map_x] = 0
        }
        self.map.register_graphics(ctx);
    }
}
