# 参考答案（solutions）

这里放着 `assessments/` 下 18 个考核文件的**参考答案**：每个练习函数该写成什么，都能在对应的
`solutions/lesson_XX_<主题>.rs` 里看到。

> **先自己写，再看答案。** 考核的意义是验证「你是真的会了」；直接照抄答案，报告上会是满分，
> 但你并没有获得那份能力。建议的顺序是：
> ① 读 `#[test]` 明确验收标准 → ② 自己实现 → ③ 跑测试 → ④ **实在卡住**（比如超过 20 分钟）
> 再看这里对应那一题 → ⑤ 看完合上答案，自己重写一遍。

---

## 一、文件对照

| 课 | 考核文件 | 参考答案 | 练习函数 | 知识点 |
| --- | --- | --- | --- | --- |
| 01 | `assessments/lesson_01_variables_mutability.rs` | [`lesson_01_variables_mutability.rs`](lesson_01_variables_mutability.rs) | 8 | 8 |
| 02 | `assessments/lesson_02_data_types.rs` | [`lesson_02_data_types.rs`](lesson_02_data_types.rs) | 8 | 8 |
| 03 | `assessments/lesson_03_functions.rs` | [`lesson_03_functions.rs`](lesson_03_functions.rs) | 8 | 8 |
| 04 | `assessments/lesson_04_control_flow.rs` | [`lesson_04_control_flow.rs`](lesson_04_control_flow.rs) | 8 | 8 |
| 05 | `assessments/lesson_05_ownership_borrowing.rs` | [`lesson_05_ownership_borrowing.rs`](lesson_05_ownership_borrowing.rs) | 9 | 9 |
| 06 | `assessments/lesson_06_structs.rs` | [`lesson_06_structs.rs`](lesson_06_structs.rs) | 14 | 8 |
| 07 | `assessments/lesson_07_enums_pattern_matching.rs` | [`lesson_07_enums_pattern_matching.rs`](lesson_07_enums_pattern_matching.rs) | 9 | 9 |
| 08 | `assessments/lesson_08_collections.rs` | [`lesson_08_collections.rs`](lesson_08_collections.rs) | 9 | 9 |
| 09 | `assessments/lesson_09_packages_modules.rs` | [`lesson_09_packages_modules.rs`](lesson_09_packages_modules.rs) | 7 | 7 |
| 10 | `assessments/lesson_10_error_handling.rs` | [`lesson_10_error_handling.rs`](lesson_10_error_handling.rs) | 11 | 9 |
| 11 | `assessments/lesson_11_generics.rs` | [`lesson_11_generics.rs`](lesson_11_generics.rs) | 10 | 10 |
| 12 | `assessments/lesson_12_traits.rs` | [`lesson_12_traits.rs`](lesson_12_traits.rs) | 13 | 11 |
| 13 | `assessments/lesson_13_lifetimes.rs` | [`lesson_13_lifetimes.rs`](lesson_13_lifetimes.rs) | 13 | 9 |
| 14 | `assessments/lesson_14_closures.rs` | [`lesson_14_closures.rs`](lesson_14_closures.rs) | 9 | 9 |
| 15 | `assessments/lesson_15_iterators.rs` | [`lesson_15_iterators.rs`](lesson_15_iterators.rs) | 10 | 9 |
| 16 | `assessments/lesson_16_smart_pointers.rs` | [`lesson_16_smart_pointers.rs`](lesson_16_smart_pointers.rs) | 11 | 9 |
| 17 | `assessments/lesson_17_threads_channels.rs` | [`lesson_17_threads_channels.rs`](lesson_17_threads_channels.rs) | 9 | 9 |
| 18 | `assessments/lesson_18_macros.rs` | [`lesson_18_macros.rs`](lesson_18_macros.rs) | 16 | 8 |
| **合计** | 18 个文件 | 18 个文件 | **182** | **157** |

参考答案文件是考核文件的**完整副本**：`#[test]`、类型定义、`assess(...)` 参数、注释全部原样保留，
只有练习函数的函数体换成了真实实现。这样对照着看时，题面与答案在同一处，不用来回翻。

> **`cargo test` 永远不会编译这些文件**（根 `Cargo.toml` 里 `autotests = false`，测试目标逐个显式注册），
> 所以写练习时不可能「不小心跑到答案」，测试输出里也不会出现答案内容。

---

## 二、怎么用

### 1. 只想看答案

直接打开上表里的文件，按 `#[test]` 的名字 / `exercise_*` 函数名搜索即可。列出「练习 → 答案位置」：

```powershell
cargo run --bin solution_tool -- --list --lesson 05
```

### 2. 把自己的实现临时换成参考答案跑一遍

```powershell
cargo run --bin solution_tool -- --apply   --lesson 05   # 应用（骨架先备份到 .backup/）
cargo test --test lesson_05_ownership_borrowing          # 跑一遍，看是不是真能全绿
cargo run --bin solution_tool -- --restore --lesson 05   # 还原骨架
```

### 3. 一条命令：应用 → 跑测试 → 自动还原

```powershell
cargo run --bin solution_tool -- --verify --lesson 05
# 全部 18 课一次性验证（157 个知识点应全部通过）
cargo run --bin solution_tool -- --verify
```

### 4. 其它

```powershell
cargo run --bin solution_tool -- --check   # 校验参考答案没动过考核标准（维护者用）
cargo run --bin solution_tool -- --status  # 看每个考核文件是「骨架」还是「已应用」
cargo run --bin solution_tool -- --help
```

---

## 三、保证与约定

- **答案不能改考核标准**：`solution_tool --check` 会逐字节比较「练习函数体之外」的全部内容
  （`#[test]`、`assess(...)` 参数、类型定义、注释、`use`），任何一处不一致都会报错；
  应用时也**只替换练习函数体**，`#[test]` 绝不会被答案覆盖；
- **答案真的可解**：`--verify` 会把参考答案应用进考核文件、跑一遍 libtest、再自动还原，
  只有全部通过才算过。**本次交付前实测：18 个文件、182 个练习、157 个知识点全部通过
  （`157 passed / 0 failed`）**，并且还原后 18 个考核文件与骨架备份**逐字节一致**；
- `.backup/` 里存的是应用前的骨架（已被 `.gitignore` 忽略），所以 `--restore` 任何时候都能把
  考核文件恢复成「到处是 `todo_exercise` 占位」的干净骨架；
- 参考答案同样只用**标准库**，没有 `#[allow(...)]`、没有 `unsafe`、没有第三方 crate，
  并且通过 `rustfmt --edition 2024`。

---

## 四、我写的实现和参考答案不一样，怎么办？

**只要测试全绿，你的写法就是对的**——参考答案只是「一种可行的实现」，不是唯一解。
Rust 里同一件事常有多种惯用写法（`for` 与迭代器链、`match` 与 `if let`、`&str` 与 `String`……），
如果你的实现通过了全部用例（包括边界用例），说明你已经掌握了这个知识点；参考一下答案只是想看看
「别人会怎么写」，而不是「我写错了」。
