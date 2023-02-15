use ggez::{
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameResult
};
use vect::vector2::Vector2;
use crate::{VIEWPORT_HEIGHT, SCREEN_WIDTH};
const TILE_SIZE: f32 = 9.0;
const MAP_WIDTH: f32 = 33.0;
const H_OFFSET: f32 = (SCREEN_WIDTH - MAP_WIDTH * TILE_SIZE) /2.0;
const V_OFFSET: f32 = VIEWPORT_HEIGHT + 20.0 * 2.0;
pub struct Map(pub Option<Vec<Vec<i32>>>);
// Map size 33X17
impl Map {
    pub fn new() -> Self {
        let m = Map::level_one();
        Map(Some(m))
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
    pub fn get_coordinates_for_pos(&self, pos: &Vector2)-> (f32,f32){
        let x = H_OFFSET + pos.x as f32 * TILE_SIZE;
        let y = V_OFFSET + pos.y as f32 *TILE_SIZE;
        (x,y)
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let map = self.0.as_ref().unwrap();
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == 0 {
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
                )?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }
        Ok(())
    }
}