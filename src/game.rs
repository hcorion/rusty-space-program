use utils;
use time::precise_time_ns;
use std::f32::consts::PI;
use rand::random;
use gravity;

use piston_window::*;

pub const FLAP: f32 = 0.16;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub is_big: bool
}

pub struct Game {
    // Something to do with time
    pub dt: f32,
    // Old time
    pub old_t: u64,
    pub particles: Vec<Particle>,
    pub object_list: Vec<utils::Obj>,
    pub max_score: u32,
    pub new_score: bool,
    pub show_help: bool,
    pub push: u32
}

pub struct Sprites {
    pub bird_1: G2dTexture,
    pub bird_2: G2dTexture,
    pub bird_3: G2dTexture,
    pub bird_x: G2dTexture,
    pub particle_1: G2dTexture,
    pub particle_2: G2dTexture
}

impl Game {
    pub fn new() -> Game {
    Game {
        dt: 0.0,
        old_t: 0,
        particles: Vec::new(),
        object_list: Vec::new(),
        max_score: 0,
        new_score: false,
        show_help: true,
        push: 0,
    }
}
    // Now should be equal to milliseconds since a time (on JS it's since the app started)
    pub fn run(&mut self, now: u64)
    {
        if self.bird().boost
        {
            let d = ((self.bird().u*self.bird().u) + (self.bird().v*self.bird().v)).sqrt();
            self.bird().u += self.bird().a.cos() * self.bird().f * FLAP;
            self.bird().v += self.bird().a.sin() * self.bird().f * FLAP;
            self.bird().t = 0;
            self.bird().boost = false;

            for i in 0..9 {
                let a: f32 = self.bird().a + ((0.5-random::<f32>())*0.25);
                let u = self.bird().u - (a.cos() * 100.0 * (random::<f32>()+1.0));
                let v = self.bird().v - (a.cos() * 100.0 * (random::<f32>()+1.0));

                let x1 = self.bird().x;
                let y1 = self.bird().y;
                self.add_particle(x1, y1,
                        u, v,
                        0.5+random::<f32>(), 
                        if random::<f32>() < 0.5 {true} else {false})
            }
        }

        if self.old_t == 0 {
            self.old_t = now;
        }
        self.dt += ((now - self.old_t) as f32 / 1000.0);
        self.old_t = now;

        const DT: f32 = 0.02;
        while self.dt > 0.0 {
            self.dt -= DT;

            let mut alive = 0;
            // Unfortunately not as clean as the original
            for index in 0..self.object_list.len()-1
            {
                gravity::grav(&mut self.object_list[index], DT);
                for index2 in 0..self.object_list.len()-1 {
                    if self.object_list[index] == self.object_list[index2] ||
                       self.object_list[index].dead == true ||
                       self.object_list[index2].dead == true {
                        return;
                    }
                    let d = utils::dist(self.object_list[index].x - self.object_list[index2].x, 
                            self.object_list[index].y - self.object_list[index2].y);
                    if d < 20.0 {
                        gravity::kill_bird(&self.object_list[index]);
                        gravity::kill_bird(&self.object_list[index2]);
                    }

                }
                if !self.object_list[index].dead && !self.object_list[index].is_bird {
                    alive += 1;
                }

                // TODO: Remove things
            }

            self.step_particles(DT);
        }
        if self.bird().t > 5 {
            self.new_bird();
        }

        // Drawing stuff!!!!

    }
    // Also known as addPart
    pub fn add_particle(&mut self, x: f32, y: f32, u: f32, v: f32, t: f32, is_big: bool){
        self.particles.push(
            Particle {
                x: x, 
                y: y, 
                u: u, 
                v: v, 
                t: t, 
                is_big: is_big}
            );
    }

    pub fn new_bird(&mut self){
        match self.object_list.len() {
            0 => (),
            n => {self.object_list[n - 1].is_bird = false; ()}
        }
        self.object_list.push(utils::Obj {
            x: 0.0,
            y: -utils::R*1.25,
            u: 0.0, 
            v: 0.0, 
            a: -PI/2.0,
            t: 0,
            boost: true,
            dead: false,
            is_bird: true,
            x_prev: 0.0,
            y_prev: -utils::R*1.25,
            a_prev: -PI/2.0,
            f: 0.0,

        });
    }
    pub fn bird(&mut self) -> &mut utils::Obj {
        let len = self.object_list.len()-1;
        return &mut self.object_list[len];
    }

    pub fn boost(&mut self) {
        self.show_help = false;
        if !self.bird().boost {
            // TODO play sound
            self.push += 1;
            if self.push >= 5 {
                self.push = 0;
            }
            self.bird().boost = true;
        }

    }

    // Otherwise known as stepParts
    pub fn step_particles(&mut self, dt: f32) {
        // TODO
        /*let mut particles_to_remove Vec<utils::Obj>;
        for p in self.parts.iter_mut() {
            p.x += p.u * dt;
            p.y += p.v * dt;
            p.t -= dt;
            if p.t < 0 {
                particles_to_remove.push(p);
            }
        }*/
        //unimplemented!();

    }

}
