use std::collections::HashMap;

use crate::map::Map;
use crate::player::Direction;
use crate::player::Player;
use crate::SCREEN_WIDTH;
use crate::VIEWPORT_HEIGHT;
use crate::VIEWPORT_WIDTH;
use ggez::glam::Vec2;
use ggez::graphics::Image;
use ggez::graphics::{
    self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, PxScale, Text, TextFragment,
};
use ggez::{Context, GameResult};

const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;

pub struct GameStruct {
    pub map: Map,
    pub player: Player,
    pub opponents: Vec<Player>,

    opponent_img: HashMap<Direction, Image>,
    players_last_pos: Vec2,
    players_last_dir: Direction,
    scene: MeshBuilder,
    buffer: Vec<f32>,
}
// 17 x 33
impl GameStruct {
    pub fn new(ctx: &mut Context, player_name:String) -> GameResult<Self> {
        Ok(Self {
            map: Map::new(ctx),
            player: Player::new(player_name),
            opponents: vec![],
            opponent_img: GameStruct::upload_opponet_images(ctx),
            players_last_pos: Vec2 { x: 0.0, y: 0.0 },
            players_last_dir: Direction::Up,
            scene: MeshBuilder::new(),
            buffer: vec![],
        })
    }
    fn upload_opponet_images(ctx: &mut Context) -> HashMap<Direction, Image> {
        let mut images = HashMap::new();
        let img_back = graphics::Image::from_path(ctx, "/eye-back.png").expect("Missing eye image");
        images.insert(Direction::Up, img_back);
        let img_front =
            graphics::Image::from_path(ctx, "/eye-front.png").expect("Missing eye image");
        images.insert(Direction::Down, img_front);
        let img_left = graphics::Image::from_path(ctx, "/eye-left.png").expect("Missing eye image");
        images.insert(Direction::Left, img_left);
        let img_right =
            graphics::Image::from_path(ctx, "/eye-right.png").expect("Missing eye image");
        images.insert(Direction::Right, img_right);
        images
    }
    pub fn draw(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        if self.players_last_pos != self.player.pos || self.player.dir != self.players_last_dir {
            self.trace_scene()?;
        }
        self.draw_opponents(canvas, ctx)?;
        self.map.draw(canvas, &self.player)?;
        // Helper for displaying opponents on map
        self.map.draw_opponents(ctx,canvas, &self.opponents)?;
        self.draw_fps_counter(canvas, ctx)?;
        //draw 3D scene
        let mesh = Mesh::from_data(ctx, self.scene.build());
        canvas.draw(&mesh, DrawParam::default());
        //update last position stats
        self.players_last_pos = self.player.pos;
        self.players_last_dir = self.player.dir.clone();
        Ok(())
    }
    fn draw_opponents(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let x_offset = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
        let y_offset = 20.0;
        let player_dir = self.player.dir.vec();
        for i in 0..self.opponents.len() {
            //translate sprite position to relative to camera
            let sprite_pos = self.opponents[i].pos - self.player.pos;
            //transform sprite with the inverse camera matrix
            let inv_det = 1.0
                / (self.player.camera_plane.x * player_dir.y
                    - player_dir.x * self.player.camera_plane.y);

            let transform_x = inv_det * (player_dir.y * sprite_pos.x - player_dir.x * sprite_pos.y);
            let transform_y = inv_det
                * (-self.player.camera_plane.y * sprite_pos.x
                    + self.player.camera_plane.x * sprite_pos.y); //depth
            let sprite_screen_x = (VIEWPORT_WIDTH as f32 / 2.0) * (1. + transform_x / transform_y);

            // calc the height of the sprite plane
            let h = 165.0;
            let sprite_height = (VIEWPORT_HEIGHT as f32 / transform_y).abs() as f32;
            let sprite_y_start = -sprite_height / 2.0 + VIEWPORT_HEIGHT as f32 / 2.0;
            let sprite_y_end = sprite_height / 2. + VIEWPORT_HEIGHT / 2.0;

            let sprite_x_start = -sprite_height / 2.0 + sprite_screen_x as f32;
            let sprite_x_end = sprite_height / 2.0 + sprite_screen_x as f32;

            let scaled_size = (sprite_y_end - sprite_y_start) * h / VIEWPORT_HEIGHT;
            let x = (sprite_x_start + sprite_x_end) / 2. as f32 + x_offset - scaled_size / 2.0;
            let y = sprite_y_end as f32 + y_offset - scaled_size;
            if transform_y >= 0.0
                && sprite_x_start > 0.0
                && sprite_x_end < VIEWPORT_WIDTH + x_offset
                && self.buffer[(x - x_offset) as usize] + y_offset < y + scaled_size
            {
                // find correct direction
                let player_dir = self.player.get_opponent_direction(&self.opponents[i].dir);
                let player_img = &self.opponent_img[&player_dir];
                // let player_img = graphics::Image::from_path(ctx, "/eye-front.png")?;
                let scale = scaled_size / player_img.height() as f32 * 1.2;
                canvas.draw(
                    player_img,
                    DrawParam::default()
                        .dest([x - scaled_size * 0.15, y])
                        .scale([scale, scale]),
                );
            }
        }
        Ok(())
    }
    fn trace_scene(&mut self) -> GameResult {
        self.scene = MeshBuilder::new();
        let maze = &self.map.maze.clone();
        let mut last_side = 0;
        let mut last_height: f32 = 0.;
        self.buffer = vec![]; //used for drawing opponents
                              // calculate rays for ech pixel in horizontal direction
        for i in 0..VIEWPORT_WIDTH as i32 {
            let camera_x = (2 * i) as f32 / VIEWPORT_WIDTH as f32 - 1.0;
            let ray_dir_x = self.player.dir.vec().x + self.player.camera_plane.x * camera_x;
            let ray_dir_y = self.player.dir.vec().y + self.player.camera_plane.y * camera_x;

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
                side_dist_x = (player_x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f32 + 1.0 - player_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (player_y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f32 + 1.0 - player_y) * delta_dist_y;
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
            if self.player.dir.vec().x == 0. {
                side_type = 0
            }
            if !edge && side != side_type {
                if line_height.round() != last_height.round() {
                    edge = true
                }
            } else if !edge && side == side_type {
                if (line_height.round() - last_height.round()).abs() > 5.0 {
                    edge = true
                }
            }
            // draw the walls
            self.draw_walls(line_height, i)?;
            if edge {
                self.draw_edge(line_height, last_height, i)?;
            }
            self.draw_frame()?;
            self.buffer.push(GameStruct::calc_bottom_point(line_height));
            last_height = line_height;
            last_side = side;
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
    fn draw_frame(&mut self) -> GameResult {
        let frame = graphics::Rect::new(X, Y, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
        self.scene
            .rectangle(DrawMode::stroke(1.0), frame, Color::BLACK)?;
        Ok(())
    }
    fn draw_walls(&mut self, wall_height: f32, line: i32) -> GameResult {
        //calculate lowest and highest pixel to fill in current stripe
        let start_point = GameStruct::calc_up_point(wall_height);
        let end_point = GameStruct::calc_bottom_point(wall_height);
        let x = line as f32 + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
        let y_offset = 20.0;

        self.draw_point(x, end_point + y_offset)?;
        self.draw_point(x, start_point + y_offset)?;
        Ok(())
    }
    fn draw_edge(&mut self, wall_height: f32, previous_height: f32, line: i32) -> GameResult {
        let y_offset = 20.0;

        if previous_height < wall_height {
            let x = line as f32 + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
            let end_point = GameStruct::calc_bottom_point(wall_height);
            let start_point = GameStruct::calc_up_point(wall_height);
            let points = &[
                Vec2::new(x, start_point + y_offset),
                Vec2::new(x, end_point + y_offset),
            ];
            self.draw_line(points)?;
        } else {
            let x = line as f32 - 1. + (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
            let end_point = GameStruct::calc_bottom_point(previous_height);
            let start_point = GameStruct::calc_up_point(previous_height);
            let points = &[
                Vec2::new(x, start_point + y_offset),
                Vec2::new(x, end_point + y_offset),
            ];
            self.draw_line(points)?;
        }

        Ok(())
    }
    fn draw_point(&mut self, x: f32, y: f32) -> GameResult {
        let point = graphics::Rect::new(x, y, 1.0, 1.0);
        self.scene
            .rectangle(DrawMode::fill(), point, Color::BLACK)?;
        Ok(())
    }
    fn draw_line(&mut self, points: &[Vec2]) -> GameResult {
        self.scene.line(points, 1.0, Color::BLACK)?;
        Ok(())
    }
    fn draw_fps_counter(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let counter = ctx.time.fps().trunc();
        let text = Text::new(TextFragment {
            text: counter.to_string(),
            color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(50.0)),
            ..Default::default()
        });
        canvas.draw(&text, DrawParam::default());
        Ok(())
    }
}
