use utils;
use time::precise_time_ns;
use std::f32::consts::PI;
use rand::random;

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
    pub particle_2: G2dTexture,
    pub arrow: G2dTexture,
    pub logo: G2dTexture,
    pub medal_bronze: G2dTexture,
    pub medal_gold: G2dTexture,
    pub medal_platinum: G2dTexture,
    pub medal_silver: G2dTexture,
    pub new: G2dTexture,
    pub number_0: G2dTexture,
    pub number_1: G2dTexture,
    pub number_2: G2dTexture,
    pub number_3: G2dTexture,
    pub number_4: G2dTexture,
    pub number_5: G2dTexture,
    pub number_6: G2dTexture,
    pub number_7: G2dTexture,
    pub number_8: G2dTexture,
    pub number_9: G2dTexture,
    pub scoreboard: G2dTexture,
    pub tap: G2dTexture,
    pub tap_top: G2dTexture
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
            self.bird().t = 0.0;
            self.bird().boost = false;

            for i in 0..9 {
                let a: f32 = self.bird().a + ((0.5-random::<f32>())*0.25);
                let u = self.bird().u - (a.cos() * 100.0 * (random::<f32>()+1.0));
                let v = self.bird().v - (a.sin() * 100.0 * (random::<f32>()+1.0));

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
            for index in (0..self.object_list.len()).rev()
            {
                if self.grav(index, DT) {
                    self.kill_bird(index);
                }
                for index2 in 0..self.object_list.len() {
                    if self.object_list[index] == self.object_list[index2] ||
                       self.object_list[index].dead == true ||
                       self.object_list[index2].dead == true {
                        continue;
                    }
                    let d = utils::dist(self.object_list[index].x - self.object_list[index2].x, 
                            self.object_list[index].y - self.object_list[index2].y);
                    if d < 20.0 {
                        self.kill_bird(index);
                        self.kill_bird(index2);
                    }

                }
                if !self.object_list[index].dead && !self.object_list[index].is_bird {
                    alive += 1;
                }

                // Remove dead objects that have fallen to earth
                if self.object_list[index].remove_me {
                    self.object_list.remove(index);
                }
            }

            self.step_particles(DT);
        }
        if self.bird().t > 5.0 {
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
            t: 0.0,
            boost: false,
            dead: false,
            is_bird: true,
            x_prev: 0.0,
            y_prev: -utils::R*1.25,
            a_prev: -PI/2.0,
            f: 0.0,
            remove_me: false,

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
        for p in self.particles.iter_mut() {
            p.t -= dt;
            p.x += p.u * dt;
            p.y += p.v * dt;
        }
        self.particles.retain(|ref x| x.t >= 0.0);
    }

    pub fn grav(&mut self, index: usize, dt: f32) -> bool
    {
        let obj = &mut self.object_list[index];
        obj.x_prev = obj.x;
        obj.y_prev = obj.y;
        obj.a_prev = obj.a;

        let d = utils::dist(obj.x, obj.y);
        let f = utils::G*utils::M/(d*d);
        obj.f = f;
        let n_x = obj.x/d;
        let n_y = obj.y/d;

        obj.u -= n_x * f * dt as f32;
        obj.v -= n_y * f * dt as f32;

        // Compute angle
        if obj.dead == true {
            obj.a += (10.0*dt) as f32;
        }
        else {
            let aa = obj.y.atan2(obj.x);
            if d < 200.0 {
            obj.a = aa + (PI/2.0) * ((d-100.0)/100.0);
            }
            else {
            obj.a = aa + (PI/2.0);
            }
        }
        let xx = obj.x + obj.u * dt as f32;
        let yy = obj.y + obj.v * dt as f32;
        let dd = utils::dist(xx, yy);
        if dd > utils::R {
            obj.x = xx;
            obj.y = yy;

            obj.t += dt;
            
            if dd > 400.0 && !obj.dead { // kill if out of range
                return true;
                //self.kill_bird(index);
            }
        }
        else {
            // Colliding
            obj.x = utils::R*xx/dd;
            obj.y = utils::R*yy/dd;
            obj.u = 0.0;
            obj.v = 0.0;

            //remove if not controlled bird
            if !obj.is_bird {
            obj.remove_me = true;
            }
        }
        return false;
    }

    pub fn kill_bird(&mut self, index: usize)
    {
        // TODO play sound
        //let obj = &mut self.object_list[index];
        self.object_list[index].dead = true;
        self.object_list[index].u /= 10.0;
        self.object_list[index].v /= 10.0;
        
        if self.object_list[index].is_bird{
            self.new_bird();
        }
        for i in 0..9
        {
            let a = random::<f32>()*PI*2.0;
            let U = a.cos() * 100.0 * (random::<f32>()+1.0);
            let V = a.sin() * 100.0 * (random::<f32>()+1.0);
            let x = self.object_list[index].x;
            let y = self.object_list[index].y;
            self.add_particle(x, y,
                        U, V,
                        0.5+random::<f32>(), 
                        if random::<f32>() < 0.5 {true} else {false});
            
        }
    }
}
