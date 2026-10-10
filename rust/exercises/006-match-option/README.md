# 006 用 match 看 Some 和 None

依据：第 5 题错误原因是空的，但这题是 AI 代写的，`Some` 和 `None` 还没自己写过。这一题只练用 `match` 区分它们。

打开 `src/lib.rs`，只改 `balance_text`，把 `todo!()` 换成 `match`。测试不要动。

`balance` 只有两种样子：

| 样子 | 应返回 |
|------|--------|
| `Some(12)` | `"balance 12"` |
| `Some(0)` | `"balance 0"` |
| `None` | `"missing"` |

数字用传入的值，不要写死成 12。`Some(0)` 也是有值，和 `None` 不一样。

返回值要是 `String`。`None` 那一支用 `"missing".to_string()`。

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
