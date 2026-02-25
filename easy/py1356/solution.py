"""
LeetCode 1356. Sort Integers by The Number of 1 Bits
根据数字二进制下 1 的数目排序

题目描述：
给你一个整数数组 arr。请你将数组中的元素按照其二进制表示中数字 1 的数目升序排序。
如果存在多个数字二进制中 1 的数目相同，则必须将它们按照数值大小升序排列。

示例：
输入: arr = [0,1,2,3,4,5,6,7,8]
输出: [0,1,2,4,8,3,5,6,7]
解释:
- [0] 是唯一一个有 0 个 1 的数
- [1,2,4,8] 都有 1 个 1
- [3,5,6] 都有 2 个 1
- [7] 有 3 个 1
"""

from typing import List


class Solution:
    def sortByBits(self, arr: List[int]) -> List[int]:
        """
        按照二进制中 1 的数目升序排序，1 的数目相同则按数值大小排序
        
        Args:
            arr: 整数数组
            
        Returns:
            排序后的数组
        """
        # TODO: 请在这里实现你的解法
        pass


def main():
    """
    主函数：用于本地调试和测试
    """
    solution = Solution()
    
    # 示例 1
    arr1 = [0, 1, 2, 3, 4, 5, 6, 7, 8]
    result1 = solution.sortByBits(arr1)
    print(f"示例 1: {arr1}")
    print(f"结果 1: {result1}")
    print()
    
    # 示例 2
    arr2 = [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]
    result2 = solution.sortByBits(arr2)
    print(f"示例 2: {arr2}")
    print(f"结果 2: {result2}")
    print()
    
    # 示例 3
    arr3 = [10000, 10000]
    result3 = solution.sortByBits(arr3)
    print(f"示例 3: {arr3}")
    print(f"结果 3: {result3}")


if __name__ == "__main__":
    main()
