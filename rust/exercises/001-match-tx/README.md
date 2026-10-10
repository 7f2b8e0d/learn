# 001 用 match 把通知变成一句话

这题不用懂区块链。只练 `match`：看一个值是哪一种，返回对应的句子。

打开 `src/lib.rs`，只改 `text_of`，把 `todo!()` 换成 `match`。上面的 `Notice` 和下面的测试都不要动。

`Notice` 只有三种样子：

| 样子 | 意思 | 应返回 |
|------|------|--------|
| `Ready { count: 2 }` | 准备好了，数量是 2 | `"ready 2"` |
| `Failed("timeout")` | 失败了，原因是 timeout | `"failed: timeout"` |
| `Waiting` | 还在等 | `"waiting"` |

数量和原因都用传入的值，不要写死成 2 和 timeout。

做完后在本目录运行 `cargo test`。3 个测试都通过就完成。
