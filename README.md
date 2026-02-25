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
| Easy | 2 | 100 |
| Medium | 0 | 150 |
| Hard | 0 | 50 |

## ✅ 已完成题目

### Easy
- [1. 两数之和 (Two Sum)](https://leetcode.cn/problems/two-sum/)
  - 文件: `easy/py1/main.py`
  - 测试: `easy/py1/test_main.py`
  - 方法: 哈希表 O(n)

- [1356. 根据数字二进制下 1 的数目排序](https://leetcode.cn/problems/sort-integers-by-the-number-of-1-bits/)
  - 文件: `easy/py1356/solution.py`
  - 测试: `easy/py1356/test_solution.py`
  - 方法: 查表法 (Lookup Table) O(n log n)
  - 优化: 针对 n ≤ 10000，仅需 2 次查表

## 📝 刷题模板

### 文件名规范
```
{题号}-{题目名}.{语言后缀}
# 例如: 001-two-sum.py
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
