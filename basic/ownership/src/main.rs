fn main() {
    let s = String::from("hello");

    takes_ownership(s);


    let x = 5;

    makes_copy(x);


    let s1 = gives_ownership();

    let s2 = String::from("哈囉");

    let s3 = takes_and_gives_back(s2);

    let (s4, len) = calculate_length(s1);

    println!("'{}' 的長度為 {}。", s4, len);


}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn gives_ownership() -> String {

    let some_string = String::from("Hello");

    some_string
}


fn takes_and_gives_back(a_string: String) -> String {

    a_string
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}