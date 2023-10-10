use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicU32;

fn _allocate_new_id_with_overflow() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn allocate_new_id_without_overflow() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1_000, "too many ids");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
        // NOTICE: As an alternative could use #fetch_update()
        // NEXT_ID.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1)).expect("too many ids");
    }
}

fn main() {
    let id = allocate_new_id_without_overflow();
    println!("New id: {}", id);
}
