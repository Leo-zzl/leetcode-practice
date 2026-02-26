"""
LeetCode 1404 - 测试用例
"""

import unittest
from solution import Solution, SolutionV2


class TestSolution(unittest.TestCase):
    """测试 Solution 类"""
    
    def setUp(self):
        self.solution = Solution()
    
    def test_example_1(self):
        """示例 1: s = \"1101\" -> 6"""
        self.assertEqual(self.solution.numSteps("1101"), 6)
    
    def test_example_2(self):
        """示例 2: s = \"10\" -> 1"""
        self.assertEqual(self.solution.numSteps("10"), 1)
    
    def test_example_3(self):
        """示例 3: s = \"1\" -> 0"""
        self.assertEqual(self.solution.numSteps("1"), 0)
    
    def test_single_bit_one(self):
        """单个比特 1"""
        self.assertEqual(self.solution.numSteps("1"), 0)
    
    def test_all_ones(self):
        """全为 1 的二进制数: \"111\" = 7 -> 8 -> 4 -> 2 -> 1, 共4步"""
        self.assertEqual(self.solution.numSteps("111"), 4)
    
    def test_power_of_two(self):
        """2 的幂次: \"1000\" = 8 -> 4 -> 2 -> 1, 共3步"""
        self.assertEqual(self.solution.numSteps("1000"), 3)
    
    def test_odd_number(self):
        """奇数: \"101\" = 5 -> 6 -> 3 -> 4 -> 2 -> 1, 共5步"""
        self.assertEqual(self.solution.numSteps("101"), 5)
    
    def test_large_even(self):
        """较大的偶数: \"10000\" = 16, 需要4步"""
        self.assertEqual(self.solution.numSteps("10000"), 4)
    
    def test_binary_three(self):
        """二进制 11 = 3 -> 4 -> 2 -> 1, 共3步"""
        self.assertEqual(self.solution.numSteps("11"), 3)
    
    def test_binary_two(self):
        """二进制 10 = 2 -> 1, 共1步"""
        self.assertEqual(self.solution.numSteps("10"), 1)
    
    def test_long_binary_with_zeros(self):
        """包含多个0的二进制: \"1010\" = 10 -> 5 -> 6 -> 3 -> 4 -> 2 -> 1, 共6步"""
        self.assertEqual(self.solution.numSteps("1010"), 6)
    
    def test_carry_propagation(self):
        """测试进位传播: \"1111\" = 15 -> 16 -> 8 -> 4 -> 2 -> 1, 共5步"""
        self.assertEqual(self.solution.numSteps("1111"), 5)


class TestSolutionV2(unittest.TestCase):
    """测试 SolutionV2 类（十进制转换方法）"""
    
    def setUp(self):
        self.solution = SolutionV2()
    
    def test_example_1(self):
        """示例 1: s = \"1101\" -> 6"""
        self.assertEqual(self.solution.numSteps("1101"), 6)
    
    def test_example_2(self):
        """示例 2: s = \"10\" -> 1"""
        self.assertEqual(self.solution.numSteps("10"), 1)
    
    def test_example_3(self):
        """示例 3: s = \"1\" -> 0"""
        self.assertEqual(self.solution.numSteps("1"), 0)
    
    def test_all_ones(self):
        """全为 1 的二进制数"""
        self.assertEqual(self.solution.numSteps("111"), 4)
    
    def test_power_of_two(self):
        """2 的幂次"""
        self.assertEqual(self.solution.numSteps("1000"), 3)


if __name__ == "__main__":
    unittest.main(verbosity=2)
