#[derive(PartialEq)]
pub struct Obj {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    // Probably velocity
    pub v: f32,
    // Probably acceleration
    pub a: f32,
    pub t: u64,
    pub f: f32,
    pub boost: bool,
    pub dead: bool,
    pub is_bird: bool,
    pub x_prev: f32,
    pub y_prev: f32,
    pub a_prev: f32,
}

pub const M: f32 = 1000000.0;
pub const G: f32 = 10.0;
pub const R: f32 = 100.0;

pub fn dist(x1: f32, y1: f32) -> f32 { (((x1*x1)+(y1*y1))).sqrt() }
/*
 function newBird() {
    var bird = {
      x : 0,
      y : -R*1.25,
      u: 0,
      v: 0,
      a : -Math.PI/2,
      t: 0,
      boost: 0,
      dead: false
    };
    bird.xPrev = bird.x;
    bird.yPrev = bird.y;
    bird.aPrev = bird.a;
*/