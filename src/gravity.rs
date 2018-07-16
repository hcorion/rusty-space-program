use utils;
use std::f32::consts::PI;

fn grav(mut obj: utils::Obj, dt: f32)
{
  obj.x_prev = obj.x;
  obj.y_prev = obj.y;
  obj.a_prev = obj.a;

  let d = utils::dist(obj.x, obj.y);
  let f = utils::G*utils::M/d*d;
  obj.f = f;
  let n_x = obj.x/d;
  let n_y = obj.y/d;

  obj.u -= n_x * f * dt;
  obj.v -= n_y * f * dt;

  // Compute angle
  if obj.dead == true {
    obj.a += 10.0*dt;
  }
  else {
    let A = obj.y.atan2(obj.x);
    if d < 200.0 {
      obj.a = A + (PI/2.0) * (d-100.0/100.0);
    }
    else {
      obj.a = A + (PI/2.0);
    }
  }
  let X = obj.x + obj.u*dt;
  let Y = obj.y + obj.v*dt;
  let D = utils::dist(X, Y);
  if D > utils::R {
    obj.x = X;
    obj.y = Y;

    obj.t += dt;
    
    if D > 400.0 && !obj.dead { // kill if out of range
        kill_bird(obj);
      }
  }
  else {
    // Colliding
    obj.x = R*X/D;
    obj.y = R*Y/D;
    obj.u = 0;
    obj.v = 0;

    //remove if not controlled bird
    remove_obj(obj);
  }


}

fn kill_bird(mut obj: utils::Obj){
    unimplemented!();
}

fn remove_obj(mut obj: utils::Obj){
  // RELEVANT CODE:
  /*
        // remove if not controlled bird
      if (obj !== bird && toRemove.indexOf(obj) == -1) { 
        toRemove.push(obj);
      }
  */
    unimplemented!();
}