use std::f32::consts::PI;

#[derive(PartialEq)]
pub struct Obj {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    // Probably velocity
    pub v: f32,
    // Probably acceleration
    pub a: f32,
    // Amount of time in seconds since bird's birth, probably
    pub t: f32,
    pub f: f32,
    pub boost: bool,
    pub dead: bool,
    pub is_bird: bool,
    pub x_prev: f32,
    pub y_prev: f32,
    pub a_prev: f32,
    pub remove_me: bool,
}

pub const M: f32 = 1000000.0;
pub const G: f32 = 10.0;
pub const R: f32 = 100.0;

// Distance away from 0,0
pub fn dist(x1: f32, y1: f32) -> f32 { (((x1*x1)+(y1*y1))).sqrt() }

pub fn interpolate_angle(ang2: f32, ang1: f32, alpha: f32) -> f64 {
    let mut da: f32 = (ang2 - ang1) % (2.0 * PI);
    if da > PI {
        da = da - 2.0 * PI;
    } else {
        if da < -PI {
            da = da + 2.0 * PI;
        }
    }
    (ang1 + alpha * da) as f64
}
