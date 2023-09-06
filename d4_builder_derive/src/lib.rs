extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Setter)]
pub fn setter_derive(input: TokenStream) -> TokenStream {
    let mut top = input.into_iter();
    let ttype: TokenTree = top.next().unwrap();
    if ttype.to_string() == "pub".to_string() {
        top.next().unwrap();
    }
    let name = top.next().unwrap();
    let output: TokenStream = format!("impl {} {{
        fn print(&self) {{
            println!(\"message\");
        }}
    }}", name).parse().expect("Failed to parse");
    output
}

// #[cfg(test)]
// mod specs {
//     use super::*;
// }
