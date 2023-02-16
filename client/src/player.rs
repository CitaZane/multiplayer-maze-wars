use ggez::glam::Vec2;

#[derive(Debug)]
pub struct Player {
    pub pos: Vec2,
    pub dir: Direction,
    pub camera_plane:Vec2,
}

#[derive(Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    pub fn vec(&self)-> Vec2{
        match self{
            Direction::Up => Vec2 { x: 0., y: -1. },
            Direction::Down => Vec2 { x: 0., y: 1. },
            Direction::Left => Vec2 { x: -1., y: 0. },
            Direction::Right => Vec2 { x: 1., y: 0. }
        }
    }
}


impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(1., 1.),
            dir:Direction::Right,
            camera_plane:Vec2 { x:0.0, y: 0.65 },
        }
    }
    pub fn turn_right(&mut self){
        self.dir = match self.dir {
            Direction::Right => Direction::Down,
            Direction::Down =>Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        };
        self.configure_camera_plane();
    }
    pub fn turn_left(&mut self){
        self.dir = match self.dir {
            Direction::Right => Direction::Up,
            Direction::Down =>Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        };
        self.configure_camera_plane();
    }
    fn configure_camera_plane(&mut self){
        self.camera_plane = match self.dir {
            Direction::Right => Vec2 { x: 0.0, y: 0.65 },
            Direction::Down => Vec2 { x: -0.65, y: 0. },
            Direction::Left =>  Vec2 { x: 0.0, y: -0.65 },
            Direction::Up => Vec2 { x: 0.65, y: 0.0 },
        }
    }
    pub fn get_rotation(&self)->f32{
        match self.dir{
            Direction::Right=> 0.0,//right
            Direction::Left=>3.1415926536,//left
            Direction::Up=>4.7123889804,//up
            Direction::Down=> 1.5707963268 //down
        }
    }
    pub fn get_rotation_compensaion(&self)->(f32,f32){
        match self.dir{
            Direction::Right=> (0., 0.),
            Direction::Left=>(1., 1.),
            Direction::Up=>(0., 1.),
            Direction::Down => (1., 0.) 
        }
    }
}
