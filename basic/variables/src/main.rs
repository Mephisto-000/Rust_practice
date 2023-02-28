const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("常數：{THREE_HOURS_IN_SECONDS}");

    let first_variable = 7;         // 不可變的
    let mut second_variable = 7;   // 可變的

    println!("first_variable (original) : {first_variable}");
    println!("second_variable (original) : {second_variable}");

    //此段敘述會出錯：
    // first_variable = 8;
    // println!("first_variable : {first_variable}");
    //此段可以過：
    second_variable = 8;
    println!("second_variable (changed) : {second_variable}");

    // 遮蔽說明：

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x 在內部範圍的數值為：{x}");
    }
    
    println!("x 的數值為：{x}");
}
