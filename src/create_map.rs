use std::collections::HashMap;

use ggez::{
    graphics::{self, Rect},
    Context, GameResult,
};

use crate::{
    drawer::Drawer,
    main_menu::MainMenuStruct,
    state::Map,
    view::View,
};

pub struct CreateMap {
    element_rects: HashMap<String, Rect>,
    map: Map,
    drawer: Drawer,
}

impl CreateMap {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let drawer = Drawer::new(ctx)?;
        Ok(CreateMap {
            element_rects: Self::get_elements(),
            map: Map::empty_map(ctx),
            drawer,
        })
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
        Ok(())
    }
    pub fn get_elements() -> HashMap<String, Rect> {
        let mut elems = HashMap::new();
        elems.insert(
            "BACK_ARROW_IMG".to_string(),
            graphics::Rect::new(100.0 - 6.0, 100.0 - 6.0, 256.0 * 0.15, 256.0 * 0.15),
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
            if mouse_x > elem_rect.x && mouse_x < elem_rect.x + elem_rect.w && mouse_y > elem_rect.y && mouse_y < elem_rect.y + elem_rect.h
            {
                println!("Click back {mouse_x} - {mouse_y} , {} - {}",elem_rect.w, elem_rect.h);
                if name == "BACK_ARROW_IMG" {
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
        if self.map.maze[map_y][map_x] ==0 {
            self.map.maze[map_y][map_x] = 1
        }else{
            self.map.maze[map_y][map_x] = 0
        }
        self.map.register_graphics(ctx);
    }
}
