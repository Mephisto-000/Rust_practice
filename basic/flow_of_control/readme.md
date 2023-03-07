# 控制流程

[參考網址](https://rust-lang.tw/book-tw/ch03-05-control-flow.html)  

在大多數程式語言中，能夠決定依據某項條件是否為 `true` 來執行些程式碼，以及依據某項條件是否為 `true` 來重複執行些程式碼是非常基本的組成元件。在 Rust 程式碼中能讓你控制執行流程的常見方法有 `if` 表達式以及迴圈。

## 1. if 表達式

Example:
```rust
fn main() {
    
    let number = 6;

    if number < 5 {
        println!("條件為真");
    } else {
        println!("條件為否");
    }

}
```

$\star$ 程式碼的條件判斷**必須**是 `bool`。

想要實現多重條件的話，你可以將 `if` 和 `else` 組合成 `else if` 表達式。
Example:

```rust
fn main() {

    if number % 4 == 0 {
        println!("數字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("數字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("數字可以被 2 整除");
    } else {
        println!("數字無法被 4、3、2 整除");
    }

}
```

因為 `if` 是表達式，所以我們可以放在 `let` 陳述式的右邊，將結果賦值給變數。
Example:

```rust
fn main() {

    let condition = true;
    let number_2 = if condition { 5 } else { 6 };
    println!("number_2 數字結果為：{number_2}");

}
```

$\star$ **每一個 `if` 分支必須要是相同型別**。在上方範例中，各分支的型別都是 `i32`。如果型別不一致的話，就會出錯。

## 2. 使用迴圈重複執行

Rust 提供三種迴圈：`loop`、`while` 和 `for`。

### 2-1. 使用 loop

其中一種使用 `loop` 的用途是重試某些你覺得會失敗的動作，像是檢查一個執行緒是否已經完成其任務。這樣你可能就會想傳遞任務結果給之後的程式碼。要做到這樣的事，你可以在你要用來停下迴圈的 `break` 表達式內加上一個你想回傳數值，該值就會被停止的迴圈回傳。
Example:

```rust
fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("迴圈結果 result 為：{result}");

}
```

在迴圈之前，我們宣告了一個變數 `counter` 並初始化為 `0`，然後我們宣告了另一個變數 `result` 來取得迴圈回傳的值。在迴圈每一次的疊代中，我們將變數 `counter` 加上 `1` 並檢查它是否等於 `10`。如果是的話就用 `break` 關鍵字回傳 `counter * 2`。在迴圈結束後，我們用分號才結束這個賦值給 `result` 的陳述式。最後我們印出 `result`，而結果為 `20`。

如果你有迴圈在迴圈之內的話，`break` 和 `continue` 會用在該位置最內層的迴圈中。你可以選擇在迴圈使用**迴圈標籤（loop label）**，然後使用 `break` 和 `continue` 加上那些迴圈標籤定義的關鍵字，而不是作用在最內層迴圈而已。以下是使用雙層巢狀迴圈的範例：
```rust
fn main() {

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

}
```

外層迴圈有個 `'counting_up` 的標籤，而且其會從 0 數到 2。而內層沒有迴圈標籤的迴圈則會從 10 數到 9。第一個 `break` 沒有指定任何標籤只會離開內層迴圈。而陳述式 `break 'counting_up;` 則會離開外層迴圈。

### 2-2. 使用 while

Example:
```rust
fn main() {

    let mut number_3 = 3;

    while number_3 != 0 {
        println!("{number_3}");

        number_3 -= 1;
    }

    println!("number_3 result : {number_3}");

}
```

這樣消除了很多使用 `loop`、`if`、`else` 與 `break` 會有的巢狀結構，這樣可以更易閱讀。

### 2-3. 使用 for

你可以用 `while` 來遍歷一個集合的元素，像是陣列等等。
Example:

```rust
fn main() {

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("數值為：{}", a[index]);

        index += 1;
    }

}
```

所有五個元素都如預期顯示在終端機上。儘管 `index` 會在某一刻達到 `5`，但是迴圈會在嘗試取得陣列第六個元素前就停止執行。

但**這樣的方式是容易出錯的，我們可能取得錯誤的索引數值或測試條件而造成程式恐慌。這同時也使程式變慢，因為編譯器得在執行時的程式碼對迴圈中每次疊代中進行索引是否在陣列範圍內的條件檢查**。

所以更簡潔的替代方案是，你可以使用 `for` 迴圈來對集合的每個元素執行一些程式碼。
Example:

```rust
fn main() {

    let b = [20, 40, 60, 80, 100];

    for element in b {
        println!("數值為：{element}");
    }

}
```

上面程式我們增加了程式的安全性，去除了造成程式錯誤的可能性。不會出現超出陣列大小或是讀取長度不足的風險。

`for` 迴圈的安全性與簡潔程度讓它成為 Rust 最常被使用的迴圈結構。就算你想執行的是依照次數循環的程式碼，像是範例 3-3 的 `while` 迴圈範例，多數 Rustaceans 還是會選擇 `for` 迴圈。要這麼做的方法是使用 `Range`，這是標準函式庫提供的型別，用來產生一連串的數字序列，從指定一個數字開始一直到另一個數字之前結束。

另一種寫法：
```rust
fn main() {

    for n_index in (1..4).rev() {
        println!("{n_index}!");
    }

}
```

