# 01 · 测试环境配置指南（零基础新人版）

本文件面向**完全没装过 Rust 的新人**：从下载安装到把考核环境跑通，全部照着敲即可。
目标读者不需要任何前置知识，本仓库的考核体系**不需要联网、不需要第三方 crate**。

> 本文件的安装与验证说法与根 [README.md](../../README.md) 第二节「环境搭建指南」、
> 第六节「如何运行」保持一致；课程目录请对照
> [src/tutorial/README.md](../../src/tutorial/README.md)。

**全部命令都在项目根目录（含 `Cargo.toml` 的那一层）执行。**

---

## 一、先看结论：考核环境最快验证路径

已经装好 Rust 的同学，只做下面三步（几分钟内）就能确认考核环境可用：

| 步骤 | 命令 | 成功标志 |
| --- | --- | --- |
| 1 | `cargo test --test lesson_01_variables_mutability` | 输出 `0 passed; 8 failed`（**8 个失败是正常的**，说明环境通了） |
| 2 | `cargo run --bin assessment_report` | 命令执行成功（退出码 0），报告体系认得你的这次运行 |
| 3 | `cargo test` | 18 个考核目标全部被编译并执行 |

跑完第 1 步看到「8 个 FAILED」就说明**环境已经好了**，可以去读
[02_how_to_answer.md](02_how_to_answer.md) 开始做题。

---

## 二、安装 rustup（Rust 官方工具链管理器）

### 2.1 Windows

1. 打开 <https://www.rust-lang.org/tools/install>，下载 **`rustup-init.exe`**
   （也可以直接下载 <https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe>）；
2. **双击运行**（用普通权限即可，不需要管理员权限）；
3. 安装程序会先检查 C/C++ 链接器：如果本机没有 Visual Studio / C++ 生成工具，
   它会**询问是否自动安装这些前置组件**——选同意即可，它装的是 Visual Studio Community
   版及其「使用 C++ 的桌面开发」组件；
4. 前置组件就绪后，安装程序会给出选项菜单，选择默认项
   `1) Proceed with standard installation (default - just press enter)`，直接回车；
5. 看到 `Rust is installed now. Great!` 即安装完成。

> 本仓库的开发机使用的是 **GNU 工具链**（`rustc -vV` 里 `host: x86_64-pc-windows-gnu`），
> 链接器是 MinGW 的 `ld`，同样可以完整跑通本项目（详见第七节第 2 条的链接器噪声）。

### 2.2 macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

按提示回车选择默认安装；安装完成后**重开终端**，或者执行：

```bash
source "$HOME/.cargo/env"
```

### 2.3 关键一步：重开终端

安装脚本只修改了**新终端**的环境变量。请关掉当前终端窗口，重新打开一个，再执行：

```powershell
rustc --version
cargo --version
rustup --version
```

期望输出（版本号会随时间变化，1.99 是本仓库的验证版本）：

```text
rustc 1.99.0 (b940084d7 2026-09-28)
cargo 1.99.0 (5f94df478 2026-08-27)
rustup 1.29.1 (d95a37b6a 2026-08-13)
```

> 注意：`rustup --version` 会顺带在 stderr 打印一段 `info: This is the version for the
> rustup toolchain manager, not the rustc compiler.`，那是**正常提示**，不是错误。

### 2.4 本项目要求的版本

- 工具链：**stable**（本项目不写任何 nightly / 实验性特性）；
- 版本：**edition 2024**，见根 `Cargo.toml` 的 `edition = "2024"`。
  使用 2024 edition 需要较新的 stable 工具链；本仓库在 **rustc 1.99.0** 上验证通过。
  如果编译时报 `feature ... is not stable` 或 `edition2024 is unstable`，说明工具链太旧，
  执行下面这条升级即可：

```bash
rustup update stable
rustup default stable
```

---

## 三、可选组件：格式化 / 静态检查 / IDE 补全

这三个组件都不是跑考核的必需品，但强烈建议装：

```bash
rustup component add rustfmt        # 格式化：cargo fmt / rustfmt
rustup component add clippy         # 静态检查：cargo clippy
rustup component add rust-analyzer  # IDE 补全、跳转、内联类型提示
```

验证：

```powershell
rustfmt --version
cargo clippy --version
```

期望输出（示例）：

```text
rustfmt 1.10.0-stable (b940084d7e 2026-09-28)
clippy 0.1.99 (b940084d7e 2026-09-28)
```

用 `rustup component list --installed` 可以查看当前工具链已装的所有组件。
常用命令速查：

| 命令 | 作用 |
| --- | --- |
| `cargo fmt` | 按 rustfmt 默认风格格式化整个项目 |
| `rustfmt --edition 2024 --check assessments/lesson_01_variables_mutability.rs` | 只检查单个考核文件是否合规 |
| `cargo clippy --bins` | 对 18 个课程二进制做静态检查（学习惯用法） |
| `cargo check` | 只做类型检查，速度最快 |

