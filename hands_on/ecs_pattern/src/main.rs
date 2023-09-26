mod store;
mod gen;
mod data;
mod system;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use store::{EcsStore, VecStore};
use std::io::Write;
use data::*;

fn main() {
    let (sink, stream) = std::sync::mpsc::channel();
    std::thread::spawn(move||{
        let stdin = std::io::stdin();
        for key in stdin.keys() {
            if let Ok(k) = key {
                sink.send(k).ok();
            }
        }
    });
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);

    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = gen::GenManager::new();
    let mut strengths: VecStore<Strength> = VecStore::new(); // NEED: To specify concrete type
    let mut directions: VecStore<Direction> = VecStore::new();
    let mut positions: VecStore<Position> = VecStore::new();
    let mut pass = 0;
    loop {
        let g = gen.next();
        strengths.insert(g, Strength { s: 1, h: 5 });
        directions.insert(g, Direction { velocity_x: 0, velocity_y: 0 });
        positions.insert(g, Position { x: rand::random::<i32>() % w, y: rand::random::<i32>() % h });
        system::direction_sys(&mut directions, &positions);
        system::move_sys(&directions, &mut positions);
        system::collision_sys(&positions, &mut strengths);
        system::death_sys(&mut gen, &mut strengths, &mut positions, &mut directions);
        system::render_sys(&mut screen, &positions, &strengths);
        write!(&mut screen, "{}Pass={}", termion::cursor::Goto(1,1), pass).ok();
        pass += 1;
        screen.flush().ok();
        while let Ok(k) = stream.try_recv() {
            match k {
                Key::Char('q') => return,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
