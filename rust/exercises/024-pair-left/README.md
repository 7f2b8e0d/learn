# 024 两个同类型的值

依据：第 23 题自己写完了。`&impl Named` 只写在参数上，这一题只加类型参数。

打开 `src/lib.rs`，只改 `left`。测试不要动。

`Pair<T>` 里的 `left` 和 `right` 类型相同。`T` 是什么由调用方决定，可以是 `u32`，也可以是 `String`。`impl<T>` 表示这些方法对任何 `T` 都适用。

只借出左边的值，不要把它搬走：

```rust
&self.left
```

| `Pair` | `left()` |
|--------|----------|
| `left: 1, right: 2` | `&1` |
| `left: "alice", right: "bob"` | `"alice"` |

做完后在本目录运行 `cargo test`。2 个测试都通过就完成。
