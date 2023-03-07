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

    println!("＝＝元組型別測試範例＝＝：");
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (400, 7.5, 2);
    let (x, y, k) = tup2;


    println!("tup1 is : {:?}", tup1);
    println!("first element in tup1 is : {:?}", tup1.0);
    println!("y : {y}");

    println!("＝＝陣列型別測試範例＝＝：");
    let array_1 = [1, 2, 3, 4, 5];
    let array_2: [i32; 3] = [21, 56, 17];
    let array_3 = [7; 4];
    let months = ["一月", "二月", "三月", 
                  "四月", "五月", "六月", 
                  "七月", "八月", "九月", 
                  "十月", "十一月", "十二月"];
    let array_1_first = array_1[0];

    
    println!("array_1 : {:?}", array_1);
    println!("array_2 : {:?}", array_2);
    println!("array_3 : {:?}", array_3);
    println!("months : {:?}", months);
    println!("length of months array : {}", months.len());
    println!("array_1 first element is : {array_1_first}");

}
