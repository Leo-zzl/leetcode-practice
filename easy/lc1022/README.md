# LeetCode 1022 - 从根到叶的二进制数之和

## 题目信息

| 项目 | 内容 |
|------|------|
| 题号 | 1022 |
| 标题 | Sum of Root To Leaf Binary Numbers |
| 中文 | 从根到叶的二进制数之和 |
| 难度 | Easy |
| 链接 | https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/ |

## 题目描述

给出一棵二叉树，其上每个结点的值都是 `0` 或 `1` 。

每一条从根到叶的路径都代表一个从最高有效位开始的二进制数。

- 例如，如果路径为 `0 -> 1 -> 1 -> 0 -> 1`，那么它表示二进制数 `01101`，也就是 `13`。

对树上的每一片叶子，我们都要找出从根到该叶子的路径所表示的数字。

返回这些数字之和。题目数据保证答案是一个 **32 位** 整数。

## 示例

### 示例 1

```
输入：root = [1,0,1,0,1,0,1]
      1
     / \
    0   1
   / \
  0   1
输出：22
解释：(100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
```

### 示例 2

```
输入：root = [0]
输出：0
```

## 提示

- 树中的节点数在 `[1, 1000]` 范围内
- `Node.val` 仅为 `0` 或 `1`

## 解题思路

### 核心思想

使用 **DFS (深度优先搜索)** 遍历二叉树：

1. 从根节点开始，维护当前路径表示的数值
2. 到达叶子节点时，将当前数值加入总和
3. 利用位运算优化：每深入一层，当前值左移 1 位再加上节点值

### 位运算技巧

```
当前值 = (当前值 << 1) | 节点值

例如：路径 1 -> 0 -> 1
      1: (0 << 1) | 1 = 1
      0: (1 << 1) | 0 = 2  (二进制 10)
      1: (2 << 1) | 1 = 5  (二进制 101)
```

### 复杂度分析

| 项目 | 复杂度 | 说明 |
|------|--------|------|
| 时间 | O(n) | 遍历每个节点一次 |
| 空间 | O(h) | 递归栈深度，h 为树高度 |

## 文件结构

```
lc1022/
├── Cargo.toml      # Rust 项目配置
├── src/
│   └── lib.rs      # Solution 实现和测试
└── README.md       # 本文件
```

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_example_1

# 显示测试输出
cargo test -- --nocapture
```

## 核心代码模板

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }
    
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                let val = (current << 1) | n.val;
                
                // 如果是叶子节点
                if n.left.is_none() && n.right.is_none() {
                    return val;
                }
                
                // 继续遍历左右子树
                Self::dfs(n.left.clone(), val) + 
                Self::dfs(n.right.clone(), val)
            }
        }
    }
}
```

---

Happy Coding! 🦀
