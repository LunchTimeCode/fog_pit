use colored::Colorize;

pub enum Foggy {
    Green(String),
    Red(String),
    Yellow(String),
}

impl Foggy {
    pub fn swallow(&self) {
        match self {
            Foggy::Green(g) => println!("{}", g.green()),
            Foggy::Red(r) => println!("{}", r.green()),
            Foggy::Yellow(y) => println!("{}", y.green()),
        }
    }
}

pub fn green_fog(s: impl Into<String>) -> Foggy {
    Foggy::Green(s.into())
}

pub fn red_fog(s: impl Into<String>) -> Foggy {
    Foggy::Red(s.into())
}

pub fn yellow_fog(s: impl Into<String>) -> Foggy {
    Foggy::Yellow(s.into())
}
