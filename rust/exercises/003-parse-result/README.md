# 003 用 Result 表示成不成功

依据：第 2 题已通过。这题只多一件事：`Result` 用来表示成功或失败。

打开 `src/lib.rs`，只改 `parse_count`。

`text.parse::<u32>()` 会得到一个 `Result`。成功是 `Ok(数字)`，失败是 `Err(...)`。用 `match` 看这个结果。

| 输入 | 返回 |
|------|------|
| `""` | `Err(Fault::Empty)` |
| `"12"` | `Ok(12)` |
| `"nope"` | `Err(Fault::NotANumber)` |

空字符串不要交给 `parse`，直接返回 `Empty`。其他失败返回 `NotANumber`。

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
