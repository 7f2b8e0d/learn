# 020 只数大于 0 的

依据：第 19 题自己写完了。`map` 会改每个数字，这一题只加 `filter`。

打开 `src/lib.rs`，只改 `paid_count`。`add` 已经写好。测试不要动。

`filter` 只留下闭包返回 `true` 的那些，再 `count()` 数有几个。`0` 留下的是 `false`，不计数。

`filter` 的 `|fee|` 比 `map` 多一层引用，所以 `fee` 是 `&&u32`。比较要拿到数字本身，写两个 `*`：

```rust
self.fees.iter().filter(|fee| **fee > 0).count()
```

| 过程 | `paid_count` |
|------|----------------|
| `add(0)`、`add(5)`、`add(0)`、`add(2)` | `2` |
| 只 `add(0)` | `0` |
| 空账本 | `0` |

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
