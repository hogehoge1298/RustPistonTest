extern crate PistonTutorial;

use PistonTutorial::utility;

struct RectActor{
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2],
    move_dir: [f64; 2]
}

impl RectActor{
    fn new() => Self{
        RectActor{
            color: [1.0, 0.0, 0.0, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [120.0, 120.0],
            move_dir: [0.0, 0.0]
        }
    }

    fn update(&mut self, dt: f64, size: (f64, f64)){
        self.move(dt);
    }

    fn move(&mut self, dt: f64, size: (f64, f64)){
        self.position[0] += (self.velocity[0] * self.move_dir[0]) * delta_time;

        self.position[0] = utility::clamp(self.move_dir)
        self.position[1] += (self.velocity[1] * self.move_dir[1]) * delta_time;
    }
}