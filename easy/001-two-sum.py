"""
题目: Two Sum
链接: https://leetcode.com/problems/two-sum/
难度: Easy
编号: 001
标签: Array, Hash Table

思路:
- 使用哈希表存储遍历过的数字及其索引
- 对于每个数字，检查 target - num 是否在哈希表中
- 如果存在，返回两个索引；否则将当前数字加入哈希表

时间复杂度: O(n)
空间复杂度: O(n)
"""

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}  # num -> index
        for i, num in enumerate(nums):
            complement = target - num
            if complement in seen:
                return [seen[complement], i]
            seen[num] = i
        return []  # 题目保证有解，这里不会执行


# 测试
if __name__ == "__main__":
    sol = Solution()
    assert sol.twoSum([2, 7, 11, 15], 9) == [0, 1]
    assert sol.twoSum([3, 2, 4], 6) == [1, 2]
    assert sol.twoSum([3, 3], 6) == [0, 1]
    print("All tests passed!")
