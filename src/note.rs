use crate::circle::Circle;

#[derive(Clone)]
pub struct Fdl {
    pub fun: bool,
    pub done: bool,
    pub learn: bool,
}

#[derive(Clone)]
pub struct Note {
    pub description: String,
    pub fdl: Fdl,
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
    pub original_x: isize,
    pub original_y: isize,
    pub client_x: isize,
    pub client_y: isize,
    pub dragging: bool,
}

impl Note {
    #[must_use]
    pub const fn new(description: String, x: isize, y: isize) -> Self {
        Self {
            description,
            fdl: Fdl {
                fun: false,
                done: false,
                learn: false,
            },
            x,
            y,
            width: 150,
            height: 100,
            original_x: 0,
            original_y: 0,
            client_x: 0,
            client_y: 0,
            dragging: false,
        }
    }

    pub fn set_fdl(&mut self, fdl: (Circle, Circle, Circle)) {
        let fun = fdl.0;
        let done = fdl.1;
        let learn = fdl.2;
        self.fdl = Fdl {
            fun: fun.include(self.x + self.width / 2, self.y + self.height / 2),
            done: done.include(self.x + self.width / 2, self.y + self.height / 2),
            learn: learn.include(self.x + self.width / 2, self.y + self.height / 2),
        }
    }
}
