# Hello Cargo

Cargo 是 Rust 的建構系統與套件管理工具。[參考網址](https://rust-lang.tw/book-tw/ch01-03-hello-cargo.html)

測試是否有裝 Cargo :
```shell
$ cargo --version
```



## 1. 使用 Cargo 建立專案

建立 *hello_cargo* 專案：

```shell
$ cargo new hello_cargo
```

$\star$ 如果已經在 Git repository 內的話，執行上面的指令不會產生 Git 的檔案，可以用下述指令去覆寫：
```shell
$ cargo new --vcs=git hello_cargo
```

專案建立後，生成的檔案會有：
```shell
.
└── hello_cargo
    ├── Cargo.toml
    └── src
        └── main.rs
```



### 1-1. Cargo.toml

檔案 `Cargo.toml` 內容為：
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

說明：

1. 第一行的 `[package]` 是一個段落（section）標題，說明以下的陳述式（statement）會配置這個套件。隨著我們加入更多資訊到此文件，我們也會加上更多段落。
2. 接下來三行就是 Cargo 編譯你的程式所需的配置資訊：名稱、版本、誰寫的以及哪個 Rust `edition` 會用到。
3. 最後一行 `[dependencies]` 是用來列出你的專案會用到哪些依賴的段落。在 Rust 中，程式碼套件會被稱為 *crates*。我們在此專案還不需要任何其他 crate。



### 1-2. src/main.rs

檔案 `main.rs` 內容為：
```rust
fn main() {
    println!("Hello, world!");
}
```

說明：
Cargo 預設會為你產生一個「Hello, world!」程式，並且 Cargo 預期你的原始檔案都會放在 *src* 目錄底下。專案的根目錄是用來放 README 檔案、授權條款、配置檔案以及其他與你的程式碼不相關的檔案。



## 2. 建構並執行 Cargo 專案

在你的 *hello_cargo* 目錄下輸入以下命令來建構專案：
```shell
$ cargo build
	 Compiling hello_cargo v0.1.0 (/home/mephisto/github/Rust_practice/basic/hello_cargo)
     Finished dev [unoptimized + debuginfo] target(s) in 0.37s
```

此命令會產生一個執行檔 *target/debug/hello_cargo* ，而不是在你目前的目錄。因為預設的建構會是 debug build，Cargo 會將執行檔放進名為 *debug* 的目錄。

可以用以下命令運行執行檔：
```shell
$ ./target/debug/hello_cargo
Hello, world!
```

$\star$ 第一次執行 `cargo build` 的話，還會在根目錄產生另一個新檔案：*Cargo.lock*。此檔案是用來追蹤依賴函式庫的確切版本。不過此專案沒有任何依賴。你不會需要去手動更改此檔案，Cargo 會幫你管理這個檔案的內容。

我們也可以只用一道命令 `cargo run` 來編譯程式碼並接著運行產生的執行檔：
```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

$\star$ 請注意到這次輸出的結果我們沒有看到 Cargo 有在編譯 `hello_cargo` 的跡象，這是因為 Cargo 可以知道檔案完全沒被更改過，所以它不用重新建構可以選擇直接執行執行檔。

Cargo 還提供一道命令 `cargo check`，此命令會快速檢查你的程式碼，確保它能編譯通過但不會產生執行檔：
```shell
$ cargo check
    Checking hello_cargo v0.1.0 (/home/mephisto/github/Rust_practice/basic/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
```

$\star$ `cargo check` 省略了產生執行檔的步驟，所以它執行的速度比 `cargo build` 還來的快。如果你在寫程式時需要持續檢查的話，使用 `cargo check` 可以加快整體過程，讓你知道你的專案是否能編譯！所以許多 Rustaceans 都會在寫程式的過程中時不時執行 `cargo check` 來確保它能編譯。最後當他們準備好要使用執行檔時，才會用 `cargo build`。



- 我們可以用 `cargo new` 產生專案。
- 我們可以用 `cargo build` 建構專案。
- 我們可以用 `cargo run` 同時建構並執行專案。
- 我們可以用 `cargo check` 建構專案來檢查錯誤，但不會產生執行檔。
- Cargo 會儲存建構結果在 *target/debug* 目錄底下，而不是放在與我們程式碼相同的目錄。



## 3. 建構發佈版本（Release）

當你的專案正式準備好要發佈的話，你可以使用 `cargo build --release` 來最佳化編譯結果。此命令會產生執行檔到 *target/release* 而不是 *target/debug*。最佳化可以讓你的 Rust 程式碼跑得更快，不過也會讓編譯的時間變得更久。這也是為何 Cargo 提供兩種不同的設定檔（profile）：一個用來作為開發使用，讓你可以快速並經常重新建構；另一個用來最終產生你要給使用者運行的程式用，它通常不會需要重新建構且能盡所能地跑得越快越好。如果你要做基準化分析（benchmarking）來檢測程式運行時間的話，請確認執行的是 `cargo build --release` 並使用 *target/release* 底下的執行檔做檢測。
