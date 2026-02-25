/**
 * LeetCode 1022. Sum of Root To Leaf Binary Numbers
 * 从根到叶的二进制数之和
 */

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};

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

pub struct Solution;

impl Solution {
    // ========== 方案 1: 标准 DFS (自顶向下) ==========
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                let val = (current << 1) | n.val;

                if n.left.is_none() && n.right.is_none() {
                    return val;
                }

                Self::dfs(n.left.clone(), val) + Self::dfs(n.right.clone(), val)
            }
        }
    }

    // ========== 方案 2: 标准 BFS (自顶向下) ==========
    /// 队列存 (节点, 当前路径值)
    /// 遇到叶子累加，非叶子将子节点入队
    pub fn sum_root_to_leaf_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut queue = VecDeque::new();

        if let Some(r) = root {
            queue.push_back((r, 0));
        }

        while let Some((node, val)) = queue.pop_front() {
            let n = node.borrow();
            let new_val = (val << 1) | n.val;

            if n.left.is_none() && n.right.is_none() {
                sum += new_val;
                continue;
            }

            if let Some(left) = n.left.clone() {
                queue.push_back((left, new_val));
            }
            if let Some(right) = n.right.clone() {
                queue.push_back((right, new_val));
            }
        }

        sum
    }

    // ========== 方案 3: 按层贡献 BFS (两次遍历) ==========
    /// 先算每个节点的 T(n) = Σ(2^叶子深度)，再算贡献
    /// 节点贡献 = val × T(n) / 2^depth
    pub fn sum_root_to_leaf_layer_contrib(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        // 第一次 BFS：收集遍历顺序，记录深度
        let mut order = Vec::new();
        let mut depth_map: HashMap<*const TreeNode, i32> = HashMap::new();
        let mut queue = VecDeque::new();

        if let Some(r) = root.clone() {
            queue.push_back((r, 0));
        }

        while let Some((node, depth)) = queue.pop_front() {
            let ptr: *const TreeNode = node.as_ptr();
            let n = node.borrow();
            
            order.push(node.clone());
            depth_map.insert(ptr, depth);

            if let Some(left) = n.left.clone() {
                queue.push_back((left, depth + 1));
            }
            if let Some(right) = n.right.clone() {
                queue.push_back((right, depth + 1));
            }
        }

        // 第二次：后序遍历（逆序）计算 T(n)
        // T(n) = 叶子: 2^depth, 内部: T(左) + T(右)
        let mut t_map: HashMap<*const TreeNode, i64> = HashMap::new();
        
        for node in order.iter().rev() {
            let ptr: *const TreeNode = node.as_ptr();
            let n = node.borrow();
            let depth = *depth_map.get(&ptr).unwrap();

            if n.left.is_none() && n.right.is_none() {
                // 叶子节点
                t_map.insert(ptr, (1i64) << depth);
            } else {
                // 内部节点
                let left_t = n.left.as_ref()
                    .map(|l| *t_map.get(&(l.as_ptr() as *const TreeNode)).unwrap_or(&0))
                    .unwrap_or(0);
                let right_t = n.right.as_ref()
                    .map(|r| *t_map.get(&(r.as_ptr() as *const TreeNode)).unwrap_or(&0))
                    .unwrap_or(0);
                t_map.insert(ptr, left_t + right_t);
            }
        }

        // 第三次：计算贡献和
        // 贡献 = val × T(n) / 2^depth
        let mut sum = 0i64;
        for node in &order {
            let ptr: *const TreeNode = node.as_ptr();
            let n = node.borrow();
            let depth = *depth_map.get(&ptr).unwrap();
            let t = *t_map.get(&ptr).unwrap();
            
            // T(n) 一定能被 2^depth 整除
            let contrib = (n.val as i64) * (t >> depth);
            sum += contrib;
        }

        sum as i32
    }
}

// ============ 测试用例 ============

#[cfg(test)]
mod tests {
    use super::*;

