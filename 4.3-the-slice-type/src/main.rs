fn main() {
    println!("4.3 The Slice Type");

    // Slices le you reference a contiguous sequence of elements in a
    // collection rather than the whole colleciton. Is a king of reference,
    // so it does not have ownership.

    manual_slice();
    string_slices();
    string_literal_as_slices();
}

fn manual_slice() {
    let mut s = String::from("hello world");
    let word = first_word_index(&s); // word will get the value 5

    s.clear(); // this empties the string, making it equal to ""
    // word still has the value 5 here, but there's no more string that we
    // could meaningfully use the value 5 with. word is now totally invaldy!

    // This program compiles without any errors and would also do so if we used
    // word after calling s.clear().
    // Having to worry about the index in word getting out of sync with de data in
    second_word_index(&s);
    // s is tedious and error prpone! Managing these indices is even more brittle
    // if we wirte a second_word function.
    // We have inralates variables floating around that need to be kept in sync.

    let mut s = String::from("hello world");
    // return slices instead of indices
    let word = first_word(&s);
    second_word(&s);
    // s.clear(); // error here
    println!("the first word is: {word}");
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word_index(s: &str) -> (usize, usize) {
    todo!()
}

fn second_word(s: &String) -> &str {
    todo!()
}

fn string_slices() -> _ {
    // A string slice is a reference to part of a String

    let s = String::from("Hello world!");
    let hello = &s[0..5]; // same as [..5]
    let world = &s[6..11];
    let till_end = &s[6..]; // same as [6..s.len()]
    let entire = &s[..]; // same as [0..s.len()]
    // [string_index..ending_index]

    // Internally, the slice data structure stores the starting position and
    // the length of the slice, which corresponds to
    // ending_index minus starting_index
}

fn string_literal_as_slices() -> _ {
    todo!()

    // https://doc.rust-lang.org/stable/book/ch04-03-slices.html#string-literals-as-slices
}
