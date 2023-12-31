use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), std::io::Error> {
    // let (l1, l2) = make_with_life("test_data/v3_data.txt")?;
    let (l1, l2) = make_no_life("test_data/v3_data.txt")?;
    let mut s = l2.s.borrow_mut();
    s.push_str(" Mutated");
    println!("{:?}", s);
    drop(s);
    println!("{:?}", l1);
    println!("{:?}", l2);

    Ok(())
}

#[derive(Debug)]
pub struct NoLife {
    s: Rc<RefCell<String>>,
}

#[derive(Debug)]
pub struct WithLife<'a> {
    pub s: &'a String,
}

// fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
//     let s =  std::fs::read_to_string(fname)?;
//     // Ok((WithLife { s: &s }, WithLife { s: &s }))
//     Ok((WithLife { s: &s }, WithLife { s: &s }))
// }

// WIP
fn make_no_life<'a>(fname: &str) -> Result<(NoLife, NoLife), std::io::Error> {
    let s =  std::fs::read_to_string(fname)?;
    // Ok((WithLife { s: &s }, WithLife { s: &s }))
    let r = Rc::new(RefCell::new(s));
    Ok((NoLife { s: r.clone() }, NoLife { s: r }))
}
