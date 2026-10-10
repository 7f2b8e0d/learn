# 025 两个值是否相同

依据：第 24 题只填了方法体。泛型结构还没自己从头写过，这一题只多一个约束 `T: PartialEq`。

打开 `src/lib.rs`，在测试上面自己写结构。测试不要动。

`Pair` 有两个字段，类型相同。写一个方法 `same`，两边相等时返回 `true`，不相等时返回 `false`。数字和字符串都要能比较。

约束写在 `impl` 的尖括号里：

```rust
impl<T: PartialEq> Pair<T>
```

方法里用 `==` 比较两个字段。

英文单词：

| 单词 | 含义 |
|------|------|
| `struct` | 结构体 |
| `impl` | 为类型写方法 |
| `T` | 类型参数 |
| `PartialEq` | 可以比较是否相等 |
| `self` | 这份 `Pair` 自己 |
| `bool` | `true` 或 `false` |
| `left` / `right` | 两个字段名 |
| `same` | 方法名 |

| `left` | `right` | `same()` |
|--------|---------|----------|
| `1` | `1` | `true` |
| `1` | `2` | `false` |
| `"alice"` | `"alice"` | `true` |
| `"alice"` | `"bob"` | `false` |

做完后在本目录运行 `cargo test`。4 个测试都通过就完成。
