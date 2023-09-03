fn main() -> Result<(), std::io::Error> {
    let (l1, l2) = make_with_life("test_data/v3_data.txt")?;
    Ok(())
}

#[derive(Debug)]
pub struct WithLife<'a> {
    pub s: &'a String,
}

// WIP
fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
    let s =  std::fs::read_to_string(fname)?;
    Ok((WithLife { s: &s }, WithLife { s: &s }))
}
