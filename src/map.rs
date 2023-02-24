use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::player::Player;
use crate::{SCREEN_WIDTH, VIEWPORT_HEIGHT};
use ggez::graphics::{DrawMode, MeshBuilder};
use ggez::{
    glam::Vec2,
    graphics::{self, Color, DrawParam, Image, Mesh},
    Context, GameResult,
};
// pub const TILE_SIZE: f32 = 9.0;
// pub const MAP_WIDTH: f32 = 33.0;
// pub const H_OFFSET: f32 = (SCREEN_WIDTH - MAP_WIDTH * TILE_SIZE) / 2.0;
// pub const V_OFFSET: f32 = VIEWPORT_HEIGHT + 20.0 * 2.0;
#[derive( Clone, Debug)]
pub struct Map {
    pub maze: Vec<Vec<i32>>,
    pub graphics: Option<Mesh>,
    player_arrow: Image,
    pub tile_size:f32,
    pub width:f32,
    pub height:f32,
}
// Map size 33X17
impl Map {
    pub fn new(ctx: &mut Context, maze:Vec<Vec<i32>>) -> Self {
        // let maze = Map::level_one();
        // let graphics = Map::register_graphics(&maze, ctx);
        let player_arrow = Image::from_path(ctx, "/arrow.png").expect("Arrow image missing");
        let mut map = Map {
            maze,
            graphics:None,
            player_arrow,
            tile_size:9.,
            width:33.,
            height:17.,
        };
        map.register_graphics(ctx);
        map
    }
    pub fn make_from_file(ctx: &mut Context, path:&String)->Map{
        let input = File::open(path).expect("Map not found");
        let buffered = BufReader::new(input);
        let mut map = vec![];
        for line in buffered.lines() {
            let mut row = vec![];
            for tile in line.expect("Invalid line in map").chars(){
                row.push((tile.to_string()).parse::<i32>().unwrap());
            }
            map.push(row);
        }
        Map::new(ctx, map)

    }
    pub fn empty_map(ctx: &mut Context) -> Self {
        let mut maze = vec![vec![0; 33]; 17];
        for row in 0..17 {
            for col in 0..33 {
                if row == 0 || row == 16 || col == 0 || col == 32 {
                    maze[row][col] = 1
                }
            }
        }
        // let graphics = Map::register_graphics(&maze, ctx);
        let player_arrow = Image::from_path(ctx, "/arrow.png").expect("Arrow image missing");
        let mut map = Map {
            maze,
            graphics:None,
            player_arrow,
            tile_size:15.,
            width:33.,
            height:17.,
        };
        map.register_graphics(ctx);
        map
    }
    pub fn h_offset(&self)->f32{
        (SCREEN_WIDTH - self.width * self.tile_size) / 2.0
    }
    pub fn v_offset(&self)->f32{
        VIEWPORT_HEIGHT + 20.0 * 2.0
    }
    fn level_one() -> Vec<Vec<i32>> {
        vec![
            vec![1; 33],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
                1, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0,
                1, 1, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0,
                0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1,
                1, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0,
                0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0,
                1, 1, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 1,
            ],
            vec![
                1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1,
                1, 1, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0,
                0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
                1, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
                1, 1, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 1,
            ],
            vec![1; 33],
        ]
    }
    pub fn get_coordinates_for_pos(&self, pos: &Vec2) -> (f32, f32) {
        let x =self.h_offset() + pos.x as f32 * self.tile_size;
        let y = self.v_offset() + pos.y as f32 * self.tile_size;
        (x, y)
    }
    pub fn get_map_corner_and_len(&self) -> (f32, f32, f32) {
        let x = self.h_offset();
        let y = self.v_offset() + self.tile_size * self.maze.len() as f32;
        let len = self.tile_size * self.maze[0].len() as f32;
        (x, y, len)
    }
    pub fn register_graphics(&mut self, ctx: &mut Context) {
        let mut mesh_builder = MeshBuilder::new();

        for row in 0..self.maze.len() {
            for col in 0..self.maze[row].len() {
                if self.maze[row][col] == 0 {
                    continue;
                }
                let y = row as f32 * self.tile_size + self.v_offset();
                let x = col as f32 * self.tile_size + self.h_offset();
                let rect = graphics::Rect::new(x, y, self.tile_size, self.tile_size);
                mesh_builder
                    .rectangle(DrawMode::fill(), rect, Color::BLACK)
                    .expect("Map drawing unsuccessful");
            }
        }
        self.graphics = Some(Mesh::from_data(ctx, mesh_builder.build()))
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let graphics = self.graphics.as_ref();
        if graphics.is_none(){return Ok(())}
        canvas.draw(graphics.unwrap(), DrawParam::default());
        Ok(())
    }
    pub fn draw_opponents(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        opponents: &Vec<Player>,
    ) -> GameResult {
        for player in opponents.iter() {
            let (x, y) = self.get_coordinates_for_pos(&player.pos);
            let rot = player.get_rotation();
            let scale = 0.6;
            let size = self.player_arrow.height();
            let x = x + size as f32 * scale;
            let y = y + size as f32 * scale;

            let point =
                Mesh::new_circle(ctx, DrawMode::fill(), [0., 0.], 5., 10., Color::RED).unwrap();
            canvas.draw(
                &point,
                DrawParam::default()
                    .dest([x - 5., y - 5.])
                    .scale([scale, scale])
                    .rotation(rot),
            );
        }
        Ok(())
    }
    pub fn draw_player_position(
        &self,
        canvas: &mut graphics::Canvas,
        player: &Player,
    ) -> GameResult {
        let (x, y) = self.get_coordinates_for_pos(&player.pos);
        let rot = player.get_rotation();
        let (x_comp, y_comp) = player.get_rotation_compensaion();
        let scale = 0.6;
        let size = self.player_arrow.height();
        let x = x + size as f32 * scale * x_comp;
        let y = y + size as f32 * scale * y_comp;

        canvas.draw(
            &self.player_arrow,
            DrawParam::default()
                .dest([x, y])
                .scale([scale, scale])
                .rotation(rot),
        );
        Ok(())
    }
}
