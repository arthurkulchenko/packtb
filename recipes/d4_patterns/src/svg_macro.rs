macro_rules! svg {
    // tt - token tree
    // * - repetition
    // ident - identifier
    ($svg_name: ident $($property: ident = $value: expr), * => $($child: tt)*) => {
        SvgTag::new(
          stringify!($svg_name)
        )
        $(.$property($value))*
        $(.child(svg! $child))*
    };
    ($svg_name: ident $($property: ident = $value: expr), *) => {
        SvgTag::new(
          stringify!($svg_name)
        )
        $(.$property($value))* // .x(7).y(8).width(9).height(9)
    };
}

#[cfg(test)]
pub mod specs {
    // use super::*;
    use crate::SvgTag;

    #[test]
    pub fn macro_creates_svg_as_builder() {
        let csvg = svg! { svg => { rect x = 7, y = 8, width = 9, height = 9 } };
        let dsvg = SvgTag::new("svg").child(SvgTag::new("rect").x(7).y(8).width(9).height(9));
        assert_eq!(csvg, dsvg);
    }
}

// DECLARATIVE MACROS:
#[macro_export]
macro_rules! macro_example {
    ($x: expr) => {
        println!("{} is a macro", $x);
    };
}
// EXAMPLE:
// let some_var: String = macro_example!(1 + 2);
#[macro_export]
macro_rules! macro_i32 {
    ($x: ty) => {
        match stringify!($x) {
            "i32" => println!("{} is i32 type", stringify!($x)),
            _ => println!("{} is somethint not i32 type", stringify!($x)),
        }
    };
}
// EXAMPLE:
// let some_var: String = macro_i32!(i32);
#[macro_export]
macro_rules! macro_my_vec {
    //  - zero or more
    // + - one or more
    // * - zero or more
    ( $($x: expr),+ ) => {
        {
            let mut temp = Vec::new();

            $(
                temp.push($x);
            )+
            temp
        }
    }
}
// EXAMPLE:
// let some_var: Vec<i32> = macro_my_vec![1, 2, 3];


