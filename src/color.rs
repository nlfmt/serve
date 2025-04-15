#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        concat!("\x1b[38;2;", $r, ";", $g, ";", $b, "m")
    };
}

pub const RST: &str = "\x1b[0m";

pub const BLUE: &str = color!(142, 190, 249);
pub const LBLUE: &str = color!(179, 209, 242);
pub const GREEN: &str = color!(147, 249, 142);
pub const ORANGE: &str = color!(244, 183, 117);
pub const GRAY: &str = color!(150, 150, 150);