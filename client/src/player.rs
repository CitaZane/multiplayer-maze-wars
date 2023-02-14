use vect::vector2::Vector2;

#[derive(Debug)]
pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub camera_plane:Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(1., 1.),
            dir:Vector2 { x:1., y: 0.},
            camera_plane:Vector2 { x:0.0, y: 0.65 },
        }
    }
}
