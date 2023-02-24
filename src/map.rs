use crate::player::Player;
use crate::{SCREEN_WIDTH, VIEWPORT_HEIGHT};
use ggez::graphics::{DrawMode, MeshBuilder};
use ggez::{
    glam::Vec2,
    graphics::{self, Color, DrawParam, Image, Mesh},
    Context, GameResult,
};
const TILE_SIZE: f32 = 9.0;
const MAP_WIDTH: f32 = 33.0;
const H_OFFSET: f32 = (SCREEN_WIDTH - MAP_WIDTH * TILE_SIZE) / 2.0;
const V_OFFSET: f32 = VIEWPORT_HEIGHT + 20.0 * 2.0;
pub struct Map {
    pub maze: Vec<Vec<i32>>,
    graphics: Mesh,
    player_arrow: Image,
}
// Map size 33X17
impl Map {
    pub fn new(ctx: &mut Context) -> Self {
        let maze = Map::level_one();
        let graphics = Map::register_graphics(&maze, ctx);
        let player_arrow = Image::from_path(ctx, "/arrow.png").expect("Arrow image missing");
        Map {
            maze,
            graphics,
            player_arrow,
        }
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
        let x = H_OFFSET + pos.x as f32 * TILE_SIZE;
        let y = V_OFFSET + pos.y as f32 * TILE_SIZE;
        (x, y)
    }
    pub fn get_map_corner_and_len(&self) -> (f32, f32, f32) {
        let x = H_OFFSET;
        let y = V_OFFSET + TILE_SIZE * self.maze.len() as f32;
        let len = TILE_SIZE * self.maze[0].len() as f32;
        (x, y, len)
    }
    fn register_graphics(maze: &Vec<Vec<i32>>, ctx: &mut Context) -> Mesh {
        let mut mesh_builder = MeshBuilder::new();

        for row in 0..maze.len() {
            for col in 0..maze[row].len() {
                if maze[row][col] == 0 {
                    continue;
                }
                let y = row as f32 * TILE_SIZE + V_OFFSET;
                let x = col as f32 * TILE_SIZE + H_OFFSET;
                let rect = graphics::Rect::new(x, y, TILE_SIZE, TILE_SIZE);
                mesh_builder
                    .rectangle(DrawMode::fill(), rect, Color::BLACK)
                    .expect("Map drawing unsuccessful");
            }
        }
        Mesh::from_data(ctx, mesh_builder.build())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, player: &Player) -> GameResult {
        canvas.draw(&self.graphics, DrawParam::default());
        self.draw_player_position(canvas, player)?;
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
