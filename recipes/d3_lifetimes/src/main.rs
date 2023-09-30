#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

fn main() {
    println!("Hello, world!");
    let mut s = make_str(7);
    // let p = part(&s); // won work if comment out line 11, s is borrowed
    s.push_str("p");
    // let p = part(&s); // won work if comment out line 11, s is borrowed and won't be showing exact value
    s = to_people(s);
    // let p = part(&s); // won work if comment out line 11, s is borrowed and won't be showing exact value
    to_frogs(&mut s);

    let p = part(&s);
    println!("{}", p);
    let tog = two_strings(p, &s);
    // s.push_str("not that easy"); // won't work since will break reference
    println!("{:?}", tog);
    s.push_str("can be done");
    println!("{}", s);
}

fn make_str(n: i32) -> String {
    format!("hello, {}", n)
}

fn to_people(mut s: String) -> String {
    s.push_str(" people");
    s
}

fn to_frogs(s: &mut String) {
    s.push_str(" frogs");
}

fn part<'a>(s: &'a str) -> &'a str {
    if s.len() > 4 {
        // WHY do we return referenced slice, can't we return slice, s is already a reference
        &s[..4]
    } else {
        s
    }
}

pub fn two_strings<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder { s, t }
}
