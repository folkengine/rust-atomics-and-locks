use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let mut pack = cardpack::Pack::french_deck().cards().shuffle();
    let three = pack.draw(3).unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", three.len());
        });
        s.spawn(|| {
            for n in three.into_iter() {
                println!("{n}");
            }
        });
    });
}
