use rand::seq::IteratorRandom;

#[no_mangle]
pub fn get_wise_saying() -> String {
    let str = include_str!("fortunes.txt");
    let mut lines = str.lines();

    let line = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
    return line.to_string();
}
