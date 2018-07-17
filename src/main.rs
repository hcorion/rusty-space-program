extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate rand;
extern crate time;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use piston_window::*;

mod utils;
mod gravity;
mod game;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    window: PistonWindow,
    background: G2dTexture
}

impl App {
    fn render(&mut self, args: &RenderArgs, event: Event) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
        self.draw_background(event);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn draw_background (&mut self, event: Event) {
        let background = &self.background;
        self.window.set_lazy(true);
        self.window.draw_2d(&event, |c, g| {
            clear([1.0; 4], g);
            image(background, c.transform, g);
        });
    }

    fn draw_bird(&mut self, event: Event, bird: Obj, time: i32, alpha: f32) {
        let whatbird = if bird.dead self.sprites.bird_x
        else
        {let phase = time % 0.8;
         if phase < 0.2 self.sprites.bird_1
         else if phase < 0.4 self.sprites.bird_2
         else if phase < 0.6 self.sprites.bird_3 else self.sprites.bird_2};

        self.window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(
                alpha*bird.x + (1-alpha*bird.x_prev),
                alpha*bird.y + (1-alpha*bird.y_prev))
                .rot_rad(interpolate_angle(bird.a_prev, bird.a, alpha))
                .trans(-20.0, -20.0);

            image(whatbird, transform, g);
        });
    }

}

fn initialize_texture (window: &mut PistonWindow, fname: &str) -> G2dTexture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let background = assets.join(fname);
    Texture::from_path(
        &mut window.factory,
        &background, Flip::None,
        &TextureSettings::new()
    ).unwrap()
}


fn init_app () -> App {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Rusty Flaps",
        [256, 256]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let sprites = Sprites {
        bird_1: initialize_texture(&mut window, "bird-1.png"),
        bird_2: initialize_texture(&mut window, "bird-2.png"),
        bird_3: initialize_texture(&mut window, "bird-3.png"),
        bird_x: initialize_texture(&mut window, "bird-x.png"),
        particle_1: initialize_texture(&mut window, "particle-1.png"),
        particle_2: initialize_texture(&mut window, "particle-2.png")
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        background: initialize_texture(&mut window, "bg.png"),
            window: window
    };
    app
}

fn main() {

    // Create a new game and run it.
    let mut app = init_app();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window) {
        if let Some(r) = e.render_args() {
            app.render(&r, e);
        } else {

        if let Some(u) = e.update_args() {
            app.update(&u);
        }}
    }
}
