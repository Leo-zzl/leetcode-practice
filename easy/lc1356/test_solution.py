"""
LeetCode 1356 单元测试
测试 sortByBits 函数
"""

import unittest
from solution import Solution


class TestSortByBits(unittest.TestCase):
    
    def setUp(self):
        self.solution = Solution()
    
    # ===== 题目提供的示例 =====
    
    def test_example_1(self):
        """示例 1: [0,1,2,3,4,5,6,7,8] -> [0,1,2,4,8,3,5,6,7]"""
        arr = [0, 1, 2, 3, 4, 5, 6, 7, 8]
        expected = [0, 1, 2, 4, 8, 3, 5, 6, 7]
        # 二进制分析:
        # 0  -> 0000 (0 个 1)
        # 1  -> 0001 (1 个 1)
        # 2  -> 0010 (1 个 1)
        # 4  -> 0100 (1 个 1)
        # 8  -> 1000 (1 个 1)
        # 3  -> 0011 (2 个 1)
        # 5  -> 0101 (2 个 1)
        # 6  -> 0110 (2 个 1)
        # 7  -> 0111 (3 个 1)
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_example_2(self):
        """示例 2: 所有数都只有 1 个 1，直接按数值排序"""
        arr = [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]
        expected = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_example_3(self):
        """示例 3: 相同数字的数组"""
        arr = [10000, 10000]
        expected = [10000, 10000]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    # ===== 边界情况 =====
    
    def test_empty_array(self):
        """测试空数组"""
        arr = []
        expected = []
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_single_element(self):
        """测试单元素数组"""
        arr = [5]
        expected = [5]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_all_zeros(self):
        """测试全 0 数组"""
        arr = [0, 0, 0]
        expected = [0, 0, 0]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_all_same_bits(self):
        """测试所有数二进制 1 的个数相同但数值不同"""
        arr = [3, 5, 6, 9, 10, 12]  # 都有 2 个 1
        expected = [3, 5, 6, 9, 10, 12]  # 按数值排序
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    # ===== 特殊输入 =====
    
    def test_large_number(self):
        """测试较大的数 (不超过 10000)"""
        arr = [10000, 9999, 9998]
        # 10000 -> 10011100010000 (5 个 1)
        # 9999  -> 10011100001111 (7 个 1)
        # 9998  -> 10011100001110 (6 个 1)
        expected = [10000, 9998, 9999]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_reverse_sorted(self):
        """测试逆序数组"""
        arr = [7, 6, 5, 4, 3, 2, 1, 0]
        # 0: 0 个 1
        # 1, 2, 4: 各 1 个 1
        # 3, 5, 6: 各 2 个 1
        # 7: 3 个 1
        expected = [0, 1, 2, 4, 3, 5, 6, 7]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_already_sorted(self):
        """测试已排序的数组"""
        arr = [0, 1, 2, 4, 8, 3, 5, 6, 7]
        expected = [0, 1, 2, 4, 8, 3, 5, 6, 7]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_negative_not_in_scope(self):
        """测试非负数边界"""
        arr = [0, 1]
        expected = [0, 1]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    # ===== 多种 1 的个数混合 =====
    
    def test_mixed_bits_count(self):
        """测试多种 1 的个数混合"""
        arr = [15, 7, 3, 1, 0]  # 4个1, 3个1, 2个1, 1个1, 0个1
        expected = [0, 1, 3, 7, 15]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_same_bits_different_values(self):
        """测试相同 1 的个数但不同数值"""
        arr = [8, 4, 2, 1]  # 都只有 1 个 1
        expected = [1, 2, 4, 8]  # 按数值排序
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)
    
    def test_complex_case(self):
        """测试复杂情况"""
        arr = [10, 100, 1000, 10000]
        # 10    -> 1010      (2 个 1)
        # 100   -> 1100100   (3 个 1)
        # 1000  -> 1111101000 (6 个 1)
        # 10000 -> 10011100010000 (5 个 1)
        expected = [10, 100, 10000, 1000]
        result = self.solution.sortByBits(arr)
        self.assertEqual(result, expected)


if __name__ == "__main__":
    unittest.main()
