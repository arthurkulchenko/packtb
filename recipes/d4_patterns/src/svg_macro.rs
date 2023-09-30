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
