use termcolor::NoColor;
use termcolor_output::colored;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match colored!(w, "Text: {}, after it - some more", "Text") {
        Ok(_) => {}
        Err(_) => {}
    };
}
