# 026 只能判断相等

依据：第 25 题把相等约束写成了 `PartialOrd`。`==` 要的是 `PartialEq`。这一题只练这个约束，不再写结构体。

打开 `src/lib.rs`，在测试上面自己写 `same`。测试不要动。

`same` 接收两个同类型的借用，相等返回 `true`，不相等返回 `false`。函数签名写成：

```rust
fn same<T: PartialEq>(left: &T, right: &T) -> bool
```

`Label` 只实现了 `PartialEq`，不能比较大小。约束写成 `PartialOrd` 时，`same(&Label("alice"), &Label("bob"))` 会编不过。

方法体用 `==` 比较 `left` 和 `right`。

英文单词：

| 单词 | 含义 |
|------|------|
| `PartialEq` | 可以判断是否相等，给 `==` 用 |
| `PartialOrd` | 可以比较大小，给 `<`、`>` 用 |
| `fn` | 函数 |
| `bool` | `true` 或 `false` |
| `Label` | 测试里的类型，只能相等比较 |

| 调用 | 应返回 |
|------|--------|
| `same(&1, &1)` | `true` |
| `same(&1, &2)` | `false` |
| `same(&Label("alice"), &Label("alice"))` | `true` |
| `same(&Label("alice"), &Label("bob"))` | `false` |

做完后在本目录运行 `cargo test`。4 个测试都通过就完成。
