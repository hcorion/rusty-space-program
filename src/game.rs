use utils;
use std::f32::consts::PI;
use std::cmp::max;
use rand::random;
use music;

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
    pub cur_score: u32,
    pub new_score: bool,
    pub show_help: bool,
    pub push: u32,
    pub sound_sending: SoundSending,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Boom,
    Gain, 
    Medal, 
    Push,
}

pub struct SoundSending {
    pub boom: bool, 
    pub boom_timer: f32,
    pub gain: bool, 
    pub medal: bool, 
    pub push: bool,
    pub push_timer: f32,
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
        let mut result = Game {
            dt: 0.0,
            old_t: 0,
            particles: Vec::new(),
            object_list: Vec::new(),
            max_score: 0,
            cur_score: 0,
            new_score: false,
            show_help: true,
            push: 0,
            sound_sending: SoundSending{boom: false, boom_timer: 4.0, 
                                        gain: false,
                                        medal: false,
                                        push: false, push_timer: 4.0}
        };
        result.new_bird();
        return result;
    }

    // Now should be equal to milliseconds since a time (on JS it's since the app started)
    pub fn run(&mut self, now: u64)
    {
        if self.bird().boost
        {
            //FIXME: dead line?
            //let d = ((self.bird().u*self.bird().u) + (self.bird().v*self.bird().v)).sqrt();
            self.bird().u += self.bird().a.cos() * self.bird().f * FLAP;
            self.bird().v += self.bird().a.sin() * self.bird().f * FLAP;
            self.bird().t = 0.0;
            self.bird().boost = false;

            for _i in 0..9 {
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
        self.dt += (now - self.old_t) as f32 / 1000.0;
        self.old_t = now;
        let musicdt = self.dt;
        self.music_handler(musicdt);

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
            self.cur_score = alive;
            if self.new_score == true && alive < self.max_score
            {
                self.new_score = false;
            }
            if alive > self.max_score
            {
                self.new_score = true;
                if alive % 5 == 0 {
                    self.sound_sending.medal = true;
                }
                else {
                    self.sound_sending.gain = true;
                }
            }
            self.max_score = max(alive, self.max_score);
                  /*if (newScore == true && score < maxScore) {
        newScore = false;
      }
      if (score > maxScore) {
        newScore = true;
        if ([5, 10, 15, 20].indexOf(score) !== -1) {
          play(sndMedal);
        } else { 
          play(sndGain);
        }
      }
      maxScore = Math.max(score, maxScore);*/
        }
        if self.bird().t > 5.0 {
            self.new_bird();
        }

        

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

            self.sound_sending.push = true;

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
            
            // Kill the bird if out of range
            if dd > 400.0 && !obj.dead { 
                return true;
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
        self.sound_sending.boom = true;

        self.object_list[index].dead = true;
        self.object_list[index].u /= 10.0;
        self.object_list[index].v /= 10.0;
        
        if self.object_list[index].is_bird{
            self.new_bird();
        }
        for _i in 0..9
        {
            let a = random::<f32>()*PI*2.0;
            let uu = a.cos() * 100.0 * (random::<f32>()+1.0);
            let vv = a.sin() * 100.0 * (random::<f32>()+1.0);
            let x = self.object_list[index].x;
            let y = self.object_list[index].y;
            self.add_particle(x, y,
                        uu, vv,
                        0.5+random::<f32>(), 
                        if random::<f32>() < 0.5 {true} else {false});
            
        }
    }

    pub fn music_handler(&mut self, dt: f32)
    {
        self.sound_sending.boom_timer += dt + 0.02;
        self.sound_sending.push_timer += dt + 0.02;
        if self.sound_sending.boom
        {
            self.sound_sending.boom = false;
            if self.sound_sending.boom_timer > 1.10
            {
                self.sound_sending.boom_timer = 0.0;
                music::play_sound(&Sound::Boom, music::Repeat::Times(0), music::MAX_VOLUME);
            }
        }
        if self.sound_sending.push
        {
            self.sound_sending.push = false;
            if self.sound_sending.push_timer > 0.40
            {
                self.sound_sending.push_timer = 0.0;
                music::play_sound(&Sound::Push, music::Repeat::Times(0), music::MAX_VOLUME);
            }
        }
        if self.sound_sending.medal
        {
            music::play_sound(&Sound::Medal, music::Repeat::Times(0), music::MAX_VOLUME);
            self.sound_sending.medal = false;
        }
        if self.sound_sending.gain
        {
            music::play_sound(&Sound::Gain, music::Repeat::Times(0), music::MAX_VOLUME);
            self.sound_sending.gain = false;
        }
    }
}
