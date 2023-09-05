use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Property {
    Simple(&'static str, String),
    Style(&'static str, String),
    Transform(String)
}

#[derive(Debug, PartialEq)]
struct SvgTag {
    pub kind: &'static str,
    pub properties: Vec<Property>,
    pub children: Vec<SvgTag>,
}

impl SvgTag {
    pub fn new(kind: &'static str) -> Self {
        SvgTag {
            kind,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: SvgTag) -> Self {
        self.children.push(child);
        self
    }

    pub fn property<V: Display>(mut self, k: &'static str, v: V) -> Self {
        self.properties.push(Property::Simple(k, v.to_string()));
        self
    }

    pub fn style<V: Display>(mut self, k: &'static str, v: V) -> Self {
        self.properties.push(Property::Style(k, v.to_string()));
        self
    }

    pub fn x<V:Display>(self, v: V) -> Self {
        self.property("x", v)
    }

    pub fn y<V:Display>(self, v: V) -> Self {
        self.property("y", v)
    }

    pub fn width<V:Display>(self, v: V) -> Self {
        self.style("width", v)
    }

    pub fn height<V:Display>(self, v: V) -> Self {
        self.style("height", v)
    }
}

#[cfg(test)]
mod test {
    use crate::Property::Simple;
    use crate::Property::Style;
    use crate::SvgTag;

#[test]
    fn spec() {
        let a = SvgTag::new("svg")
                        .width("60px")
                        .height("80px")
                        .child(
                            SvgTag::new("rect")
                                    .width("30px")
                                    .height("30px")
                                    .x(5)
                                    .y(5)
                        );
        let b = SvgTag {
            kind: "svg",
            properties: vec![
                Style("width", "60px".to_string()),
                Style("height", "80px".to_string())
            ],
            children: vec![
                SvgTag {
                    kind: "rect",
                    properties: vec![
                        Style("width", "30px".to_string()),
                        Style("height", "30px".to_string()),
                        Simple("x", "5".to_string()),
                        Simple("y", "5".to_string()),
                    ],
                    children: vec![]
                }
            ],
        };
        assert_eq!(a, b);
    }
}
