use std::thread;

fn main() {
    let mut pack = cardpack::Pack::french_deck().cards().shuffle();
    let sb = pack.draw(3).unwrap();

    thread::spawn(move || {
        for n in sb.into_iter() {
            println!("{}", n.symbol_colorized(&cardpack::US_ENGLISH));
        }
    }).join().unwrap();
}
