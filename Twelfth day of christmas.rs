const INTRO: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth",
    "eleventh", "twelth",
];

const LYRICS: [&str; 12] = [
    "12 drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings (five golden rings)",
    "Four calling birds",
    "Three French hens",
    "Two turtle-doves",
    "a partridge in a pear tree",
];

fn main() {
    for i in 0..(INTRO.len()) {
        intro(i);

        for l in (0..(i + 1)).rev() {
            lyric(l, if l == 0 && i != 0 { "and " } else { "" });
        }
    }

    println!("\nand {}\n", LYRICS[INTRO.len() - 1])
}

fn intro(n: usize) {
    println!("\nOn the {} day of Christmas", INTRO[n]);
    println!("My true love sent to me")
}

fn lyric(n: usize, pre: &str) {
    let reverse: Vec<_> = LYRICS.iter().rev().collect();
    println!("{}{}", pre, reverse[n])
}
