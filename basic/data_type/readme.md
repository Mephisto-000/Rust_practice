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



## 2. 複合型別（compound）

Rust 有兩個基本複合型別：元組（tuples）和陣列（arrays）。

### 2-1. 元組（Tuple）型別

**元組**是個將許多不同型別的數值合成一個複合型別的常見方法。元組擁有固定長度：一旦宣告好後，它們就無法增長或縮減。

Example:
```rust
fn main() {

    let tup1: (i32, f64, u8) = (500, 6.4, 1);


    println!("tup1 is : {:?}", tup1);
    println!("first element in tup1 is : {:?}", tup1.0);

}
```

此變數 `tup1` 就是整個元組，因為一個元組就被視為單一複合元素，我們也可以直接用句號（`.`）再加上數值的索引來取得元組內的元素。要拿到元組中的每個獨立數值的話，我們可以用模式配對（pattern matching）來解構一個元組的數值，如以下所示：
```rust
fn main() {

    let tup2 = (400, 7.5, 2);
    let (x, y, k) = tup2;


    println!("y : {y}");

}
```

此程式先是建立了一個元組然後賦值給 `tup`，接著它用模式配對和 `let` 將 `tup2` 拆成三個個別的變數 `x`、`y` 和 `k`。這就叫做**解構（destructuring）**，因為它將單一元組拆成了三個部分。最後程式將 `y` 的值印出來，也就是 `7.5`。

$\star$ 沒有任何數值的元組有一種特殊的名稱叫做**單元型別（Unit）**，其數值與型別都寫作 `()`，通常代表一個空的數值或空的回傳型別。表達式要是沒有回傳任何數值的話，它們就會隱式回傳單元型別。

### 2-2. 陣列（Arrays）型別

另一種取得數個數值集合的方法是使用**陣列**。和元組不一樣的是，**陣列中的每個型別必須是一樣的**。和其他語言的陣列不同，**Rust 的陣列是固定長度的**。

Example:
```rust
fn main() {

    let array_1 = [1, 2, 3, 4, 5];

    
    println!("array_1 : {:?}", array_1);

}
```

當你想要你的資料被分配在堆疊（stack）而不是堆積（heap）的話，使用陣列是很好的選擇。或者當你想確定你永遠會取得固定長度的元素時也是。所以陣列不像向量（vector）型別那麼有彈性，**向量**是標準函式庫提供的集合型別，類似於陣列但**允許**變更長度大小。**如果你不確定該用陣列或向量的話，通常你應該用向量就好**。

要詮釋陣列型別的話，你可以在中括號寫出型別和元素個數，並用分號區隔開來，如以下所示：
```rust
fn main() {

    let array_2: [i32; 3] = [21, 56, 17];

    
    println!("array_2 : {:?}", array_2);

}
```

如果你想建立的陣列中每個元素數值都一樣的話，你可以指定一個數值後加上分號，最後寫出元素個數。如以下所示：
```rust
fn main() {

    let array_3 = [7; 4];

    
    println!("array_3 : {:?}", array_3);

}
```

一個陣列是被分配在堆疊上且已知固定大小的一整塊記憶體，你可以用索引來取得陣列的元素，比如：
```rust
fn main() {

    let array_1 = [1, 2, 3, 4, 5];
    let array_1_first = array_1[0];

    
    println!("array_1 : {:?}", array_1);
    println!("array_1 first element is : {array_1_first}");

}
```

在此範例中，變數 `first` 會得到數值 `1`，因為這是陣列索引 `[0]` 的數值。變數 `second` 則會從陣列索引 `[1]` 得到數值 `2`。