> 提示：`cargo clippy --lib` 检查的是考核框架 `assessment_harness` 本身，它可能报一条
> `clippy::missing_const_for_thread_local`（`assessments/harness/lib.rs` 里的 `thread_local!`）。
> 那个初始化**已经是** `const { ... }`，属于 clippy 的已知误报。
> 这些**不是你的问题**，考核只看学习者的练习函数，不要为此去改框架文件。

---

## 四、编辑器建议

| 编辑器 | 做法 | 说明 |
| --- | --- | --- |
| **VS Code**（推荐，免费） | 安装扩展 **rust-analyzer**（作者 rust-lang） | 装完直接打开项目根目录即可补全、跳转、看类型 |
| **CLion / IntelliJ IDEA**（付费） | 安装 **Rust** 插件（JetBrains 官方） | 与本仓库自带的 `.idea/` 目录配合最好 |
| 其它 | 任何能编辑 UTF-8 文本的编辑器 | 考核只依赖命令行，编辑器不影响结果 |

> **关于本仓库的 `.idea/` 目录**：那是 JetBrains 系列 IDE 的工程配置（`.idea/hello.iml`、
> `.idea/modules.xml` 等），**不影响命令行运行**，也不需要你去维护它。
> 用 VS Code 或纯命令行完全可以完成全部考核。

---

## 五、考核环境的三个验证步骤（每条都给确切命令与期望输出）

### 第 1 步：单课验证 —— 看到 8 个 FAILED 才算通过

```powershell
cargo test --test lesson_01_variables_mutability
```

**期望输出（骨架态的真实输出，已在本机核对）：**

