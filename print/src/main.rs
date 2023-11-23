use std::fmt;
use std::mem;
use proc_macro_lib;

#[derive(Debug, HelloWorld)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "~name: {1}, age: {0}~", self.age, self.name)
    }
}

struct Pair(i32, f32);

const OUTPUT: &str = "";

#[function_to_string]
fn funciton_to_be_filled_by_ai() {
    // function_to_string Macro will take this function for llm to fill it with a contents
    println!("{}", OUTPUT);
}

enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    print!("==================================================================\n");
    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!();
    println!("The quick brown {subject} {verb} over the lazy {object}", object="dog", subject="fox", verb="jumps");
    println!();

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    // println!("Base 10:               {}",   69420); // 69420
    // println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    // println!("Base 8 (octal):        {:o}", 69420); // 207454
    // println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    // println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("={number:>5}=", number=1);

    // You can pad numbers with extra zeroes,
    println!("={number:0>5}=", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("={number:0<5}=", number=1); // 10000

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "Jamie");
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5); // 00001

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("={number:>width$}=");
    print!("={number:>width$}=");
    println!("{:#?}", (100, 200));     // => "(
                                       //       100,
                                       //       200,
                                       //     )"
    let people = "Rustaceans";
    println!("Hello {people}!");
    println!("{:07}", 42); // => 000 0042
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");
    println!("Hello {width}!");
    let x = &42;

    println!("{x:p}"); // this produces something like '0x7f06092ac6d0'
    // :?
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="Person's");

    let person = Person { name: "Peter", age: 23 };
    person.hello_world();

    println!("{:?}", person);
    println!("{:#?}", person);
    println!("{}", person); // implemented fmt::Display for Person
    println!("1 - 2 = {}", 1_i32 - 2_i32);
    println!("1e4 is {}, -2.5e-3 is {}", 1_e4, -2.5_e-3);
    println!("0011 AND 0101 is {:04b}", 0b_0011_u32 & 0b_0101);
    println!("0011 AND 0101 is {:04b}", 0b_0011 & 0b_0101);
    println!("0011 XOR 0101 is {:04b}", 0b_0011_u32 ^ 0b_0101);
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // let xs: [i32; 1] = [1];
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    let click   = WebEvent::Click { x: 20, y: 80 };
    inspect(click);
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
    
}
