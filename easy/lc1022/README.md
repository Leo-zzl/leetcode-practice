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

### 方案 1：DFS（推荐）

递归遍历，自顶向下传路径值。

```rust
fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            let val = (current << 1) | n.val;
            
            if n.left.is_none() && n.right.is_none() {
                return val;  // 叶子节点
            }
            
            dfs(n.left.clone(), val) + dfs(n.right.clone(), val)
        }
    }
}
```

**关键观察**：每往下一层，之前的路径值左移一位（×2），再加上当前节点值。

### 方案 2：BFS

队列存 `(节点, 当前路径值)`，遇到叶子累加。

```rust
queue.push_back((root, 0));
while let Some((node, val)) = queue.pop_front() {
    let new_val = (val << 1) | node.val;
    if is_leaf(node) { sum += new_val; }
    else { push children to queue; }
}
```

### 方案 3：按层贡献 BFS（理论可行，实际不建议）

**思路**：
- 先算每个节点的 `T(n) = Σ(2^叶子深度)`，即该节点下所有叶子的权重和
- 节点贡献 = `val × T(n) / 2^depth`

**需要三次遍历**：
1. 层序遍历记录深度
2. 后序遍历计算 T(n)
3. 累加各节点贡献

**性能对比**（1000次运行）：

| 树类型 | DFS | BFS | 按层贡献 BFS | 慢多少倍 |
|--------|-----|-----|--------------|---------|
| 完美二叉树 (2048节点) | 33ms | 51ms | **2950ms** | **57x** |
| 左斜树 (50节点) | 0.9ms | 1.3ms | **59.7ms** | **47x** |

**结论**：按层贡献方案比标准 BFS 慢 **50 倍左右**，因为需要 3 次遍历 + HashMap 开销，而且容易溢出（`2^depth` 增长极快）。

**推荐用 DFS 或标准 BFS**。

---

## 复杂度

- 时间：O(n) - 每个节点访问一次
- 空间：O(h) DFS 递归栈，或 O(w) BFS 队列（w 是树的最大宽度）

---

## 踩坑记录

### 坑 1：递推公式想复杂了

一开始想的公式是 `tmp = tmp * 2^depth + cur`，其实完全没必要，**每往下走一层就乘 2** 就行。

正确公式：`val = (val << 1) | cur` 或 `val = val * 2 + cur`

### 坑 2：测试用例构建翻车

`build_tree` 按层序遍历构建，空节点不入队，数组里跳过的位置会乱掉。

**解决**：特殊结构的手动构建更靠谱：
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

全部测试通过 ✅
