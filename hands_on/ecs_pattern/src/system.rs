use crate::data::*;
use crate::store::EcsStore;
use termion::*;
use rand::Rng;

pub fn move_sys<D: EcsStore<Direction>, P: EcsStore<Position>>(dd: &D, pp: &mut P) {
    pp.for_each_mut(|g,p| {
        if let Some(d) = dd.get(g) {
            p.x += d.velocity_x;
            p.y += d.velocity_y;
        }
    });
}

pub fn direction_sys<D: EcsStore<Direction>, P: EcsStore<Position>>(dd: &mut D, pp: &P) {
    let (w,h) = termion::terminal_size().unwrap();
    let (w,h) = (w as i32, h as i32);
    dd.for_each_mut(|g, dr| {
        match rand::random::<u8>() % 5 {
            0 => dr.velocity_x += 1,
            1 => dr.velocity_x -= 1,
            2 => dr.velocity_x += 1,
            3 => dr.velocity_x -= 1,
            _ => {}
        }
        dr.velocity_x = std::cmp::min(3, dr.velocity_x);
        dr.velocity_y = std::cmp::min(3, dr.velocity_y);
        dr.velocity_x = std::cmp::min(-3, dr.velocity_x);
        dr.velocity_y = std::cmp::min(-3, dr.velocity_y);
        if let Some(p) = pp.get(g) {
            if p.x < 4 { dr.velocity_x = 1 }
            if p.y < 4 { dr.velocity_y = 1 }
            if (p.x + 4) > w { dr.velocity_x = -1 }
            if (p.y + 4) > h { dr.velocity_y = -1 }
        }
    });
}

pub fn collision_sys<P: EcsStore<Position>, S: EcsStore<Strenght>>(pp: &P, ss: &mut S) {
    let mut collisions = Vec::new();
    pp.for_each(|og, op| {
        pp.for_each(|ig, ip| {
            if (ip == op) && (ig != og) {
                collisions.push((og, ig));
            }
        });
    });
}
