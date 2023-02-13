use vect::vector2::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub camera_plane:Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(17.0, 9.0),
            dir:Vector2::up(),
            camera_plane:Vector2::right(),
        }
    }
}
