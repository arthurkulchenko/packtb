// cargo run --bin v2_closure

#[derive(Debug)]
pub struct Hider {
    pub public: String,
    hidden: String,
    hidden_access: i32,
}

impl Hider {
    // pub fn new(public: String, hidden: String, hidden_access: i32) -> Hider {
    pub fn new(public: String, hidden: String) -> Self {
        Hider { public, hidden, hidden_access: 0 }
    }

    pub fn edit<F>(&mut self, f: F) where F: FnOnce(&mut String) {
        // WHY: do we need to declare "&mut" for "self.hidden"?
        f(&mut self.hidden);
        self.hidden_access += 1;
    }
}

#[derive(Debug)]
pub struct HideVec {
    v: Vec<String>,
}

// NOTICE:
// variation 1
// let variable_name = [value_1, value_2, ...];
// variation 2
// let variable_name:[data_type; array_size] = [value_1, value_2, ...];
// variation 3
// let variable_name:[data_type; array_size] = [default_value_for_all_elements; array_size];

impl HideVec {
    pub fn new(n: usize) -> Self {
        // let v = Vec::with_capacity(n);
        HideVec { v: vec![String::new(); n] }
    }

    pub fn edit_all<F>(&mut self, mut f: F) where F: FnMut(&mut String) {
    // pub fn edit_all<F>(&mut self, f: F) where F: Fn(&mut String) {
        // WHY: function has to be mutable? Because we will do counting inside this function and it has match mutability - mut f: F and F: FnMut
        // loop through mutable items
        for item in &mut self.v {
            f(item)
        }
    }
}

fn main() {
    let mut h = Hider::new("showme".to_string(), "hideme".to_string());
    h.edit(|s| s.push_str(" please"));

    println!("{:?}", h);

    let mut hv = HideVec::new(6);
    let mut count = 0;
    hv.edit_all(|s| {
        // WHY: do we pass reference of format
        // s.push_str(&format!("Item = {}", count));
        s.push_str(&format!("Item = {}", count));
        count += 1;
    });

    println!("HV: {:?}", hv);
    println!("count: {}", count);
}
