# 022 按 trait 规定返回名字

依据：第 21 题自己写完了。方法还只会写在类型自己的 `impl` 里，这一题只加 `trait`。

打开 `src/lib.rs`，只改 `name`。`Named` 和 `Account` 已经写好。测试不要动。

`trait Named` 规定了：任何实现它的类型都要有 `name`，返回 `&str`。`impl Named for Account` 就是为 `Account` 实现这条规定。

名字存在 `String` 里。这里只借出来看，不要把它变成新的 `String`：

```rust
&self.name
```

| `Account` 里的 `name` | `account.name()` |
|------------------------|------------------|
| `"alice"` | `"alice"` |
| `""` | `""` |

做完后在本目录运行 `cargo test`。2 个测试都通过就完成。
