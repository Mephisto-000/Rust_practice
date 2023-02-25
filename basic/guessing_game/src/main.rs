use std::io;  // 將 io (輸入/輸出)函式庫引入，io 函式庫來自標準函式庫(std)
use std::cmp::Ordering;
use rand::Rng;

fn main() {  // main 函式是程式的入口點(entry point)
             // fn 語法來宣告新的函式

    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("祕密數字為：{secret_number}");

    loop {
        println!("請輸入你的猜測數字。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜測數字：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("獲勝！");
                break;
            }
        }
    }
}