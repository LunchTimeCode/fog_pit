use colored::Colorize;

pub enum Foggy {
    Green(String),
    Red(String),
    Yellow(String),
    Blue(String),
    White(String),
}

impl Foggy {
    pub fn swallow(&self) {
        match self {
            Foggy::Green(g) => println!("{}", g.green()),
            Foggy::Red(r) => {
                println!("{}", r.red());
                std::process::exit(1)
            }
            Foggy::Yellow(y) => println!("{}", y.yellow()),
            Foggy::Blue(b) => println!("{}", b.blue()),
            Foggy::White(w) => println!("{w}"),
        }
    }
}

pub fn blue_fog(s: impl Into<String>) -> Foggy {
    Foggy::Blue(s.into())
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

pub fn white_fog(s: impl Into<String>) -> Foggy {
    Foggy::White(s.into())
}

pub fn explain_fog() {
    let blue = blue_fog("Blue: is information");
    let green = green_fog("Green: An action was successfull");
    let yellow = yellow_fog("Yellow: Is a warning, you may want to remidiate this");
    let white = white_fog("White: Is meant for piping into a different command");
    let visit = blue_fog("Visit https://github.com/LunchTimeCode/fog_pit for more");
    let red = red_fog("Red: An action created an error and the process may exit with exit code 0");

    blue.swallow();
    green.swallow();
    yellow.swallow();
    white.swallow();
    visit.swallow();
    red.swallow();
}
