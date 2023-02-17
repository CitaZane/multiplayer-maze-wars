use crate::{SCREEN_WIDTH, VIEWPORT_HEIGHT};
use crate::player::Player;
use ggez::{
    graphics::{self, Color, DrawParam, Mesh, Image},
    Context, GameResult, glam::Vec2,
};
const TILE_SIZE: f32 = 9.0;
const MAP_WIDTH: f32 = 33.0;
const H_OFFSET: f32 = (SCREEN_WIDTH - MAP_WIDTH * TILE_SIZE) / 2.0;
const V_OFFSET: f32 = VIEWPORT_HEIGHT + 20.0 * 2.0;
pub struct Map {
    pub maze: Vec<Vec<i32>>,
    graphics: Vec<Mesh>,
    pub player_arrow: Image,
}
// Map size 33X17
impl Map {
    pub fn new(ctx: &mut Context) -> Self {
        let maze = Map::level_one();
        let graphics = Map::register_graphics(&maze, ctx);
        let player_arrow = Image::from_path(ctx, "/back-arrow.png").expect("Arrow image missing");
        Map { maze, graphics , player_arrow}
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

    fn register_graphics(maze: &Vec<Vec<i32>>, ctx: &mut Context) -> Vec<Mesh> {
        let mut graphics = vec![];

        for row in 0..maze.len() {
            for col in 0..maze[row].len() {
                if maze[row][col] == 0 {
                    continue;
                }
                let y = row as f32 * TILE_SIZE + V_OFFSET;
                let x = col as f32 * TILE_SIZE + H_OFFSET;
                let rect = graphics::Rect::new(x, y, TILE_SIZE, TILE_SIZE);
                let mesh = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    Color::from_rgb(0, 0, 0),
                )
                .unwrap();
                graphics.push(mesh)
            }
        }
        graphics
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, _ctx: &mut Context) -> GameResult {
        for mesh in self.graphics.iter() {
            canvas.draw(mesh, DrawParam::default());
        }
        Ok(())
    }
    pub fn draw_player_position(&mut self, canvas: &mut graphics::Canvas,player:&Player)-> GameResult {
        let (x, y) = self.get_coordinates_for_pos(&player.pos);
        let rot = player.get_rotation();
        let (x_comp, y_comp)=player.get_rotation_compensaion();
        let scale = 0.6;
        let size = self.player_arrow.height();
        let x = x + size as f32*scale  * x_comp;
        let y = y + size as f32*scale  * y_comp;

        canvas.draw(&self.player_arrow, DrawParam::default()
        .dest([x , y])
        .scale([scale,scale])
        .rotation(rot));
        Ok(())
    }
}