```text
running 8 tests
test kp_01_02_shadowing_chain ... FAILED
test kp_01_01_mut_counter ... FAILED
test kp_01_05_inference_and_annotation ... FAILED
...
test result: FAILED. 0 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test lesson_01_variables_mutability`
```

> 说明：8 行 `test kp_01_XX_... ... FAILED` 的**打印顺序与上面不同是正常的**——
> libtest 默认多线程并发跑测试，输出顺序不固定（上面就是本机的真实顺序）。
> 关键看最后一行：`0 passed; 8 failed`。

**为什么 8 个 FAILED 是正常的？**
因为这 8 个练习函数里都还留着 `todo_exercise(...)` 占位，考核框架会把它们记为
「未实现」并让 libtest 判失败。**失败的是「你还没写代码」，不是「你的环境有问题」。**
反过来，如果这一步**编译失败**、或者找不到测试目标，才是真正的环境问题。

正常/异常对照：

| 现象 | 判断 |
| --- | --- |
| `0 passed; 8 failed` | ✅ 环境正确，可以开始做题 |
| `0 passed; 0 failed; ... 0 filtered out` 且没有 `running 8 tests` | ❌ 测试目标没选中，确认在项目根目录、且 `--test` 名字拼写正确 |
| `error: no test target named ...` | ❌ 名字拼错（正确名字见 `Cargo.toml` 的 `[[test]] name`），或不在项目根目录 |
| `error[E...]` 编译错误 | ❌ 环境或工具链有问题，见第七节排查 |

> 这个命令只编译**考核框架库** `assessment_harness` 和你指定的那 1 个测试目标，
> 不会去编译 18 个课程二进制，所以速度很快。

### 第 2 步：生成学习评估报告

```powershell
cargo run --bin assessment_report
```

**它做什么**：读取账本 `assessments/report/data/ledger.tsv`（第 1 步的每一次运行都会被追加进去），
汇总每个模块、每个知识点的掌握情况，输出人读的 Markdown 报告。

**期望产物**（跑完后出现在 `assessments/report/` 下）：

| 文件 | 内容 |
| --- | --- |
| `assessments/report/assessment_report.md` | 学习评估报告：薄弱环节 + 行动清单 |
| `assessments/report/summary.json` | 机器可读摘要 |
| `assessments/report/data/history.jsonl` | 每次运行一行，用于「学习进度对比」 |
| `assessments/docs/04_knowledge_map.md` | 知识点覆盖矩阵（**生成物**）：加 `--emit-map` 参数时由 `cargo run --bin assessment_report -- --emit-map` 产出 |

> `assessment_report` 是独立的 `[[bin]]` 目标，可以用
> `cargo build --bin assessment_report` 单独编译、用
> `cargo run --bin assessment_report -- --emit-map` 额外生成知识矩阵
> （`--emit-map` 是报告生成器的参数，写在 `--` 之后）；
> `docs/04_knowledge_map.md` 属于**生成物**——由 `--emit-map` 产出，
> 不要手写（增删知识点请改考核文件与框架登记表，见
> [05_maintainer_guide.md](05_maintainer_guide.md)）。
>
> 数据目录可以用环境变量 `ASSESSMENT_DATA_DIR` 改到别处（做实验时避免污染自己的进度记录）：
>
> ```powershell
> $env:ASSESSMENT_DATA_DIR = "$env:TEMP\assess-selfcheck"
> ```

### 第 3 步：一次运行全部 18 门课

```powershell
cargo test
```

**它会做什么**：编译并执行 `Cargo.toml` 里注册的 **18 个 `[[test]]` 目标**
（`lesson_01_variables_mutability` ~ `lesson_18_macros`），一次把 18 门课的考核跑完。

**会比较慢**：首次运行需要编译 18 个测试目标 + 考核框架库，请耐心等待（通常几分钟；
之后有缓存就快得多——本机在已缓存的情况下全量运行约 7.5 秒）。慢的原因是编译，
不是你的代码有问题。

**建议加一个参数**：默认情况下第一个失败的测试目标会让 cargo 停止后续目标。
想一次看完全部 18 课的结果，用：

```powershell
cargo test --no-fail-fast
```

**结果会自动合并成一次考核**：账本按「10 分钟会话窗口」自动归并，
所以直接连续敲 `cargo test`（不经过任何脚本）也能得到一份完整的评估报告。

---

## 六、完全离线可用

**考核体系零第三方依赖**，不需要联网下载任何 crate：

- 根 `Cargo.toml` 的 `[dependencies]` 段是**空的**；
- 18 个考核文件只 `use assessment_harness`（框架库，本仓库自带）
  与标准库，不引用任何外部 crate；
- 考核框架 `assessments/harness/lib.rs` 也只使用标准库；
- 报告生成器同样只用标准库。

因此「下载 rustup 工具链」之后，`cargo test` / `cargo run --bin assessment_report`
/ `cargo check` 都可以**完全离线**执行。

> ⚠️ 例外：`src/demoweb/` 是**独立的 Cargo 项目**（Axum + Tokio + SQLx 等），
> 它**需要联网**下载第三方 crate（首次 `cargo run` 或 `cargo fetch` 会拉取依赖，
> 国内网络建议配置镜像，详见 [src/demoweb/README.md](../../src/demoweb/README.md)）。
> 考核环境与 demoweb 无关：只跑考核，不需要网络。

---

## 七、常见环境问题排查

### 1）PowerShell 执行策略导致 `.ps1` 脚本无法直接运行

- **现象**：运行仓库里的脚本时报
  `无法加载文件 ...\xxx.ps1，因为在此系统上禁止运行脚本`，
  或 `File ... cannot be loaded because running scripts is disabled on this system`。
- **原因**：本机 PowerShell 的 ExecutionPolicy 限制（默认常见为 `Restricted`），
  与脚本内容和 Rust 环境都无关。
- **解决**：不修改执行策略，直接把脚本内容读出来变成脚本块执行。
  这与根 [README.md](../../README.md) 第六节「一键校验全部课程」的写法完全一致：

  ```powershell
  $code = [System.IO.File]::ReadAllText("$PWD\.dsh\check_expected_output.ps1", [System.Text.Encoding]::UTF8)
  & ([scriptblock]::Create($code))
  ```

  > `ReadAllText` 的第二个参数显式指定 **UTF8**，避免脚本里的中文注释被按本地代码页解码而乱码。

### 2）`warning: linker stderr: corrupt .drectve at end of def file`

- **现象**：`cargo build` / `cargo test` 时反复出现
  `warning: linker stderr: corrupt .drectve at end of def file`
  （有时一次测试会打印好几条），但编译继续、可执行文件正常生成。
- **原因**：这是 **Windows + MinGW（`x86_64-pc-windows-gnu`）工具链**下，
  MinGW 链接器 `ld` 对 cargo 传入的 `.drectve` 段给出的**噪声提示**，
  不是你的代码问题，也不是项目配置问题（连根目录原有的 `src/main.rs` 也会报同一条）。
- **解决**：**忽略即可**，不影响考核结果——用 `rustc` 直接编译零诊断，
  `cargo check --bins` 同样零诊断，程序照常运行、退出码为 0。
  如果一定要消除这条噪声，可以切换到 MSVC 工具链：

  ```bash
  rustup toolchain install stable-x86_64-pc-windows-msvc
  rustup default stable-x86_64-pc-windows-msvc
  ```

### 3）中文显示乱码

- **现象**：终端里中文变成 `���` 或方块；`cargo test` 的失败信息（含中文提示）看不清。
- **原因**：**终端代码页**不是 UTF-8，与文件编码无关——
  本仓库所有 `.md` / `.rs` 文件本身就是 **UTF-8** 编码，内容没有损坏。
- **解决**：
  1. 临时切换到 UTF-8 代码页（PowerShell / cmd 均可）：

     ```powershell
     chcp 65001
     ```

  2. 更省事的做法：改用 **Windows Terminal**（默认 UTF-8），
     或在 VS Code 的集成终端里执行命令；
  3. 用 `Get-Content` 看文档时显式指定编码：

     ```powershell
     Get-Content assessments\docs\02_how_to_answer.md -Encoding UTF8
     ```

### 4）`cargo: command not found` / 版本过旧

- **现象 A**：无法识别 `cargo` 命令。英文系统上是
  `'cargo' is not recognized as an internal or external command` /
  `cargo: command not found`；中文 PowerShell 上是
  `无法将“cargo”项识别为 cmdlet、函数、脚本文件或可运行程序的名称`。
  - **原因**：Cargo 不在当前终端的 PATH 里——最常见的原因是**安装后没有重开终端**。
  - **解决**：关掉终端重新打开；仍不行则确认 `%USERPROFILE%\.cargo\bin`（Windows）
    或 `$HOME/.cargo/bin`（macOS/Linux）已加入 PATH，必要时手动执行
    `source "$HOME/.cargo/env"`。
- **现象 B**：命令能跑但报版本/edition 相关错误，例如
  `error: edition 2024 is unstable`、`feature ... is not stable`。
  - **原因**：工具链太旧（本项目要求 stable + edition 2024）。
  - **解决**：

    ```bash
    rustup update stable
    rustup default stable
    rustc --version
    ```

### 5）磁盘与首次编译时间

- **现象**：第一次 `cargo test` 很慢，并且仓库根目录突然多出一个很大的 `target/` 目录。
- **原因**：Cargo 把全部中间产物与可执行文件放在 `target/` 下；
  `cargo test` 会同时编译考核框架 + 18 个考核目标，产物较多，
  `target/` 常见占用在**几百 MB 到 1 GB 级别**。
- **解决**：
  - 耐心等第一次编译完成，之后有缓存很快；
  - 只在真的需要时清理，因为清理**代价很大**（下次要重新全量编译）：

    ```bash
    cargo clean          # 删除整个 target/（下次全量重编译）
    ```

  - `target/` 已被 `.gitignore` 忽略，不会进入版本控制；
  - 想分步观察编译情况可以先只编译不运行：

    ```powershell
    cargo test --test lesson_01_variables_mutability --no-run
    ```

---

## 八、环境自检清单

逐条执行，全部打勾即可开始做题：

| # | 命令 | 期望结果 | 不通过怎么办 |
| --- | --- | --- | --- |
| 1 | `rustc --version` | 打印 `rustc 1.x.y (...)` | 未安装或没重开终端：重装 rustup / 重开终端（第七节第 4 条） |
| 2 | `cargo --version` | 打印 `cargo 1.x.y (...)` | 同上，检查 PATH 是否含 `.cargo\bin` |
| 3 | `rustup --version` | 打印 `rustup 1.x.y`（stderr 的 info 提示正常） | 未装 rustup：按第二节重新安装 |
| 4 | `rustup component list --installed` | 至少含 `rustfmt`、`clippy` | `rustup component add rustfmt` / `add clippy` |
| 5 | `cargo test --test lesson_01_variables_mutability` | `running 8 tests` → `0 passed; 8 failed` | 编译错误看第七节；找不到目标检查是否在项目根目录、名字是否拼错 |
| 6 | `cargo run --bin assessment_report` | 退出码 0，`assessments/report/` 下出现报告产物 | 报 `no bin target` → 不在项目根目录；报编译错误 → 第七节 |
| 7 | `cargo test --no-fail-fast` | 18 个考核目标逐个 `Running assessments\lesson_XX_....rs` | 同上；只是慢不属于失败 |
| 8 | `rustfmt --edition 2024 --check assessments/lesson_01_variables_mutability.rs` | 无输出、退出码 0（合规） | 未装 rustfmt → 第 4 条；有 diff → 说明你改动了格式 |

> 第 5 条的「8 个失败」再强调一次：**这是环境的正确状态**，不是异常。

---

## 九、下一步

环境搭好后，请先读 **[02_how_to_answer.md](02_how_to_answer.md)**（答题指南：怎么练、怎么写、怎么自测），
再去 `assessments/` 下挑第一课开始做：

```powershell
cargo test --test lesson_01_variables_mutability
```

想了解整个考核体系的结构与考核框架 API，见
[05_maintainer_guide.md](05_maintainer_guide.md)；
课程目录与学习流程见 [src/tutorial/README.md](../../src/tutorial/README.md) 与根 [README.md](../../README.md)。
