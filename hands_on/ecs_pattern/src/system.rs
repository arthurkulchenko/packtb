use crate::data::*;
use crate::store::EcsStore;
use crate::gen::{GenManager};
use termion::color;
use termion::raw::RawTerminal;

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

pub fn collision_sys<P: EcsStore<Position>, S: EcsStore<Strength>>(pp: &P, ss: &mut S) {
    let mut collisions = Vec::new();
    pp.for_each(|og, op| {
        pp.for_each(|ig, ip| {
            if (ip == op) && (ig != og) {
                collisions.push((og, ig));
            }
        });
    });

    for (og, ig) in collisions {
        let damage = match ss.get(og) {
            Some(b) => b.s,
            None => continue
        };
        let damage = damage as i16;
        let h_up = if let Some(bumpee) = ss.get_mut(ig) {
            let n = bumpee.s + 1;
            if bumpee.h <= i16::MIN + damage {
                0
            } else {
                bumpee.h -= damage;
                if bumpee.h <= 0 {
                    n
                } else {
                    0
                }
            }
        } else {
            0
        };

        if h_up > 0 {
            if let Some(bumper) = ss.get_mut(og) {
                if i16::MAX - h_up >= bumper.h {
                    bumper.s += 1;
                } else {
                    bumper.h += h_up;
                    bumper.s += 1;
                }
            }
        }
    }
}

pub fn render_sys<T: std::io::Write, P: EcsStore<Position>, S: EcsStore<Strength>>(
    t: &mut RawTerminal<T>,
    pp: &P,
    ss: &S
) {
    write!(t, "{}", termion::clear::All).ok();
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    pp.for_each(|g, position| {
        if let Some(st) = ss.get(g) {
            let color = match st.h {
                0 => color::Fg(color::Black).to_string(),
                1 => color::Fg(color::Red).to_string(),
                2 => color::Fg(color::Yellow).to_string(),
                3 => color::Fg(color::Green).to_string(),
                _ => color::Fg(color::Blue).to_string(),
            };
            let x = (position.x % w) + 1;
            let y = (position.y % h) + 1;
            write!(t, "{}{}{}", termion::cursor::Goto(x as u16, y as u16), color, st.s).ok();

        }
    });
}

pub fn death_sys<S: EcsStore<Strength>, P: EcsStore<Position>, D: EcsStore<Direction>>(
    g: &mut GenManager,
    ss: &mut S,
    pp: &mut P,
    dd: &mut D
) {
    let mut death_list = Vec::new();
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    pp.for_each(|ig, position| {
      if position.x > w || position.x < 0 || position.y > h || position.y < 0 {
          death_list.push(ig);
      }
    });

    for pray in death_list {
        g.drop(pray);
        ss.remove(pray);
        pp.remove(pray);
        dd.remove(pray);
    }

}
