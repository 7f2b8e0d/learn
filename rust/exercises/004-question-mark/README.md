# 004 用 ? 把失败交回去

依据：第 3 题已通过。错误处理还差 `?`，这一题只练它。

`parse_count` 已经写好。打开 `src/lib.rs`，只改 `double_count`。

写成：

```rust
let n = parse_count(text)?;
```

失败时，这一行会立刻把 `Err` 返回给调用方。成功时，数字放进 `n`。接着返回这个数字的两倍，包在 `Ok` 里。

| 输入 | 返回 |
|------|------|
| `"12"` | `Ok(24)` |
| `""` | `Err(Fault::Empty)` |
| `"nope"` | `Err(Fault::NotANumber)` |

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
