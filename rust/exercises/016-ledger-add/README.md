# 016 把添加放进账本里

依据：第 15 题自己写完了。函数还都是单独的，这一题只加 `impl`。

打开 `src/lib.rs`，只改 `add`。测试不要动。

`add` 写在 `impl Ledger` 里，所以它是 `Ledger` 的方法。`&mut self` 就是这笔账自己的可变借用。往它里面的那组数字末尾放：

```rust
self.fees.push(fee);
```

调用写成 `ledger.add(2)`。`ledger` 要是 `let mut ledger`。

| 调用前的 `fees` | 调用 | 调用后的 `fees` |
|-----------------|------|-----------------|
| `vec![1]` | `ledger.add(2)` | `vec![1, 2]` |
| `vec![]` | `ledger.add(0)` | `vec![0]` |

做完后在本目录运行 `cargo test`。2 个测试都通过就完成。
