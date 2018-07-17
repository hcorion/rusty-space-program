use utils;
use time::precise_time_ns;
use rand::random;
use gravity;

pub const flap: f32 = 0.16;

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
    pub bird: utils::Obj,
    pub particles: Vec<Particle>,
    pub objectList: Vec<utils::Obj>,
}
impl Game {
    // Now should be equal to milliseconds since a time (on JS it's since the app started)
    pub fn run(&mut self, now: u64)
    {
        if self.bird.boost
        {
            let d = ((self.bird.u*self.bird.u) + (self.bird.v*self.bird.v)).sqrt();
            self.bird.u += self.bird.a.cos() * self.bird.f * flap;
            self.bird.v += self.bird.a.sin() * self.bird.f * flap;
            self.bird.t = 0;
            self.bird.boost = false;

            for i in 0..9 {
                let a: f32 = self.bird.a + ((0.5-random::<f32>())*0.25);
                let U = self.bird.u - (a.cos() * 100.0 * (random::<f32>()+1.0));
                let V = self.bird.v - (a.cos() * 100.0 * (random::<f32>()+1.0));

                let x1 = self.bird.x;
                let y1 = self.bird.y;
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
            /*for obj in self.objectList.iter_mut() {
                gravity::grav(obj, DT);
                for obj2 in self.objectList.iter() {
                    if *obj == *obj2 || obj.dead == true || obj2.dead == true{
                        return;
                    }
                    let d = utils::dist(obj.x - obj2.x, obj.y - obj2.y);
                    if d < 20.0 {
                        gravity::kill_bird(obj);
                        gravity::kill_bird(obj2);
                    }
                }
            }*/

        }

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

}