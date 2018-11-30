extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate rand;
extern crate time;
// Music
extern crate music;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use piston_window::Window as Window2;
use time::precise_time_ns;

use piston_window::*;

mod utils;
mod game;

use std::f32::consts::PI;
use std::cmp::max;

pub struct App {
   // gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    window: PistonWindow,
    sprites: game::Sprites,
    background: G2dTexture,
    game: game::Game
}

impl App {
    fn render(&mut self, _args: &RenderArgs, event: Event) {
        let time = (precise_time_ns()/1000000) as f32 / 1000.0;
        self.draw_background(event.clone());
        self.draw_limit(event.clone());
        if self.game.show_help {self.draw_help(event.clone(), time);}
        let dt = self.game.dt;
        self.draw_particles(event.clone(), dt);
        self.draw_score(event.clone());
        self.draw_birds(event, time, dt/0.02+1.0);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn x_center (&self) -> f32 {
        return (self.window.size().width / 2) as f32;
    }

    fn y_center (&self) -> f32 {
        return (self.window.size().height / 2) as f32;
    }

    fn draw_limit (&mut self, event: Event) {
        let quantity = 50;
        let q2 = 50.0;
        let xcenter = self.x_center() as f64;
        let ycenter = self.y_center() as f64;
        for n in 0..quantity {
            let whatpart = if n % 2 == 0 {&self.sprites.particle_1}
            else {&self.sprites.particle_2};
            self.window.draw_2d(&event, |c, g| {
                let transform = c.transform.rot_rad(n as f64 * (PI * 2.0 / q2) as f64)
                    .trans(400.0, 0.0)
                    .rot_rad(-n as f64 * (PI * 2.0 / q2) as f64)
                    .trans(xcenter, ycenter)
                    .scale(2.0, 2.0);

                image(whatpart, transform, g);
            });
        }
    }
    fn get_letter_sprite(&self, number: u32) -> G2dTexture
    {
        return match number {
            1 => self.sprites.number_1.clone(),
            2 => self.sprites.number_2.clone(),
            3 => self.sprites.number_3.clone(),
            4 => self.sprites.number_4.clone(),
            5 => self.sprites.number_5.clone(),
            6 => self.sprites.number_6.clone(),
            7 => self.sprites.number_7.clone(),
            8 => self.sprites.number_8.clone(),
            9 => self.sprites.number_9.clone(),
            _ => self.sprites.number_0.clone(),
            };
    }
    fn draw_score(&mut self, event: Event)
    {
        let SCORE_POS_X: f64 = (self.window.size().width - 60) as f64;
        let SCORE_POS_Y: f64 = 31.0;
        let SCORE_SCALE: f64 = 2.5;
        // Single digit numbers

        let max_score = self.game.max_score;
        let cur_score = self.game.cur_score;
        let max_score_tens = (max_score as f32 * 0.1).floor() as u32;
        let max_score_ones = (((max_score as f32 * 0.1) - max_score_tens as f32) * 10.0) as u32;
        let cur_score_tens = (cur_score as f32 * 0.1).floor() as u32;
        let cur_score_ones = (((cur_score as f32 * 0.1) - cur_score_tens as f32) * 10.0) as u32;
        
        let max_number_tens = &self.get_letter_sprite(max_score_tens);
        let max_number_ones = &self.get_letter_sprite(max_score_ones);
        
        let cur_number_tens = &self.get_letter_sprite(cur_score_tens);
        let cur_number_ones = &self.get_letter_sprite(cur_score_ones);

        self.window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(SCORE_POS_X, SCORE_POS_Y).scale(SCORE_SCALE, SCORE_SCALE);
            image(cur_number_tens, transform, g);
        });
        self.window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(SCORE_POS_X+20.0, SCORE_POS_Y).scale(SCORE_SCALE, SCORE_SCALE);
            image(cur_number_ones, transform, g);
        });
        self.window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(SCORE_POS_X, SCORE_POS_Y+45.0).scale(SCORE_SCALE, SCORE_SCALE);
            image(max_number_tens, transform, g);
        });
        self.window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(SCORE_POS_X+20.0, SCORE_POS_Y+45.0).scale(SCORE_SCALE, SCORE_SCALE);
            image(max_number_ones, transform, g);
        });
        
        if self.game.new_score
        {
            let new = &self.sprites.new;
            self.window.draw_2d(&event, |c, g| {
                let transform = c.transform.trans(SCORE_POS_X-45.0, SCORE_POS_Y + 4.0).scale(SCORE_SCALE, SCORE_SCALE);
                image(new, transform, g);
            });
        }
    }

    fn draw_help (&mut self, event: Event, time: f32) {
        let timefrag = time % 1.0;
        let tap = &self.sprites.tap;
        let top = &self.sprites.tap_top;
        let xcenter = self.x_center() as f64;
        let ycenter = self.y_center() as f64;
        if (0.25 < timefrag) && (timefrag < 0.5)
        {self.window.draw_2d(&event, |c, g| {
            image(tap,
                  c.transform.trans(xcenter - 11.0, ycenter - 6.0).scale(2.0, 2.0), g);
            image(top,
                  c.transform.trans(xcenter - 7.0, ycenter - 18.0).scale(2.0, 2.0), g);
        });}
        else
        {self.window.draw_2d(&event, |c, g| {
            image(tap,
                  c.transform.trans(xcenter - 11.0, ycenter - 2.0).scale(2.0, 2.0), g);
        });}
    }

    fn draw_background (&mut self, event: Event) {
        let background = &self.background;
        let logo = &self.sprites.logo;
        let scoreboard = &self.sprites.scoreboard;
        let xcenter = self.x_center() as f64;
        let ycenter = self.y_center() as f64;
        let width = self.window.size().width as f64;
        self.window.set_lazy(true);
        self.window.draw_2d(&event, |c, g| {
            clear([0.296875, 0.5234375, 0.546875, 1.0], g);
            image(background,
                  c.transform.trans(xcenter - 256.0, ycenter - 256.0).scale(2.0, 2.0), g);
            image(logo, c.transform.scale(2.0, 2.0), g);
            image(scoreboard,
            c.transform.trans(width - 226.0, 0.0).scale(2.0, 2.0), g);
        });
    }

    fn draw_particles (&mut self, event: Event, time: f32) {
        let xcenter = self.x_center();
        let ycenter = self.y_center();
        for pcl in self.game.particles.iter() {
            let whatpart = if pcl.is_big {&self.sprites.particle_1}
            else {&self.sprites.particle_2};
            self.window.draw_2d(&event, |c, g| {
                let transform = c.transform.trans(
                    (xcenter + pcl.x + pcl.u * time).into(),
                    (ycenter + pcl.y + pcl.v * time).into())
                    .scale(2.0, 2.0);

                image(whatpart, transform, g);
            });
        }
    }

    fn draw_birds
        (&mut self, event: Event, time: f32, alpha: f32) {
            let xcenter = self.x_center();
            let ycenter = self.y_center();
            for bird in self.game.object_list.iter() {
                let whatbird = if bird.dead {&self.sprites.bird_x}
                else
                {let phase = time as f32 % 0.8;
                 if phase < 0.2 {&self.sprites.bird_1}
                 else if phase < 0.4 {&self.sprites.bird_2}
                 else if phase < 0.6 {&self.sprites.bird_3}
                 else {&self.sprites.bird_2}};
                self.window.draw_2d(&event, |c, g| {
                    let transform = c.transform.trans(
                        (xcenter + alpha*bird.x + (1.0-alpha)*bird.x_prev).into(),
                        (ycenter + alpha*bird.y + (1.0-alpha)*bird.y_prev).into())
                        .rot_rad(utils::interpolate_angle(bird.a_prev, bird.a, alpha))
                        .trans(-20.0, -20.0)
                        .scale(2.0, 2.0);

                    image(whatbird, transform, g);
                });
            }}
}

