extern crate piston_window;

use piston_window::*;

mod utility;

struct InputFlg{
    FORWARD: bool,
    BACK: bool,
    RIGHT: bool,
    LEFT: bool
}

impl InputFlg{
    fn new() -> Self{
        InputFlg{
            FORWARD: false,
            BACK: false,
            RIGHT: false,
            LEFT: false
        }
    }
}

struct ColoredRect{
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2],
    move_dir: [f64; 2],
    input_flg: InputFlg
}

impl ColoredRect{
    fn new() -> Self{
        ColoredRect{
            color: [1.0, 0.5, 0.25, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [1.0, 1.0],
            move_dir: [0.0, 0.0],
            input_flg: InputFlg::new()
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

        self.change_move_dir();
        self.move_obj(dt);

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

    fn change_move_dir(&mut self){
        if self.input_flg.FORWARD == true {
            self.move_dir[1] += -1.0;
        }

        if self.input_flg.BACK == true {
            self.move_dir[1] += 1.0;
        }

        self.move_dir[1] = utility::clamp(self.move_dir[1], -1.0, 1.0);

        if self.input_flg.BACK == false && self.input_flg.FORWARD == false {
            self.move_dir[1] = 0.0;
        }

        if self.input_flg.RIGHT == true {
            self.move_dir[0] += 1.0;
        }

        if self.input_flg.LEFT == true {
            self.move_dir[0] += -1.0;
        }

        self.move_dir[0] = utility::clamp(self.move_dir[0], -1.0, 1.0);

        if self.input_flg.RIGHT == false && self.input_flg.LEFT == false {
            self.move_dir[0] = 0.0;
        }
    }

    fn change_input_flg(&mut self, dir: i32, flg: bool){
        match dir{
            0 => {
                self.input_flg.FORWARD = flg;
            }
            1 => {
                self.input_flg.BACK = flg;
            }
            2 => {
                self.input_flg.RIGHT = flg;
            }
            3 => {
                self.input_flg.LEFT = flg;
            }
            _ => {}
        }
    }

    fn move_obj(&mut self, delta_time: f64){
        self.position[0] += (self.velocity[0] * self.move_dir[0]) * delta_time * 120.0;
        self.position[1] += (self.velocity[1] * self.move_dir[1]) * delta_time * 120.0;
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
                                rect.change_input_flg(0, true);
                            }
                            Key::S => {
                                rect.change_input_flg(1, true);
                            }
                            Key::A => {
                                rect.change_input_flg(3, true);
                            }
                            Key::D => {
                                rect.change_input_flg(2, true);
                            }
                            _ => {
                                //rect.change_move_dir((0.0, 0.0));
                            }
                        };
                    }
                    _ => {}
                };
            }
            Input::Release(b) => {
                match b {
                    Button::Keyboard(k) => {
                        match k {
                            Key::W => {
                                rect.change_input_flg(0, false);
                            }
                            Key::S => {
                                rect.change_input_flg(1, false);
                            }
                            Key::A => {
                                rect.change_input_flg(3, false);
                            }
                            Key::D => {
                                rect.change_input_flg(2, false);
                            }
                            _ => {
                                //rect.change_move_dir((0.0, 0.0));
                            }
                        };
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}