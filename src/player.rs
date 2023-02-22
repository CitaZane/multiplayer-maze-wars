use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use throttle::Throttle;

pub struct Player {
    pub name: String,
    pub pos: Vec2,
    pub dir: Direction,
    pub throttle: Throttle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn vec(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2 { x: 0., y: -1. },
            Direction::Down => Vec2 { x: 0., y: 1. },
            Direction::Left => Vec2 { x: -1., y: 0. },
            Direction::Right => Vec2 { x: 1., y: 0. },
        }
    }
    pub fn from_vec(dir : &(f32, f32))->Direction{
        match dir{
            (0.,-1.) => Direction::Up,
            (0., 1.) => Direction::Down,
            (-1., 0.)=> Direction::Left,
            (1., 0.) => Direction::Right,
            _ => Direction::Up
        }
    }
}

impl Player {
    pub fn new(name:String) -> Self {
        Self {
            pos: Vec2::new(1., 1.),
            dir: Direction::Right,
            throttle: Throttle::new(Duration::from_millis(100), 1),
            name,
        }
    }
    pub fn go_forward(&mut self, maze: &Vec<Vec<i32>>) -> bool{
        return self.go(maze, self.dir.clone());
    }
    pub fn go_backward(&mut self, maze: &Vec<Vec<i32>>) -> bool{
        let direction = match self.dir {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        };
        return self.go(maze, direction);
    }
    fn go(&mut self, maze: &Vec<Vec<i32>>, direction: Direction) -> bool {
        if self.throttle.accept().is_ok() {
            match direction {
                Direction::Right => {
                    if self.pos.x >= (maze[0].len() - 1) as f32 {
                        return false;
                    }
                    if maze[self.pos.y as usize][self.pos.x as usize + 1] == 0 {
                        self.pos.x += 1.;
                        return true
                    }
                }
                Direction::Down => {
                    if self.pos.x >= (maze.len() - 1) as f32 {
                        return false;
                    }
                    if maze[self.pos.y as usize + 1][self.pos.x as usize] == 0 {
                        self.pos.y += 1.;
                        return true
                    }
                }
                Direction::Left => {
                    if self.pos.x < 1.0 {
                        return false;
                    }
                    if maze[self.pos.y as usize][self.pos.x as usize - 1] == 0 {
                        self.pos.x -= 1.;
                        return true
                    }
                }
                Direction::Up => {
                    if self.pos.y < 1.0 {
                        return false;
                    }
                    if maze[self.pos.y as usize - 1][self.pos.x as usize] == 0 {
                        self.pos.y -= 1.;
                        return true
                    }
                }
            };
        }
        return false
    }
    pub fn turn_right(&mut self) -> bool{
        if self.throttle.accept().is_ok(){
            self.dir = match self.dir {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            };
            return true
        };
        return false
    }
    pub fn turn_left(&mut self) -> bool{
        if self.throttle.accept().is_ok() {
            self.dir = match self.dir {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
            };
            return true
        }
        return false
    }
    pub fn camera_plane(&self) -> Vec2{
        const FOV:f32 = 0.65;
        match self.dir {
            Direction::Right => Vec2 { x: 0.0, y: FOV },
            Direction::Down => Vec2 { x: -FOV, y: 0. },
            Direction::Left => Vec2 { x: 0.0, y: -FOV },
            Direction::Up => Vec2 { x: FOV, y: 0.0 },
        }
    }
    pub fn get_rotation(&self) -> f32 {
        match self.dir {
            Direction::Right => 0.0,         //right
            Direction::Left => 3.1415926536, //left
            Direction::Up => 4.7123889804,   //up
            Direction::Down => 1.5707963268, //down
        }
    }
    pub fn get_rotation_compensaion(&self) -> (f32, f32) {
        match self.dir {
            Direction::Right => (0., 0.),
            Direction::Left => (1., 1.),
            Direction::Up => (0., 1.),
            Direction::Down => (1., 0.),
        }
    }
    pub fn get_opponent_direction(&self, opponent_dir: &Direction) -> Direction {
        if self.dir == *opponent_dir {
            return Direction::Up;
        } else if self.dir.vec().x == opponent_dir.vec().x
            || self.dir.vec().y == opponent_dir.vec().y
        {
            return Direction::Down;
        } else {
            match self.dir {
                Direction::Up => {
                    if *opponent_dir == Direction::Left {
                        return Direction::Left;
                    } else {
                        return Direction::Right;
                    }
                }
                Direction::Down => {
                    if *opponent_dir == Direction::Right {
                        return Direction::Left;
                    } else {
                        return Direction::Right;
                    }
                }
                Direction::Right => {
                    if *opponent_dir == Direction::Up {
                        return Direction::Left;
                    } else {
                        return Direction::Right;
                    }
                }
                Direction::Left => {
                    if *opponent_dir == Direction::Down {
                        return Direction::Left;
                    } else {
                        return Direction::Right;
                    }
                }
            }
        }
    }
}
