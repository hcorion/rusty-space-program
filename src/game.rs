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
    pub isBig: bool
}

pub struct Game {
    // Something to do with time
    pub dt: f32,
    // Old time
    pub oldT: u64,
    pub particles: Vec<Particle>,
    pub objectList: Vec<utils::Obj>,
    pub maxScore: u32,
    pub newScore: bool
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
        oldT: 0,
        particles: Vec::new(),
        objectList: Vec::new(),
        maxScore: 0,
        newScore: false
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
                let U = self.bird().u - (a.cos() * 100.0 * (random::<f32>()+1.0));
                let V = self.bird().v - (a.cos() * 100.0 * (random::<f32>()+1.0));

                let x1 = self.bird().x;
                let y1 = self.bird().y;
                self.add_particle(x1, y1,
                        U, V,
                        0.5+random::<f32>(), 
                        if random::<f32>() < 0.5 {true} else {false})
            }
        }
        if self.oldT == 0 {
            self.oldT = now;
        }
        self.dt += ((now - self.oldT) / 1000) as f32;
        self.oldT = now;

        let mut DT = 0.02;
        while DT > 0.0 {
            self.dt -= DT;

            let mut alive = 0;
            // Unfortunately not as clean as the original
            for index in 0..self.objectList.len()-1
            {
                gravity::grav(&mut self.objectList[index], DT);
                for index2 in 0..self.objectList.len()-1 {
                    if self.objectList[index] == self.objectList[index2] ||
                       self.objectList[index].dead == true ||
                       self.objectList[index2].dead == true {
                        return;
                    }
                    let d = utils::dist(self.objectList[index].x - self.objectList[index2].x, 
                            self.objectList[index].y - self.objectList[index2].y);
                    if d < 20.0 {
                        gravity::kill_bird(&self.objectList[index]);
                        gravity::kill_bird(&self.objectList[index2]);
                    }

                }
                if !self.objectList[index].dead && !self.objectList[index].is_bird {
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
    pub fn add_particle(&mut self, x: f32, y: f32, u: f32, v: f32, t: f32, isBig: bool){
        self.particles.push(
            Particle {
                x: x, 
                y: y, 
                u: u, 
                v: v, 
                t: t, 
                isBig: isBig}
            );
    }

    pub fn new_bird(&mut self){
        match self.objectList.len() {
            0 => (),
            n => {self.objectList[n - 1].is_bird = false; ()}
        }
        self.objectList.push(utils::Obj {
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
        let len = self.objectList.len()-1;
        return &mut self.objectList[len];
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
