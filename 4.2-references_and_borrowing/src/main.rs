fn main() {
    // a reference is like a pointer in that it's an address we can follow to
    // access the data stored at that address.
    // that data is owned by some other variable.
    // unlike a pointer, a reference is guaranteed to point to a valid value of
    // a particular type for the life of that reference.

    fn example1() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("the length of '{}' is {}", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    example1();
}
