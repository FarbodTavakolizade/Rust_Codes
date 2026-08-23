# 🦀 Rust Codes — Learn Rust with 99 Hands-On Lessons

![Rust](https://img.shields.io/badge/language-Rust-DEA584?logo=rust&logoColor=black)
![Lessons](https://img.shields.io/badge/lessons-99-orange)
![Level](https://img.shields.io/badge/level-beginner%20%E2%86%92%20advanced-green)

> This repository contains the source code of a complete **Rust programming course** — starting from your very first `Hello, World!` and going all the way to **ownership**, **lifetimes**, **smart pointers**, **async programming with Tokio**, and real-world projects. 🚀
>
> Each lesson lives in its own numbered folder with clean, focused example code. You are free to use it for **learning, teaching, or as a quick reference**.
>
> Rust is: 🚀 Fast · 🔒 Safe · 🎨 Fun

---

## 📚 Course Roadmap

### 🎯 Getting Started
| Lesson | Topic |
|--------|-------|
| 01 | Hello World — your first Rust program |
| 02–04 | Variables, shadowing & scopes, constants |
| 05–06 | Type aliases, compiler directives, data types |
| 07–08 | Strings, math operators, type casting, formatting |
| 09–10 | Booleans & characters |
| 11, 14 | Arrays & tuples |
| 12 | `dbg!` macro & trait intro |
| 15 | User input |
| 16–17 | Functions, arguments & return values |

### 🔀 Control Flow
| Lesson | Topic |
|--------|-------|
| 18 | `if` / `else if` / `else` |
| 19 | `match` statements |
| 20–21 | `loop`, `break`, `continue` & `while` |

### 🔒 Ownership & Borrowing — the Heart of Rust ❤️
| Lesson | Topic |
|--------|-------|
| 22 | `Copy` trait intro |
| 23.0–23.1 | `String` vs `&str` |
| 24 | Move semantics |
| 25 | `Drop` & `Clone` |
| 26–28 | Borrowing, references (mutable & immutable) |
| 29 | Mutable arrays |
| 30 | Slices |
| 13, 31 | Iteration with `for` |

### 🏗️ Structs, Methods & Macros
| Lesson | Topic |
|--------|-------|
| 32–38 | Structs (classic, tuple, unit) |
| 39 | `Debug` trait |
| 40–43 | Methods & `self` |
| 44–45 | Associated functions |
| 46 | Builder pattern |
| 47 | Macros |
| 48 | Smart pointers |

### 📦 Collections
| Lesson | Topic |
|--------|-------|
| 49–50 | `isize`/`usize` & unions |
| 51 | Vectors |
| 52 | HashMap |

### 🎛️ Enums & Option
| Lesson | Topic |
|--------|-------|
| 53–59 | Enums in depth |
| 60–61 | The `Option` enum |

### 🧠 Functional Rust
| Lesson | Topic |
|--------|-------|
| 62 | Closures |
| 63 | Iterators |

### 🧬 Generics & Traits
| Lesson | Topic |
|--------|-------|
| 64–66 | Generics |
| 67–70 | Traits |

### 📁 Files, Errors & Lifetimes
| Lesson | Topic |
|--------|-------|
| 71 | File handling |
| 72–73 | `panic!` and error handling |
| 74 | `assert` macros |
| 77–78 | Lifetimes & `'static` |

### 🧵 Concurrency & Async
| Lesson | Topic |
|--------|-------|
| 79–81 | Threads |
| 82 | Tokio intro |
| 83 | Async broadcast chat server project |

### 🏆 Projects & Practice
| Lesson | Topic |
|--------|-------|
| 75–76 | Phonebook app & HashSet |
| 84 | Password cracker (with example word list) |
| 85–94 | Guess game, Armstrong, primes, palindrome, factorial, Fibonacci, stars, Wordle |
| 95 | Simple management system |
| 96 | 🐍 Snake |
| 97 | CoinMarketCap API client |
| 98 | ❌⭕ Tic-Tac-Toe |

---

## 🚀 Getting Started

**1. Install Rust** (if you haven't):

<details>
<summary>🐧 Linux / macOS</summary>

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details>
<summary>🪟 Windows</summary>

- Download and run [rustup-init.exe](https://win.rustup.rs/x86_64)
</details>

**2. Clone the repository:**
```bash
git clone https://github.com/FarbodTavakolizade/Rust_Codes.git
cd Rust_Codes
```

**3. Run a lesson:**

Most lessons are single-file programs — you can try them instantly in your editor, or with `rustc`:
```bash
rustc "01 👋🌍 Hello World/01 👋🌍 Hello World.rs" -o hello && ./hello
```

> 💡 **Tip:** Lessons that use external crates (like `rand`, `reqwest`, or `tokio` — e.g. lessons 82–83, 85, 89, 94, 96–97) need a Cargo project. Quick way:
> ```bash
> cargo new mylesson && cd mylesson
> # copy the lesson code into src/main.rs and add the crates to Cargo.toml
> cargo run
> ```
>
> You can also paste any lesson into the [Rust Playground](https://play.rust-lang.org) to run it in your browser. 🌐

---

## 📂 Repository Structure

```
Rust_Codes/
├── 01 👋🌍 Hello World/
│   └── 01 👋🌍 Hello World.rs
├── 02 📦Variable/
│   └── 02 📦Variable.rs
├── ...
├── 84 password_cracker/
│   ├── 84 password_cracker.rs
│   └── 84.1 password example.txt
├── 98_Tic_Tac_Toe/
│   └── 98_Tic_Tac_Toe.rs
└── README.md
```

Every folder is named after its lesson number and topic — find the code you need in seconds. 🔍

---

## ⭐ Support

If this repository helped you learn Rust, consider giving it a **star** — it helps others find it too!

> *"Rust prevents bugs before they reach production."* 💡
