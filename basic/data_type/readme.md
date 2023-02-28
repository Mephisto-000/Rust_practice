# 資料型別

[參考網址](https://rust-lang.tw/book-tw/ch03-02-data-types.html)
Rust 是一門**靜態型別**語言，這代表它必須在編譯時知道所有變數的型別。



## 1. 純量型別（scalar）

Rust 有四種主要純量型別：整數、浮點數、布林以及字元。

### 1-1. 整數型別

|   長度   |  帶號   | 非帶號  |
| :------: | :-----: | :-----: |
|  8位元   |  `i8`   |  `u8`   |
|  16位元  |  `i16`  |  `u16`  |
|  32位元  |  `i32`  |  `u32`  |
|  64位元  |  `i64`  |  `u64`  |
| 128位元  | `i128`  | `u128`  |
| 系統架構 | `isize` | `usize` |

$\star$ 能適用於數種數字型別的數字字面值都允許在最後面加上型別，比如說用 `57u8` 來指定型別。數字字面值也可以加上底線 `_` 分隔方便閱讀，比如說 `1_000` 其實就和指定 `1000` 的數值一樣。

$\star$ 前綴`0x`、`0o`、`0b`代表16進位制、8進位制、2進位制，常值`b'X'`帶表字元`X`的ASCII碼的`u8`值。

Example:
```rust
fn main() {
    let test1 = 98_200;             // 98200
    let test2 = 0xff;               // 255
    let test3 = 0o77;               // 63
    let test4 = 0b1111_0000;        // 240
    let test5 = b'X';               // 88


    println!("test1 10進位測試 : {test1}");
    println!("test2 16進位測試 : {test2}");
    println!("test3 8 進位測試 : {test3}");
    println!("test4 2 進位測試 : {test4}");
    println!("test5 位元組（僅限u8）測試 : {test5}");

}
```



### 1-2. 浮點數型別

|  長度  | 全為帶號 |                     範圍                     |
| :----: | :------: | :------------------------------------------: |
| 32位元 |  `f32`   |  $-3.4\cdot10^{38}$ 至 $+3.4\cdot 10^{38}$   |
| 64位元 |  `f64`   | $-1.8\cdot 10^{308}$ 至 $+1.8\cdot 10^{308}$ |

Example:
```rust
fn main() {

    let test6 = 31415.926e-4f64;    // 3.1415926

    println!("test6 浮點數測試 : {test6}");

}
```

說明：
`e-4`為乘上10為底指數的部份$10^{-4}$ ，`f64` 為型態後綴詞，代表64位元浮點數。



### 1-3. 數值運算

Example:
```rust
fn main() {

    // 加法
    let sum = 5 + 10;                     // 15
    
    // 減法
    let difference = 95.5 - 4.3;          // 91.2
    
    // 乘法
    let product = 4 * 30;                 // 120

    // 除法
    let quotient = 56.7 / 32.2;           // 1.7608695652173911
    let truncated = -5 / 3;               // -1

    // 取餘
    let remainder = 43 % 5;               // 3


    println!("sum : {sum}");
    println!("difference : {difference}");
    println!("product : {product}");
    println!("quotient : {quotient}");
    println!("truncated : {truncated}");
    println!("remainder : {remainder}");

}
```



### 1-4. 布林型別

Example:
```rust
fn main() {

    let t = true;
    let f: bool = false; // 型別詮釋的方式

    println!("t : {t}");
    println!("f : {f}");

}
```



### 1-5. 字元型別

Example:
```rust
fn main() {

    let c = 'z';
    let z: char = 'Z'; // 明確標註型別寫法
    let heart_eyed_cat = '😻';


    println!("c : {c}");
    println!("z : {z}");
    println!("heart_eyed_cat : {heart_eyed_cat}");

}
```

$\star$ 注意到 `char` 字面值是用單引號賦值，宣告字串字面值時才是用雙引號。Rust 的 `char` 型別大小為四個位元組並表示為一個 Unicode 純量數值，這代表它能擁有的字元比 ASCII 還來的多。舉凡標音字母（Accented letters）、中文、日文、韓文、表情符號以及零長度空格都是 Rust `char` 的有效字元。