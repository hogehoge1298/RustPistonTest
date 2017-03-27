extern crate piston_window;

use piston_window::*;

struct ColoredRect{
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2],
    delta_time: f64
}

impl ColoredRect{
    fn new() -> Self{
        ColoredRect{
            color: [1.0, 0.5, 0.25, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [1.0, 1.0],
            delta_time: 0.0
        }
    }

    fn update_color(dt: f32, color: f32) -> f32{
        if color <= 0.0{
            1.0
        }else{
            color -0.001 * dt * 120.0
        }
    }

    fn update(&mut self, dt: f64, size: (f64, f64)){
        self.color[0] = Self::update_color(dt as f32, self.color[0]);
        self.color[1] = Self::update_color(dt as f32, self.color[1]);
        self.color[2] = Self::update_color(dt as f32, self.color[2]);

        self.delta_time = dt;

        //X Updates
        if self.position[0] + self.position[2] >= size.0{
            self.position[0] = size.0;
        }
        if self.position[0] < 0.0{
            self.position[0] = 0.0
        }

        //Y Updates
        if self.position[1] + self.position[3] >= size.1{
                self.position[1] = size.1;
        }

        if self.position[1] < 0.0 {
            self.position[1] = 0.0;
        }
    }

    fn change_velocity(&mut self, factor: f64){
        self.velocity[0] *= factor;
        self.velocity[1] *= factor;
    }

    fn move_obj(&mut self, value: (f64, f64)){
        self.position[0] += (self.velocity[0] * value.0) * self.delta_time * 120.0;
        self.position[1] += (self.velocity[1] * value.1) * self.delta_time * 120.0;
    }
}

fn main(){
    let mut rect = ColoredRect::new();
    
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut window_size: (f64, f64) = (0.0, 0.0);

    while let Some(e) = window.next(){
        match e {
            Input::Render(r) => {
                window_size = (r.width as f64, r.height as f64);
                window.draw_2d(&e, |c, g|{
                    clear([1.0; 4], g);
                    rectangle(rect.color,
                                rect.position,
                                c.transform, g);
                });
            }
            Input::Update(u) => {
                rect.update(u.dt, window_size);
            }
            Input::Press(b) => {
                match b {
                    Button::Keyboard(k) => {
                        match k {
                            Key::W => {
                                rect.move_obj((0.0, -1.0));
                            }
                            Key::S => {
                                rect.move_obj((0.0, 1.0));
                            }
                            Key::A => {
                                rect.move_obj((-1.0, 0.0));
                            }
                            Key::D => {
                                rect.move_obj((1.0, 0.0));
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}