use std::io::Cursor;
use bicycle_book_wordcount::*;

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");

    let freqs = count(input, CountOption::Char);
    assert_map!(freqs,
                {
                    "a"=>6,
                    "b"=>2,
                    "c"=>1,
                    "d"=>2,
                    "r"=>2
                }
                );
}

