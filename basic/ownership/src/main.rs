fn main() {
    let s = String::from("hello"); // s 進入作用域

    takes_ownership(s);            // s 的值進入函式
                                   // 所以 s 也在此無效

    let x = 5;                     // x 進入作用域

    makes_copy(x);                 // x 本該移動進函式裡
                                   // 但 i32 有 Copy，所以 x 可以繼續使用

    let s1 = gives_ownership();                // gives_ownership 移動它的回傳值給 s1

    let s2 = String::from("哈囉");             // s2 進入作用域

    let s3 = takes_and_gives_back(s2);        // s2 移入 takes_and_gives_back
                                              // 該函式又將其回傳值移到 s3

    let (s4, len) = calculate_length(s1);

    println!("'{}' 的長度為 {}。", s4, len);


} // x 在此離開作用域，接著是 s 。但因為 s 的值已經被移動了
  // 它不會有任何動作。
  // s3 在此離開作用域並釋放
  // s2 已被移走，所以沒有任何動作發生
  // s1 離開作用域並釋放


fn takes_ownership(some_string: String) { // some_string 進入作用域
    println!("{}", some_string);
} // some_string 在此離開作用域並呼叫 'drop'
  // 佔用的記憶體被釋放


fn makes_copy(some_integer: i32) { // some_integer 進入作用域
    println!("{}", some_integer);
} // some_integer 在此離開作用域，沒有任何動作發生


fn gives_ownership() -> String {                         // gives_ownership 會將他的回傳值
                                                         // 移動給呼叫它的函式

    let some_string = String::from("Hello");             // some_string 進入作用域

    some_string                                          // 回傳 some_string 並移動給
}                                                        // 呼叫它的函式

// 此函式會取得一個 String 然後回傳它
fn takes_and_gives_back(a_string: String) -> String {

    a_string                                             // 回傳 a_string 並移動給呼叫的函式
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}