fn initialize_texture (window: &mut PistonWindow, fname: &str) -> G2dTexture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let background = assets.join(fname);
    Texture::from_path(
        &mut window.factory,
        &background, Flip::None,
        &TextureSettings::new().filter(texture::Filter::Nearest)
    ).unwrap()
}


fn init_app () -> App {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Rusty Flaps",
        [900, 900]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let sprites = game::Sprites {
        bird_1: initialize_texture(&mut window, "bird-1.png"),
        bird_2: initialize_texture(&mut window, "bird-2.png"),
        bird_3: initialize_texture(&mut window, "bird-3.png"),
        bird_x: initialize_texture(&mut window, "bird-x.png"),
        particle_1: initialize_texture(&mut window, "particle-1.png"),
        particle_2: initialize_texture(&mut window, "particle-2.png"),
        arrow: initialize_texture(&mut window, "arrow.png"),
        logo: initialize_texture(&mut window, "logo.png"),
        medal_bronze: initialize_texture(&mut window, "medal-bronze.png"),
        medal_gold: initialize_texture(&mut window, "medal-gold.png"),
        medal_platinum: initialize_texture(&mut window, "medal-platinum.png"),
        medal_silver: initialize_texture(&mut window, "medal-silver.png"),
        new: initialize_texture(&mut window, "new.png"),
        number_0: initialize_texture(&mut window, "number-0.png"),
        number_1: initialize_texture(&mut window, "number-1.png"),
        number_2: initialize_texture(&mut window, "number-2.png"),
        number_3: initialize_texture(&mut window, "number-3.png"),
        number_4: initialize_texture(&mut window, "number-4.png"),
        number_5: initialize_texture(&mut window, "number-5.png"),
        number_6: initialize_texture(&mut window, "number-6.png"),
        number_7: initialize_texture(&mut window, "number-7.png"),
        number_8: initialize_texture(&mut window, "number-8.png"),
        number_9: initialize_texture(&mut window, "number-9.png"),
        scoreboard: initialize_texture(&mut window, "scoreboard.png"),
        tap: initialize_texture(&mut window, "tap.png"),
        tap_top: initialize_texture(&mut window, "tap-top.png")
    };

    let mut app = App {
        //gl: GlGraphics::new(opengl),
        rotation: 0.0,
        background: initialize_texture(&mut window, "bg.png"),
        window: window,
        sprites: sprites,
        game: game::Game::new()
    };
    app
}

fn main() {

    // Create a new game and run it.
    let mut app = init_app();

    let mut events = Events::new(EventSettings::new());
    music::start::<game::Music, game::Sound, _>(16, || {
        music::bind_sound_file(game::Sound::Boom, "./assets/boom.ogg");
        music::bind_sound_file(game::Sound::Gain, "./assets/gain.ogg");
        music::bind_sound_file(game::Sound::Medal, "./assets/medal.ogg");
        music::bind_sound_file(game::Sound::Push, "./assets/push.ogg");

        music::set_volume(music::MAX_VOLUME);
        
        while let Some(e) = events.next(&mut app.window) {
            app.game.run(precise_time_ns()/1000000);
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::Space {
                    app.game.boost();
                }
            }
            if let Some(r) = e.render_args() {
                app.render(&r, e);
            } else {
            if let Some(u) = e.update_args() {
                app.update(&u);
            }}
        }
    });
}
