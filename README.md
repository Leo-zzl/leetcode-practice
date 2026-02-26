# LeetCode Practice

LeetCode 算法练习记录

## 📁 目录结构

```
.
├── easy/           # 简单难度
├── medium/         # 中等难度
├── hard/           # 困难难度
├── topics/         # 按主题分类
│   ├── array/
│   ├── string/
│   ├── linked-list/
│   ├── tree/
│   ├── graph/
│   ├── dp/
│   └── ...
└── README.md
```

## 📊 进度统计

| 难度 | 已完成 | 目标 |
|------|--------|------|
| Easy | 3 | 100 |
| Medium | 1 | 150 |
| Hard | 0 | 50 |

## ✅ 已完成题目

### Easy
- [1. 两数之和 (Two Sum)](https://leetcode.cn/problems/two-sum/)
  - 文件: `easy/lc1/main.py`
  - 测试: `easy/lc1/test_main.py`
  - 方法: 哈希表 O(n)

- [1022. 从根到叶的二进制数之和 (Sum of Root To Leaf Binary Numbers)](https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/)
  - 文件: `easy/lc1022/solution.py`
  - 测试: `easy/lc1022/test_solution.py`
  - 方法: DFS/BFS O(n)

- [1356. 根据数字二进制下 1 的数目排序](https://leetcode.cn/problems/sort-integers-by-the-number-of-1-bits/)
  - 文件: `easy/lc1356/solution.py`
  - 测试: `easy/lc1356/test_solution.py`
  - 方法: 查表法 (Lookup Table) O(n log n)
  - 优化: 针对 n ≤ 10000，仅需 2 次查表

### Medium
- [1404. 将二进制表示减到 1 的步骤数 (Number of Steps to Reduce a Number in Binary Representation to One)](https://leetcode.cn/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/)
  - 文件: `medium/lc1404/solution.py`
  - 测试: `medium/lc1404/test_solution.py`
  - 方法: 暴力解法（整数转换）O(n)

## 📝 刷题模板

### 文件名规范
```
{题号}.{语言后缀}
# 例如: lc1/main.py, lc1404/solution.py
```

### 文件内容模板
```python
"""
题目: Two Sum
链接: https://leetcode.com/problems/two-sum/
难度: Easy
标签: Array, Hash Table

思路:
- 方法1: 暴力枚举 O(n²)
- 方法2: 哈希表 O(n)

时间复杂度: O(n)
空间复杂度: O(n)
"""

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # 代码实现
        pass
```

## 🏷️ 常用标签

- Array / String
- Linked List
- Tree / Binary Tree / BST
- Graph / BFS / DFS
- Dynamic Programming
- Backtracking
- Greedy
- Binary Search
- Heap / Priority Queue
- Stack / Queue

---

Happy Coding! 💪
