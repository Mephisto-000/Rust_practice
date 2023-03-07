fn main() {
    println!("==== if else 範例測試： =====");
    let number = 6;

    if number < 5 {
        println!("條件為真");
    } else {
        println!("條件為否");
    }


    if number % 4 == 0 {
        println!("數字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("數字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("數字可以被 2 整除");
    } else {
        println!("數字無法被 4、3、2 整除");
    }


    let condition = true;
    let number_2 = if condition { 5 } else { 6 };
    println!("number_2 數字結果為：{number_2}");

    println!("==== 迴圈範例測試 1 ： ====");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("迴圈結果 result 為：{result}");

    println!("==== 迴圈範例測試 2 ： ====");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("==== 迴圈範例測試 3 ： ====");
    let mut number_3 = 3;

    while number_3 != 0 {
        println!("{number_3}");

        number_3 -= 1;
    }

    println!("number_3 result : {number_3}");

    println!("==== 迴圈範例測試 4 ： ====");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("數值為：{}", a[index]);

        index += 1;
    }

    println!("==== 迴圈範例測試 5 ： ====");
    let b = [20, 40, 60, 80, 100];

    for element in b {
        println!("數值為：{element}");
    }

    println!("==== 迴圈範例測試 6 ： ====");
    for n_index in (1..4).rev() {
        println!("{n_index}!");
    }

}
