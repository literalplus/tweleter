use ansi_term::Colour::{Red, Yellow, Cyan, White};
use ansi_term::ANSIStrings;
use std::string::String;

macro_rules! r {($val: expr) => (Red.paint($val))}
macro_rules! y {($val: expr) => (Yellow.paint($val))}
macro_rules! c {($val: expr) => (Cyan.paint($val))}
macro_rules! wr {($val: expr) => (White.reverse().paint($val))}

macro_rules! println_combined {
($($element:expr),*) => {{
    let mut v = Vec::new();
    $(v.push(format!("{}", $element));)*
    println!(" {}", v.join(""));
}};
}

pub fn print_banner() {
    println!(" {}", r!("  *   )              (           )"));
    println!(" {}       {}", r!("` )  /( (  (      (  )\\   (   ( /(   (   ("),
             wr!("     Tweleter vX.X.X      "));
    println_combined!(r!(" ( )("), y!("_)"), r!("))\\)"), y!(")"), r!("(    )"), y!(")"),
             r!("\\((_) )"), y!(")"), r!("\\  )\\"), y!("()"), r!(") )"), y!(")"),
             r!("\\  )(      "), wr!("      by Literallie       "));
    println_combined!(r!("("), c!("_"), y!("("), c!("_"), y!("()"), r!(")("), y!("(_)()"),
             r!("\\  /"), y!("((_"), r!(")"), c!("_"), r!(")) "), r!("/"), y!("((_"), r!("("),
             c!("_"), y!(")"), r!(")/ /"), y!("((_"), r!(")("), y!("()"), r!("\\     "),
             wr!(" github.com/xxyy/tweleter "));
    println_combined!(fill_letters("|_   _|_"), r!("("), y!("()"), r!("("), y!("("), c!("_"),
             r!(")("), c!("_"), y!(")"), r!(")"), fill_letters(" | |"), r!(" ("), c!("_"), y!(")"),
             r!(") "), fill_letters("| |_ "), r!(" ("), c!("_"), y!(")"), r!(")  ("), y!("("),
             c!("_"), r!(")    "), wr!("  more flames = more lit™ "));
    println!(" {}", fill_letters("  | |  \\ V  V // -_)| |/ -_) |  _|/ -_) | '_|"));
    println!(" {}", fill_letters("  |_|   \\_/\\_/ \\___||_|\\___|  \\__|\\___| |_|"));
}

fn fill_letters(format: &str) -> String {
    let mut inside = false;
    let reverse_cyan = Cyan.reverse();
    let mut styles_vec = Vec::new();

    for char in format.chars() {
        let is_border = char == '|' || char == '\\' || char == '/' || char == ')';
        let style = if inside || is_border {
            reverse_cyan.paint(char.to_string())
        } else {
            Cyan.paint(char.to_string())
        };
        styles_vec.push(style);
        if is_border {
            inside = !inside;
        }
    }

    return format!("{}", ANSIStrings(&styles_vec));
}






