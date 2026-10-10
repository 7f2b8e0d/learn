# 023 同一个函数接收两种类型

依据：第 22 题自己写完了。`trait` 还只会在类型自己身上调用，这一题只加 `&impl Named`。

打开 `src/lib.rs`，只改 `label`。`Account` 和 `Contract` 都已经实现了 `Named`。测试不要动。

`&impl Named` 表示：传进来的是某个实现了 `Named` 的值的借用。函数里不用知道它到底是 `Account` 还是 `Contract`，只要调用 `name`：

```rust
item.name()
```

| 传入 | `label` |
|------|---------|
| 名字是 `"alice"` 的 `Account` | `"alice"` |
| 名字是 `"vault"` 的 `Contract` | `"vault"` |
| 名字是 `""` 的 `Account` | `""` |

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
