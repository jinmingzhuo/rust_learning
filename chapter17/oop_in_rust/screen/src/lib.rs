use std::fmt;
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "the button label is {}, h:{},w:{}",
            self.label, self.width, self.height
        );
    }
}
// impl fmt::Debug for Button {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.debug_struct("Screen")
//             .field("components", &self.components.len())
//             .finish()

//     }

// }

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for ele in self.components.iter() {
            ele.draw();
        }
    }
}
impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Screen")
            .field("components", &self.components.len())
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = Button {
            width: 32,
            height: 43,
            label: String::new(),
        };
        let s = SelectBox {
            width: 2,
            height: 4,
            options: vec![],
        };
        let a = Screen {
            components: vec![Box::new(b), Box::new(s)],
        };
        println!("{:?}", a);
    }
}
