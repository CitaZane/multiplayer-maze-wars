use vect::vector2::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub camera_plane:Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(13., 16.),
            dir:Vector2 { x:0., y: -1.},
            camera_plane:Vector2 { x:0.65, y: 0.0 },
        }
    }
}
