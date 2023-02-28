# 變數與可變性

[參考網址](https://rust-lang.tw/book-tw/ch03-01-variables-and-mutability.html)

## 1. 可變性

使用 `let` 陳述式建立**變數**(variable)，在 Rust 中，**變數預設是不可變的(immutable)**，要讓變數成為可變的話，我們可以在變數名稱前面加上`mut`，如下：

```rust
let first_variable = 7;         // 不可變的
let mut second_variable = 7;   // 可變的
```

說明：
當我們嘗試改變一個原先設計為不可變的變數時，能夠產生編譯時錯誤是很重要的。因為這樣的情況很容易導致程式錯誤。如果我們有一部分的程式碼在執行時認為某個數值絕對不會改變，但另一部分的程式碼卻更改了其值，那麼這就有可能讓前一部分的程式碼就可能以無法預測的方式運行。這樣的程式錯誤的起因是很難追蹤的，尤其是當第二部分的程式碼**偶而**才會改變其值。Rust 編譯器會保證當你宣告一個數值不會被改變時，它就絕對不會被改變。這樣你就不需要去追蹤該值可能會被改變，讓你的程式碼更容易推導。

## 2. 常數

**常數（constants）**。和不可變變數一樣，常數會讓數值與名稱綁定且不允許被改變，但是不可變變數與常數還是有些差異。無法在使用常數使用 `mut`，常數不是預設不可變，它們永遠都不可變。如果你使用 `const` 宣告而非 `let` 的話，你**必須**指明型別：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## 3. 遮蔽（Shadowing）

我們可以用 `let` 關鍵字來重複宣告相同的變數名稱來遮蔽一個變數，第一個變數被第二個變數所**遮蔽**了，這代表當你使用該變數名稱時，編譯器會看到的是第二個變數的數值。第二個變數會遮蔽第一個變數，佔據變數名稱的使用權，直到它自己也被遮蔽或是離開作用域：

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x 在內部範圍的數值為：{x}");
    }

    println!("x 的數值為：{x}");
}
```

說明：
此程式首先將 `x` 給予 `5`，然後它重複用 `let x =` 建立一個新變數 `x`，取代了原本的數值並加上 `1`，所以 `x` 的數值變為 `6`。然後在接下來括號的內部範圍內，第三次的 `let` 陳述式一樣遮蔽了 `x` 讓它將原本的值乘與 `2`，讓 `x` 數值為 `12`。當該範圍結束時，內部的遮蔽也結束，所以 `x` 就回到原本的 `6`。

$\star$ **注意**：

1. 遮蔽與標記變數為 `mut` 是不一樣的，因為如果我們不小心重新賦值而沒有加上 `let` 關鍵字的話，是會產生編譯期錯誤的。使用 `let` 的話，我們可以作出一些改變，然後在這之後該變數仍然是不可變的。

2. 另一個 `mut` 與遮蔽不同的地方是，我們能有效地再次運用 `let` 產生新的變數，可以在重新運用相同名稱時改變它的型別。
   example：

   ```rust
   let spaces = "   ";
   let spaces = spaces.len();
   ```

   說明：以上面程式舉例，當我們希望程式要求使用者顯示出字串間應該顯示多少空格，同時我們又希望它被存為一個數字。第一次宣告 `spaces` 的變數是一個字串型別，而第二次宣告 `spaces` 則成了數字型別。遮蔽這項功能讓我們不必去宣告像是 `spaces_str` 與 `spaces_num`，我們可以重複使用 `spaces` 這個變數名稱。