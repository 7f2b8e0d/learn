# 005 用 Option 表示有没有

依据：第 4 题已通过。这一题只多一件事：`Option` 表示有值或没有值。

`lookup` 已经写好。打开 `src/lib.rs`，只改 `doubled`。

`lookup` 找到了是 `Some(数字)`，没找到是 `None`。写成：

```rust
let n = lookup(name)?;
```

没有时，这一行会立刻把 `None` 交回去。有值时，数字放进 `n`。接着返回这个数字的两倍，包在 `Some` 里。

| 输入 | 返回 |
|------|------|
| `"alice"` | `Some(24)` |
| `"bob"` | `Some(0)` |
| `"carol"` | `None` |

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
