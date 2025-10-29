fn main() {
    // println!("Hello, world!");

    // test_String();
    // test_ownership_and_copy();

    // immutable_and_mutable_references();

    test_slicing();
}

fn test_String() {
    let s = "hello"; //-? string literal -> "&str" -> immutable , read-only -> stored in stack

    let mut s2 = String::from("hello"); // String "hello" -> stored on heap-> but ptr to "hello" stored on stack

    // let new_s = s + "world";    ->> error -> need String on left but both are &str

    // let new_s = s.push_str("word");  ->> error

    let new_s2 = s2.push_str(" ,world");

    let mut s3 = "kartik"; // -> "hello\0"

    s3 = "pandey"; //->> "hello\0world\0"  -> ptr points to world not hello -> but hello stilll stays in memory

    println!("{s3}"); //--> print pandey

    let s4 = String::from("hello");

    let s5 = s4; // s4 moved into s5 -> s4 no longer valid

    // println!("{s4}");  --> thorws error
}

fn test_ownership_and_copy() {
    let num = 7i8; // --> copy type --> those which are stored on stack

    let s = String::from("hello"); // --> no copy type  -> those whose values are stored on heap

    takes_ownership(s); // ownership of s moved to --> takes_ownership(s)

    // now s is out of scope of test_ownership_and_copy() -->cannot be used further

    // println!("{s}"); // error

    makes_copy(num); // copy of 7 wil be created and passwd to makes_copy();

    println!("{num}"); // now error

    let mut s2 = String::from("kartik pandey");

    let length = calculate_length(&mut s2); // just passing reference not the ownership

    println!("{s2} --> not dumped by parent;;;;; length is {length}");
}

fn takes_ownership(input_string: String) {
    //ownership of s is given to input_string

    println!("{input_string}");
}

fn makes_copy(num: i8) {
    println!("{num}");
}

fn calculate_length(/*s: &String*/ s2: &mut String) -> usize {
    // s.push_str(", world");   --> error as "s" -> borrows data of s2 (by reference)

    // s.len()

    s2.push_str(",world");
    s2.len()
}

fn immutable_and_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    // let r3 = &mut s;

    // println!("{r1}, {r2}, and {r3}");  ---> error ? - > mutable borrow must be after the use of all imutable borrows

    println!("{r1}, {r2}");
    let r3 = &mut s;

    println!("{r3}");
}

fn test_slicing() {
    let s = String::from("hello this is kp");

    let first_word = get_first_word(&s);

    // s.clear();  -->> error as s is empties now and first_word hast reference to first word of s

    // println!("first word : {first_word}");

    // array slicing

    let arr = [10, 20, 30, 40, 50];

    let arr_slice = &arr[..=3];

    println!("{:?}", arr_slice);
}

fn get_first_word(s: &String) -> &str {
    // &str -> type for string slice

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
