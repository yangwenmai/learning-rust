# 运行

```sh
$ cargo new hello-world-cargo
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `./learning-rust/hello-world-cargo/target/debug/hello-world-cargo`
Hello, Rust use cargo!
```
----

## cargo command help

cargo 工具本身为我们提供了各种好用的命令。大家可以在终端中输入 `cargo --help` 或 `cargo -h` 或 `cargo help` 查看。

### cargo new

`cargo new hello-world-cargo --bin`

>用于在当前目录下新建基于 cargo 项目管理的 rust 项目，项目名称为 `hello-world-cargo`，`--bin` 表示该项目将生成可执行文件，还可以指定为 `--lib` 表示该项目将生成一个库。

```sh
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

### cargo build

>用于构建项目。

### cargo run

>用于运行可执行项目。

如果先执行 `cargo build` 然后在执行 `cargo run` 之前没有对之前 `build` 的代码做修改的话，是直接使用之前编译的结果来运行的，否则会重新再编译，然后再执行。

### cargo update

>根据 `Cargo.toml` 描述文件重新检索并更新各种依赖项的信息，并写入 `Cargo.lock` 文件，例如依赖项版本的更新变化等。

### cargo install

>可用于实际的生产部署。

cargo run 与 cargo install 的区别：

cargo run 生成的 target 是：`hello-world-cargo/target/debug`。
cargo install 生成的 target 是：`hello-world-cargo/target/release`。

### cargo clean

>用于清理 `target` 文件夹的所有内容（包括 `target` 文件夹）。

## cargo 目录结构

`src`: 源代码。
`Cargo.toml`: 存储了项目的所有信息。
`Cargo.lock`: cargo 工具根据 toml 文件生成的项目依赖详细清单文件，不需要我们去修改这个文件。

## 依赖

当我们在 `main.rs` 头部引入：

```rust
extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap();
    println!("Did our date is match?{}", re.is_match("2018-08-13"));
}
```

我们必须事先在 `Cargo.toml` 中添加 `regex` 的依赖配置，否则报错： `can't find crate for regex`。
那我们要引入 `regex` ，应该用哪个版本呢？我们可以从 [https://crates.io/](https://crates.io/) 上去找。然后添加到 `Cargo.toml` 的 [dependencies] 版块，我们再执行 `cargo run` 或者 `cargo build` 就会自动把它的相关依赖项写入 `Cargo.lock`。

注意 https://crates.io/ 不是 https://crate.io/ 。

执行过程：

```sh
$ cargo build
Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling version_check v0.1.4
   Compiling libc v0.2.43
   Compiling void v1.0.2
   Compiling ucd-util v0.1.1
   Compiling regex v1.0.2
   Compiling utf8-ranges v1.0.0
   Compiling unreachable v1.0.0
   Compiling regex-syntax v0.6.2
   Compiling lazy_static v1.1.0
   Compiling memchr v2.0.1
   Compiling aho-corasick v0.6.6
   Compiling thread_local v0.3.5
   Compiling hello-world-cargo v0.1.0 (file:///Users/maiyang/develop/goworkspace/src/github.com/developer-learning/learning-rust/hello-world-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 20.05s
```

## 参考资料

1. [Cargo Book](https://doc.rust-lang.org/cargo/index.html)
2. [https://doc.rust-lang.org/cargo/guide/dependencies.html](https://doc.rust-lang.org/cargo/guide/dependencies.html)