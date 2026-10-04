# 样例报告（samples）

这里放**样板**，方便对照「报告应该长什么样」，以及查看考核用例的验收记录。

| 文件 | 说明 |
| --- | --- |
| `sample_new_learner.md` | **刚上手时**的报告：18 课都是骨架态（157 个知识点全部「未实现」，总分 0 / 待加强），用来认识报告结构与行动清单 |
| `sample_single_lesson.md` | **只完成第 1 课**的报告：该课 8/8 通过（模块得分 100），其余 149 个知识点记为「未运行或编译失败」，总分 5/157——看「没跑的课怎么标注」与「分母口径」 |
| `sample_full_pass.md` | **全部通过**的报告：18 课每个知识点都实现正确（总分 100 · 优秀），含四类知识点掌握率、进度对比与行动清单 |
| `verification.md` | **验收记录**：18 个考核文件在参考实现下「157/157 全部通过」的证据，以及过程中修正过的考核缺陷 |

## 怎么生成自己的报告

```bash
cargo test --no-fail-fast              # 跑考核（写入账本）
cargo run --bin assessment_report      # 生成报告（写到 assessments/report/ 下）
```

或者一步到位用脚本：

```powershell
.\assessments\scripts\run_assessments.ps1              # Windows
./assessments/scripts/run_assessments.sh               # macOS / Linux
```

## 这些样例是怎么来的

三份样例报告现在都由**真实运行**生成，内容不手工编辑：

- `sample_new_learner.md`：在全新数据目录里跑一遍**骨架态**（18 个目标、157 个知识点全部「未实现」）：

  ```powershell
  $env:ASSESSMENT_DATA_DIR = 'target\ledger-sample-new'
  $env:ASSESSMENT_RUN_ID   = 'sample-new-learner'
  cargo test --no-fail-fast
  cargo run --bin assessment_report -- --data-dir target\ledger-sample-new --run sample-new-learner --report-dir <输出目录>
  ```

- `sample_single_lesson.md`：把第 1 课的**参考答案**应用到考核文件后只跑这一课（8/8 通过 → 模块得分 100），
  其余课程本轮没有记录，于是显示为「未运行或编译失败」：

  ```powershell
  cargo run --bin solution_tool -- --apply   --lesson 01
  cargo test --test lesson_01_variables_mutability
  cargo run --bin solution_tool -- --restore --lesson 01
  ```

- `sample_full_pass.md`：`solution_tool --verify` 会对 18 个目标逐个「应用参考答案 → 跑测试 → 自动还原」，
  账本统一记在 `target/solution-verify/`（运行 ID = `solution-verify`），因此**不需要手工合并账本**
  就能整体出报告：

  ```powershell
  cargo run --bin solution_tool -- --verify
  cargo run --bin assessment_report -- --data-dir target/solution-verify --run solution-verify --report-dir <输出目录>
  ```

- `verification.md`：上述验证的结论汇总（参考实现与参考答案两次往返验证、过程中修正的考核缺陷）。

> `assessments/report/data/`（你自己的账本与历史）已在 `.gitignore` 里，不会进版本库；
> 样例报告放在本目录，会进版本库，便于离线阅读与对照。

### 路径与行号的归一化

1. **目录合并**（`src/lessons` + `src/study` → `src/tutorial`）后：三份样例里的**课程路径字段**
   已同步替换为新路径；
2. **「位置」字段对齐骨架**：`sample_single_lesson.md` 与 `sample_full_pass.md` 的账本来自
   「参考答案已应用」的状态，而练习函数体的行数与骨架不同会让后续行号偏移，
   所以生成后按 kp id 把每行的 `位置：` 换算回**骨架文件**里的对应行
   （`sample_new_learner.md` 本来就是骨架态生成的，无需换算）。

> 三份样例里的 `文件:行号` 现在都指向你手上那份**骨架**考核文件，可以直接照着跳转；
> `assessments/solutions/` 下的参考答案则另有一份独立的行号（用
> `cargo run --bin solution_tool -- --list --lesson 05` 查看）。
