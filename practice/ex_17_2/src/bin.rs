pub strcut Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub trait Draw {
    fn draw(&self);
}

impl Draw for Button {
    fn draw(&self) {
        // 
    }
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }