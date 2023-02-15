use ggez::glam::Vec2;
use vect::vector2::Vector2;
pub use crate::map::Map;
pub use crate::player::Player;
pub use crate::view::View;
use crate::SCREEN_WIDTH;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
pub const VIEWPORT_WIDTH: f32 = 370.0;
pub const VIEWPORT_HEIGHT: f32 = 410.0;
pub struct Game {
    // Your state here...
    // viewport
    map: Map,
    view: View,
    player: Player,
    opponents:Vec<Player>,
}
// 17 x 33
impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        let opponent = Player::new();
        // Load/create resources such as images here.
        Self {
            map: Map::new(),
            view: View::new(),
            player: Player::new(),
            opponents:vec![opponent],
            // ...
        }
    }
    fn draw_scene(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let maze = self.map.0.as_ref().unwrap();
        let mut last_side = 0;
        let mut last_height: f32 = 0.;
        let mut buffer:Vec<f32> = vec![]; //used for drawing opponents
        // calculate rays for ech pixel in horizontal direction
        for i in 0..VIEWPORT_WIDTH as i32 {
            let camera_x = (2 * i) as f64 / VIEWPORT_WIDTH as f64 - 1.0;
            let ray_dir_x = self.player.dir.x + self.player.camera_plane.x * camera_x;
            let ray_dir_y = self.player.dir.y + self.player.camera_plane.y * camera_x;

            //which box of the map we're in
            let mut map_x = self.player.pos.x as i32;
            let mut map_y = self.player.pos.y as i32;

            //length of ray from current position to next x or y-side
            let mut side_dist_x = 0.0;
            let mut side_dist_y = 0.0;

            //length of ray from one x or y-side to next x or y-side
            let delta_dist_x = (1. / ray_dir_x).abs();
            let delta_dist_y = (1. / ray_dir_y).abs();
            let mut prep_wall_dist = 0.0;

            let mut step_x = 0;
            let mut step_y = 0;

            let mut hit = 0;
            let mut side = 0;
            let mut edge = false;

            //calculate step and initial sideDist
            let player_x = self.player.pos.x + 0.5; //center player in square
            let player_y = self.player.pos.y + 0.5; //center player in square
            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (player_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - player_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (player_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - player_y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
                if map_x as usize >= maze[0].len() {
                    map_x -= 1
                }
                if map_y as usize >= maze.len() {
                    map_y -= 1
                }
                if maze[map_y as usize][map_x as usize] > 0 {
                    hit = 1;
                    if side == 0 {
                        prep_wall_dist = side_dist_x - delta_dist_x;
                    } else {
                        prep_wall_dist = side_dist_y - delta_dist_y;
                    }
                    if last_side != side {
                        edge = true;
                    }
                }
            }

            //Calculate height of line to draw on screen
            let line_height = VIEWPORT_HEIGHT / prep_wall_dist as f32;
            let mut side_type = 1;
            if self.player.dir.x == 0.{side_type =0}
            if !edge && side != side_type{
                if line_height.round() != last_height.round() {
                    edge = true
                }
            }else if !edge && side == side_type{
                if (line_height.round() - last_height.round()).abs() > 5.0 {
                    edge = true
                }
            }
            // draw the walls
            self.draw_walls(canvas, ctx, line_height, i)?;
            if edge {
                self.draw_edge(canvas, ctx, line_height, last_height, i)?;
            }

            buffer.push(Game::calc_bottom_point(line_height));
            last_height = line_height;
            last_side = side;
        }
        self.draw_opponents(canvas,ctx, buffer)?;
        Ok(())
    }
    fn draw_opponents(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context, buffer:Vec<f32>) -> GameResult{
        let x_offset = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
        let y_offset = 20.0;
        for i in 0..self.opponents.len(){
             //translate sprite position to relative to camera
            let sprite_pos = self.opponents[i].pos - self.player.pos;
            //transform sprite with the inverse camera matrix
            let inv_det = 1.0 / (self.player.camera_plane.x * self.player.dir.y - self.player.dir.x * self.player.camera_plane.y);

            let transform_x = inv_det * (self.player.dir.y * sprite_pos.x - self.player.dir.x * sprite_pos.y);
            let transform_y = inv_det * (-self.player.camera_plane.y * sprite_pos.x + self.player.camera_plane.x * sprite_pos.y); //depth
            let sprite_screen_x = (VIEWPORT_WIDTH as f64/ 2.0) * (1.+ transform_x/transform_y);

            // calc the height of the sprite plane
            let h = 165.0;
            let sprite_height = (VIEWPORT_HEIGHT as f64 /transform_y).abs() as f32;
            let sprite_y_start = -sprite_height / 2.0 + VIEWPORT_HEIGHT as f32 / 2.0;
            let sprite_y_end = sprite_height / 2. + VIEWPORT_HEIGHT /2.0;

            let sprite_x_start =  -sprite_height / 2.0 + sprite_screen_x  as f32;
            let sprite_x_end = sprite_height / 2.0 + sprite_screen_x as f32;
            
            let scaled_size = (sprite_y_end - sprite_y_start) * h / VIEWPORT_HEIGHT;
            let x = (sprite_x_start  + sprite_x_end) / 2. as f32 + x_offset - scaled_size / 2.0;
            let y = sprite_y_end as f32 + y_offset - scaled_size ;
            if transform_y > 0.0 && sprite_x_start > 0.0 && sprite_x_end < VIEWPORT_WIDTH+x_offset && buffer[(x  - x_offset) as usize]+y_offset < y+ scaled_size{
                let player_img = graphics::Image::from_path(ctx, "/eye-front.png")?;
                let scale = scaled_size / player_img.height() as f32 * 1.2;
                canvas.draw(&player_img, DrawParam::default()
                .dest([x - scaled_size*0.15 , y])
                .scale([scale,scale]));
                // let point = graphics::Rect::new(x, y, scaled_size, scaled_size);

                // let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), point, Color::RED)?;
                // canvas.draw(&mesh, DrawParam::default());   
            }
            }
        Ok(())
    }
    fn calc_up_point(line_height: f32) -> f32 {
        let mut draw_start = -line_height / 2. + VIEWPORT_HEIGHT / 2.;
        if draw_start < 0.0 {
            draw_start = 0.0;
        }
        draw_start
    }
    fn calc_bottom_point(line_height: f32) -> f32 {
        let mut draw_end = line_height / 2. + VIEWPORT_HEIGHT / 2.;
        if draw_end >= VIEWPORT_HEIGHT {
            draw_end = VIEWPORT_HEIGHT - 1.0
        }
        draw_end
    }
    fn draw_walls(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        wall_height: f32,
        line: i32,
    ) -> GameResult {
        //calculate lowest and highest pixel to fill in current stripe
        let start_point = Game::calc_up_point(wall_height);
        let end_point = Game::calc_bottom_point(wall_height);
        let x = line as f32 + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
        let y_offset = 20.0;

        self.draw_point(canvas, ctx, x, end_point + y_offset)?;
        self.draw_point(canvas, ctx, x, start_point + y_offset)?;
        Ok(())
    }
    fn draw_edge(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        wall_height: f32,
        previous_height: f32,
        line: i32,
    ) -> GameResult {
        let y_offset = 20.0;

        if previous_height < wall_height {
            let x = line as f32 + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
            let end_point = Game::calc_bottom_point(wall_height);
            let start_point = Game::calc_up_point(wall_height);
            let points = &[
                Vec2::new(x, start_point + y_offset),
                Vec2::new(x, end_point + y_offset),
            ];
            self.draw_line(canvas, ctx, points)?;
        } else {
            let x = line as f32 - 1. + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
            let end_point = Game::calc_bottom_point(previous_height);
            let start_point = Game::calc_up_point(previous_height);
            let points = &[
                Vec2::new(x, start_point + y_offset),
                Vec2::new(x, end_point + y_offset),
            ];
            self.draw_line(canvas, ctx, points)?;
        }

        Ok(())
    }
    fn draw_point(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
    ) -> GameResult {
        let point = graphics::Rect::new(x, y, 1.0, 1.0);
        let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), point, Color::BLACK)?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
    fn draw_line(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
        points: &[Vec2],
    ) -> GameResult {
        let mesh = Mesh::new_polyline(ctx, DrawMode::stroke(1.), points, Color::BLACK)?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
    fn draw_player_pos(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let arrow_img = graphics::Image::from_path(ctx, "/arrow.png")?;
        let (x, y) = self.map.get_coordinates_for_pos(&self.player.pos);
        let rot = self.player.get_rotation();
        let (x_comp, y_comp)=self.player.get_rotation_compensaion();
        let scale = 0.6;
        let size = arrow_img.height();
        let x = x + size as f32*scale  * x_comp;
        let y = y + size as f32*scale  * y_comp;
        canvas.draw(&arrow_img, DrawParam::default()
        .dest([x , y])
        .scale([scale,scale])
        .rotation(rot));
        Ok(())
    }
}
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        // if ctx.keyboard.is_key_pressed(KeyCode::Right) {
        //     self.player.pos += 1;
        // }
        // if ctx.keyboard.is_key_pressed(KeyCode::Left) {
        //     self.player.position.coordinates.x -= 1;
        // }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            match self.player.dir {
                Vector2 { x: 1., y: 0. } => {
                    // Looking Right
                    if self.player.pos.x < (self.map.0.clone().unwrap()[0].len() - 1) as f64 {
                        if self.map.0.clone().unwrap()[self.player.pos.y as usize]
                            [(self.player.pos.x + 1.) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.x += 1.
                        }
                    }
                }
                Vector2 { x: 0., y: 1. } => {
                    // Down
                    if self.player.pos.x < (self.map.0.clone().unwrap().len() - 1) as f64 {
                        if self.map.0.clone().unwrap()[(self.player.pos.y + 1.) as usize]
                            [(self.player.pos.x) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.y += 1.;
                        }
                    }
                }
                Vector2 { x: -1., y: 0. } => {
                    //  Left
                    if self.player.pos.x >= 1. {
                        if self.map.0.clone().unwrap()[self.player.pos.y as usize]
                            [(self.player.pos.x - 1.) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.x -= 1.
                        }
                    }
                }
                Vector2 { x: 0., y: -1. } => {
                    // Up
                    if self.player.pos.y >= 1. {
                        if self.map.0.clone().unwrap()[(self.player.pos.y - 1.) as usize]
                            [(self.player.pos.x) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.y -= 1.;
                        }
                    }
                }
                _ => (),
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            match self.player.dir {
                Vector2 { x: 1., y: 0. } => {
                    // Right
                    if self.player.pos.x >= 1. {
                        if self.map.0.clone().unwrap()[self.player.pos.y as usize]
                            [(self.player.pos.x - 1.) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.x -= 1.
                        }
                    }
                }
                Vector2 { x: 0., y: 1. } => {
                    // Down
                    if self.player.pos.y >= 1. {
                        if self.map.0.clone().unwrap()[(self.player.pos.y - 1.) as usize]
                            [(self.player.pos.x) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.y -= 1.;
                        }
                    }
                }
                Vector2 { x: -1., y: 0. } => {
                    //  Left
                    if self.player.pos.x < (self.map.0.clone().unwrap()[0].len() - 1) as f64 {
                        if self.map.0.clone().unwrap()[self.player.pos.y as usize]
                            [(self.player.pos.x + 1.) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.x += 1.
                        }
                    }
                }
                Vector2 { x: 0., y: -1. } => {
                    // Up
                    if self.player.pos.y < (self.map.0.clone().unwrap().len() - 1) as f64 {
                        if self.map.0.clone().unwrap()[(self.player.pos.y + 1.) as usize]
                            [(self.player.pos.x) as usize]
                            == 1
                        {
                            return Ok(());
                        } else {
                            self.player.pos.y += 1.;
                        }
                    }
                }
                _ => (),
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            match self.player.dir {
                Vector2 { x: 1., y: 0. } => {
                    // Looking Right
                    self.player.dir = Vector2 { x: 0., y: -1. };
                    self.player.camera_plane = Vector2 { x: 0.65, y: 0.0 }
                }
                Vector2 { x: 0., y: 1. } => {
                    // Down
                    self.player.dir = Vector2 { x: 1., y: 0. };
                    self.player.camera_plane = Vector2 { x: 0.0, y: 0.65 }
                }
                Vector2 { x: -1., y: 0. } => {
                    // Left
                    self.player.dir = Vector2 { x: 0., y: 1. };
                    self.player.camera_plane = Vector2 { x: -0.65, y: 0. }
                }
                Vector2 { x: 0., y: -1. } => {
                    // Up
                    self.player.dir = Vector2 { x: -1., y: 0. };
                    self.player.camera_plane = Vector2 { x: 0., y: -0.65 }
                }
                _ => (),
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            match self.player.dir {
                Vector2 { x: 1., y: 0. } => {
                    // Looking Right
                    self.player.dir = Vector2 { x: 0., y: 1. };
                    self.player.camera_plane = Vector2 { x: -0.65, y: 0. }
                }
                Vector2 { x: 0., y: 1. } => {
                    // Down
                    self.player.dir = Vector2 { x: -1., y: 0. };
                    self.player.camera_plane = Vector2 { x: 0., y: -0.65 }
                }
                Vector2 { x: -1., y: 0. } => {
                    // Left
                    self.player.dir = Vector2 { x: 0., y: -1. };
                    self.player.camera_plane = Vector2 { x: 0.65, y: 0.0 }
                }
                Vector2 { x: 0., y: -1. } => {
                    // Up
                    self.player.dir = Vector2 { x: 1., y: 0. };
                    self.player.camera_plane = Vector2 { x: 0.0, y: 0.65 }
                }

                _ => (),
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        self.map.draw(&mut canvas, ctx)?;
        self.view.draw(&mut canvas, ctx)?;
        self.draw_scene(&mut canvas, ctx)?;
        self.draw_player_pos(&mut canvas, ctx)?;
        canvas.finish(ctx)
    }
}
