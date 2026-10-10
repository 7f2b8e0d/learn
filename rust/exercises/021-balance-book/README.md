# 021 用名字记下一个数字

依据：第 20 题自己写完了。列表现在会加、会筛，这一题只加 `HashMap`。

打开 `src/lib.rs`，只改 `remember`。测试不要动。

`HashMap<String, u32>` 用名字查出一个数字。`insert` 放进一对。名字要放进表里自己拥有，所以用 `to_string()`。同一个名字再记一次，新的数字盖掉旧的。

```rust
book.insert(name.to_string(), amount);
```

查的时候用 `get`。有这个名字是 `Some`，没有是 `None`。

| 操作 | `book.get("alice")` 或 `"bob"` |
|------|--------------------------------|
| `remember(&mut book, "alice", 10)` | `Some(10)` |
| 再 `remember(&mut book, "alice", 3)` | `Some(3)` |
| 没记过 `"bob"` | `None` |

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
