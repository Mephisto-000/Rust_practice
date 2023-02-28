fn main() {
    let test1 = 98_200;             // 98200
    let test2 = 0xff;               // 255
    let test3 = 0o77;               // 63
    let test4 = 0b1111_0000;        // 240
    let test5 = b'X';               // 88
    let test6 = 31415.926e-4f64;    // 3.1415926


    println!("test1 10進位測試 : {test1}");
    println!("test2 16進位測試 : {test2}");
    println!("test3 8 進位測試 : {test3}");
    println!("test4 2 進位測試 : {test4}");
    println!("test5 位元組（僅限u8）測試 : {test5}");
    println!("test6 浮點數測試 : {test6}");

    println!("＝＝數值計算測試範例＝＝：");
    // 加法
    let sum = 5 + 10;
    
    // 減法
    let difference = 95.5 - 4.3;
    
    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // 取餘
    let remainder = 43 % 5;


    println!("sum : {sum}");
    println!("difference : {difference}");
    println!("product : {product}");
    println!("quotient : {quotient}");
    println!("truncated : {truncated}");
    println!("remainder : {remainder}");

    println!("＝＝布林型別測試範例＝＝：");
    let t = true;
    let f: bool = false; // 型別詮釋的方式


    println!("t : {t}");
    println!("f : {f}");

    println!("＝＝字元型別測試範例＝＝：");
    let c = 'z';
    let z: char = 'Z'; // 明確標註型別寫法
    let heart_eyed_cat = '😻';


    println!("c : {c}");
    println!("z : {z}");
    println!("heart_eyed_cat : {heart_eyed_cat}");

}
