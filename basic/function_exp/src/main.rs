fn main() {
    println!("Hello, world!");

    another_function();
    function_1(7);
    function_2(77, 'L');

    let k = function_3();
    println!("k 的數值為：{k}");

    let t = plus_one(9);
    println!("t 的數值為：{t}");

}

fn another_function() {
    println!("另一支函式。");
}

fn function_1(x: i32) {
    println!("x 的數值為：{x}");
}

fn function_2(x: i32, unit_label: char) {
    println!("代號為：{unit_label}，數值為：{x}");
}

fn function_3() -> i32 {
    return 9
}

fn plus_one(x: i32) -> i32 {
    x + 1
}