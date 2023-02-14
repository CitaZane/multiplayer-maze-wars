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
    pub fn get_rotation(&self)->f32{
        match self.dir{
            Vector2{x:1., y:0.}=> 0.0,//right
            Vector2 { x:-1., y:0. }=>3.1415926536,//left
            Vector2 { x:0., y:-1. }=>4.7123889804,//up
            _ => 1.5707963268 //down
        }
    }
    pub fn get_rotation_compensaion(&self)->(f32,f32){
        match self.dir{
            Vector2{x:1., y:0.}=> (0., 0.),//right
            Vector2 { x:-1., y:0.}=>(1., 1.),//left
            Vector2 { x:0., y:-1.}=>(0., 1.),//up
            _ => (1., 0.) //down
        }
    }
}
