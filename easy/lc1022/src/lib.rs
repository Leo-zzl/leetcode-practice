/**
 * LeetCode 1022. Sum of Root To Leaf Binary Numbers
 * 从根到叶的二进制数之和
 * 
 * 题目描述：
 * 给出一棵二叉树，其上每个结点的值都是 0 或 1 。
 * 每一条从根到叶的路径都代表一个从最高有效位开始的二进制数。
 * 例如，如果路径为 0 -> 1 -> 1 -> 0 -> 1，那么它表示二进制数 01101，也就是 13 。
 * 
 * 对树上的每一片叶子，我们都要找出从根到该叶子的路径所表示的数字。
 * 返回这些数字之和。题目数据保证答案是一个 32 位整数。
 * 
 * 示例 1：
 * 输入：root = [1,0,1,0,1,0,1]
 * 输出：22
 * 解释：(100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
 * 
 * 示例 2：
 * 输入：root = [0]
 * 输出：0
 * 
 * 提示：
 * - 树中的节点数在 [1, 1000] 范围内
 * - Node.val 仅为 0 或 1
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }

    /// DFS 遍历二叉树
    /// 
    /// # 参数
    /// - `node`: 当前节点
    /// - `current`: 从根节点到父节点路径表示的二进制数值
    /// 
    /// # 原理
    /// 每深入一层，当前值左移 1 位（乘以 2），然后加上当前节点值
    /// 即: val = (current << 1) | node.val
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                // 计算当前路径值: 父节点值 * 2 + 当前节点值
                let val = (current << 1) | n.val;

                // 如果是叶子节点，返回当前路径值
                if n.left.is_none() && n.right.is_none() {
                    return val;
                }

                // 非叶子节点，继续 DFS 左右子树，累加结果
                Self::dfs(n.left.clone(), val) + Self::dfs(n.right.clone(), val)
            }
        }
    }
}

// ============ 测试用例 ============

#[cfg(test)]
mod tests {
    use super::*;

    /// 辅助函数：从数组构建二叉树
    /// None 表示空节点
    fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = vec![root.clone()];
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let node = queue.remove(0);
            
            // 左子节点
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push(left);
                }
                i += 1;
            }
            
            // 右子节点
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    // ===== 题目提供的示例 =====

    #[test]
    fn test_example_1() {
        // 输入: [1,0,1,0,1,0,1]
        //       1
        //      / \
        //     0   1
        //    / \
        //   0   1
        //  / \
        // 0   1
        // 
        // 叶子节点路径: 100(4), 101(5), 110(6), 111(7)
        // 总和: 4 + 5 + 6 + 7 = 22
        let root = build_tree(vec![
            Some(1),
            Some(0), Some(1),
            Some(0), Some(1), Some(0), Some(1)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 22);
    }

    #[test]
    fn test_example_2() {
        // 输入: [0]
        // 只有一个节点，也是叶子
        let root = build_tree(vec![Some(0)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 0);
    }

    // ===== 边界情况 =====

    #[test]
    fn test_single_node_one() {
        // 只有根节点，值为 1
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 1);
    }

    #[test]
    fn test_left_skewed_tree() {
        // 左斜树: 1 -> 0 -> 1
        // 路径: 101 = 5
        let root = build_tree(vec![Some(1), Some(0), None, Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 5);
    }

    #[test]
    fn test_right_skewed_tree() {
        // 右斜树: 1 -> 1 -> 0
        // 路径: 110 = 6
        // 层序: [1, None, 1, None, None, 0]
        // index 0: 1 (root)
        // index 1: None (left of 1)
        // index 2: 1   (right of 1) 
        // index 3: None (left of right-1, skipped because parent is None)
        // index 4: None (right of right-1)
        // index 5: 0   (left of right-1's right None -> wait, this doesn't work)
        // 
        // 正确的构建方式：手动构建
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let right1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right2 = Rc::new(RefCell::new(TreeNode::new(0)));
        
        root.borrow_mut().right = Some(right1.clone());
        right1.borrow_mut().right = Some(right2);
        
        assert_eq!(Solution::sum_root_to_leaf(Some(root)), 6);
    }

    #[test]
    fn test_perfect_binary_tree_depth_2() {
        // 完美二叉树，深度为 2:
        //       1
        //      / \
        //     0   1
        // 路径: 10(2), 11(3) = 5
        let root = build_tree(vec![Some(1), Some(0), Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 5);
    }

    #[test]
    fn test_all_zeros() {
        // 所有节点都是 0
        //       0
        //      / \
        //     0   0
        // 路径: 00(0), 00(0) = 0
        let root = build_tree(vec![Some(0), Some(0), Some(0)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 0);
    }

    #[test]
    fn test_all_ones() {
        // 所有节点都是 1
        //       1
        //      / \
        //     1   1
        // 路径: 11(3), 11(3) = 6
        let root = build_tree(vec![Some(1), Some(1), Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root), 6);
    }

    // ===== 复杂情况 =====

    #[test]
    fn test_multiple_levels() {
        // 三层树:
        //         1
        //       /   \
        //      0     1
        //     / \   / \
        //    0   1 0   1
        // 路径: 100(4), 101(5), 110(6), 111(7) = 22
        let root = build_tree(vec![
            Some(1),
            Some(0), Some(1),
            Some(0), Some(1), Some(0), Some(1)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 22);
    }

    #[test]
    fn test_unequal_depth() {
        // 不对称树:
        //       1
        //      / \
        //     0   1
        //    /     \
        //   1       0
        // 路径: 101(5), 110(6) = 11
        let root = build_tree(vec![
            Some(1),
            Some(0), Some(1),
            Some(1), None, None, Some(0)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 11);
    }

    #[test]
    fn test_deep_tree() {
        // 深度为 4 的左斜树: 1 -> 0 -> 1 -> 1
        // 路径: 1011 = 11
        let root = build_tree(vec![
            Some(1),
            Some(0), None,
            Some(1), None,
            Some(1)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 11);
    }

    #[test]
    fn test_binary_1000() {
        // 路径 1000 = 8
        //       1
        //      /
        //     0
        //    /
        //   0
        //  /
        // 0
        let root = build_tree(vec![
            Some(1),
            Some(0), None,
            Some(0), None,
            Some(0)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root), 8);
    }

    #[test]
    fn test_large_value() {
        // 1111111111 (10个1) = 1023
        // 构建深度为 10 的右斜树
        let mut values = vec![Some(1)];
        for _ in 0..9 {
            values.push(None); // left
            values.push(Some(1)); // right
        }
        let root = build_tree(values);
        assert_eq!(Solution::sum_root_to_leaf(root), 1023);
    }
}