    /// 辅助函数：从数组构建二叉树
    fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = vec![root.clone()];
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let node = queue.remove(0);
            
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push(left);
                }
                i += 1;
            }
            
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

    // ===== 正确性测试 =====

    #[test]
    fn test_example_1() {
        let root = build_tree(vec![
            Some(1),
            Some(0), Some(1),
            Some(0), Some(1), Some(0), Some(1)
        ]);
        assert_eq!(Solution::sum_root_to_leaf(root.clone()), 22);
        assert_eq!(Solution::sum_root_to_leaf_bfs(root.clone()), 22);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(root), 22);
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(vec![Some(0)]);
        assert_eq!(Solution::sum_root_to_leaf(root.clone()), 0);
        assert_eq!(Solution::sum_root_to_leaf_bfs(root.clone()), 0);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(root), 0);
    }

    #[test]
    fn test_single_node_one() {
        let root = build_tree(vec![Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root.clone()), 1);
        assert_eq!(Solution::sum_root_to_leaf_bfs(root.clone()), 1);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(root), 1);
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = build_tree(vec![Some(1), Some(0), None, Some(1)]);
        assert_eq!(Solution::sum_root_to_leaf(root.clone()), 5);
        assert_eq!(Solution::sum_root_to_leaf_bfs(root.clone()), 5);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(root), 5);
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let right1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right2 = Rc::new(RefCell::new(TreeNode::new(0)));
        
        root.borrow_mut().right = Some(right1.clone());
        right1.borrow_mut().right = Some(right2);
        
        assert_eq!(Solution::sum_root_to_leaf(Some(root.clone())), 6);
        assert_eq!(Solution::sum_root_to_leaf_bfs(Some(root.clone())), 6);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(Some(root)), 6);
    }

    #[test]
    fn test_unequal_depth() {
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
        assert_eq!(Solution::sum_root_to_leaf(root.clone()), 11);
        assert_eq!(Solution::sum_root_to_leaf_bfs(root.clone()), 11);
        assert_eq!(Solution::sum_root_to_leaf_layer_contrib(root), 11);
    }

    // ===== 性能对比测试 =====

    /// 构建完美二叉树
    fn build_perfect_tree(depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth < 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), 0));
        
        while let Some((node, d)) = queue.pop_front() {
            if d >= depth {
                continue;
            }
            let left = Rc::new(RefCell::new(TreeNode::new(1)));
            let right = Rc::new(RefCell::new(TreeNode::new(1)));
            node.borrow_mut().left = Some(left.clone());
            node.borrow_mut().right = Some(right.clone());
            queue.push_back((left, d + 1));
            queue.push_back((right, d + 1));
        }
        Some(root)
    }

    /// 构建左斜树
    fn build_left_skewed_tree(n: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if n > 60 { panic!("n too large, will overflow"); }
        if n <= 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let mut current = root.clone();
        for _ in 1..n {
            let left = Rc::new(RefCell::new(TreeNode::new(1)));
            current.borrow_mut().left = Some(left.clone());
            current = left;
        }
        Some(root)
    }

    use std::time::Instant;

    #[test]
    fn benchmark_perfect_tree_depth_10() {
        // 完美二叉树，深度10，约 2048 个节点，1024 个叶子
        let root = build_perfect_tree(10);
        
        // 预热
        let _ = Solution::sum_root_to_leaf(root.clone());
        let _ = Solution::sum_root_to_leaf_bfs(root.clone());
        let _ = Solution::sum_root_to_leaf_layer_contrib(root.clone());

        let iterations = 1000;

        // DFS
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf(root.clone());
        }
        let dfs_time = start.elapsed();

        // BFS
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf_bfs(root.clone());
        }
        let bfs_time = start.elapsed();

        // 按层贡献
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf_layer_contrib(root.clone());
        }
        let layer_time = start.elapsed();

        println!("\n=== 完美二叉树 (深度10, ~2048节点) ===");
        println!("DFS (自顶向下):  {:?}", dfs_time);
        println!("BFS (自顶向下):  {:?}", bfs_time);
        println!("按层贡献 BFS:    {:?}", layer_time);
        println!("Ratio (Layer/BFS): {:.2}x", layer_time.as_secs_f64() / bfs_time.as_secs_f64());
    }

    #[test]
    fn benchmark_left_skewed_tree_50() {
        // 左斜树，50 个节点，1 个叶子
        let root = build_left_skewed_tree(50);
        
        let iterations = 1000;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf(root.clone());
        }
        let dfs_time = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf_bfs(root.clone());
        }
        let bfs_time = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = Solution::sum_root_to_leaf_layer_contrib(root.clone());
        }
        let layer_time = start.elapsed();

        println!("\n=== 左斜树 (50节点, 1叶子) ===");
        println!("DFS (自顶向下):  {:?}", dfs_time);
        println!("BFS (自顶向下):  {:?}", bfs_time);
        println!("按层贡献 BFS:    {:?}", layer_time);
        println!("Ratio (Layer/BFS): {:.2}x", layer_time.as_secs_f64() / bfs_time.as_secs_f64());
    }
}
