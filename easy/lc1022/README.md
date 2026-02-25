# LeetCode 1022 - 从根到叶的二进制数之和

**题号**: 1022  
**难度**: Easy  
**链接**: https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/

---

## 题目大意

每个节点是 0 或 1 的二叉树，从根到叶子的路径组成一个二进制数（比如 `1->0->1` 就是 `101` = 5）。求所有根到叶子路径的数字之和。

**示例 1**：
```
    1
   / \
  0   1
 / \
0   1
```
路径: `100`(4) + `101`(5) + `110`(6) + `111`(7) = **22**

**示例 2**：
```
  0
```
输出: **0**

---

## 我的解法

直接用 DFS，遍历每个节点时计算当前路径值。

**关键观察**：
- 每往下一层，之前的路径值要左移一位（×2），再加上当前节点的值
- 比如路径 `1 -> 0 -> 1`：
  - 根节点 1: `0 * 2 + 1 = 1`
  - 节点 0:   `1 * 2 + 0 = 2` (二进制 10)
  - 节点 1:   `2 * 2 + 1 = 5` (二进制 101)

**代码核心**（Rust）：
```rust
fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            let val = (current << 1) | n.val;  // 当前值 = 父值*2 + 当前节点值
            
            if n.left.is_none() && n.right.is_none() {
                return val;  // 叶子节点，返回这条路径的值
            }
            
            dfs(n.left.clone(), val) + dfs(n.right.clone(), val)
        }
    }
}
```

**复杂度**：
- 时间 O(n) - 每个节点访问一次
- 空间 O(h) - 递归栈深度，h 是树高

---

## 踩坑记录

### 坑 1：递推公式想复杂了

一开始想的公式是 `tmp = tmp * 2^depth + cur`，觉得要考虑深度。其实完全没必要，**每往下走一层就乘 2**，不需要管当前是第几层。

正确公式：`val = (val << 1) | cur` 或 `val = val * 2 + cur`

### 坑 2：测试用例构建翻车

右斜树的测试用例，用 `build_tree(vec![Some(1), None, Some(1), None, None, Some(0)])` 构建，结果树的结构不对。因为 `build_tree` 是按层序遍历构建的，空节点不入队，数组里跳过的位置会乱掉。

**解决**：这种特殊结构的手动构建更靠谱：
```rust
let root = Rc::new(RefCell::new(TreeNode::new(1)));
let right1 = Rc::new(RefCell::new(TreeNode::new(1)));
let right2 = Rc::new(RefCell::new(TreeNode::new(0)));

root.borrow_mut().right = Some(right1.clone());
right1.borrow_mut().right = Some(right2);
```

---

## 运行

```bash
cargo test
```

全部 13 个测试通过 ✅
