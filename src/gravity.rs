use utils;
use std::f32::consts::PI;

pub fn grav(mut obj: &mut utils::Obj, dt: f32)
{
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
        kill_bird(obj);
      }
  }
  else {
    // Colliding
    obj.x = utils::R*xx/dd;
    obj.y = utils::R*yy/dd;
    obj.u = 0.0;
    obj.v = 0.0;

    //remove if not controlled bird
    //remove_obj(obj);
  }


}

pub fn kill_bird(mut obj: &mut utils::Obj){
  // TODO play sound
  obj.dead = true;
  obj.u /= 10.0;
  obj.v /= 10.0;
  if obj.is_bird{
    obj.add_new_bird = true;
    obj.is_bird = false;
  }

  // TODO Draw death particles
}

fn remove_obj(mut obj: &utils::Obj){
  println!{"plz remove"};
  // RELEVANT CODE:
  /*
        // remove if not controlled bird
      if (obj !== bird && toRemove.indexOf(obj) == -1) { 
        toRemove.push(obj);
      }
  */
    //unimplemented!();
}
