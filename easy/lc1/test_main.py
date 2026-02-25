import pytest
from main import Solution


solution = Solution()


def test_two_sum_example1():
    """示例1: nums = [2,7,11,15], target = 9, 输出: [0,1]"""
    nums = [2, 7, 11, 15]
    target = 9
    result = solution.twoSum(nums, target)
    assert sorted(result) == [0, 1]


def test_two_sum_example2():
    """示例2: nums = [3,2,4], target = 6, 输出: [1,2]"""
    nums = [3, 2, 4]
    target = 6
    result = solution.twoSum(nums, target)
    assert sorted(result) == [1, 2]


def test_two_sum_example3():
    """示例3: nums = [3,3], target = 6, 输出: [0,1]"""
    nums = [3, 3]
    target = 6
    result = solution.twoSum(nums, target)
    assert sorted(result) == [0, 1]


def test_two_sum_negative_numbers():
    """测试负数: nums = [-1,-2,-3,-4,-5], target = -8"""
    nums = [-1, -2, -3, -4, -5]
    target = -8
    result = solution.twoSum(nums, target)
    assert sorted(result) == [2, 4]


def test_two_sum_mixed_numbers():
    """测试正负混合: nums = [-3,4,3,90], target = 0"""
    nums = [-3, 4, 3, 90]
    target = 0
    result = solution.twoSum(nums, target)
    assert sorted(result) == [0, 2]


def test_two_sum_large_numbers():
    """测试大数: nums = [1000000000, 1000000000, 999999999], target = 1999999999"""
    nums = [1000000000, 1000000000, 999999999]
    target = 1999999999
    result = solution.twoSum(nums, target)
    assert sorted(result) == [1, 2]


def test_two_sum_zero():
    """测试包含0: nums = [0,4,3,0], target = 0"""
    nums = [0, 4, 3, 0]
    target = 0
    result = solution.twoSum(nums, target)
    assert sorted(result) == [0, 3]


def test_two_sum_result_order():
    """测试返回顺序可以不同: nums = [1,2,3,4], target = 5"""
    nums = [1, 2, 3, 4]
    target = 5
    result = solution.twoSum(nums, target)
    # 可能返回 [0,3] 或 [1,2]
    assert sorted(result) in [[0, 3], [1, 2]]


def test_two_sum_distant_indices():
    """测试索引距离较远: nums = [1,2,3,4,5,6,7,8,9,10], target = 11"""
    nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    target = 11
    result = solution.twoSum(nums, target)
    # 验证返回的两个元素和为target
    assert nums[result[0]] + nums[result[1]] == target
    assert result[0] != result[1]


def test_two_sum_at_beginning():
    """测试解在开头: nums = [10, 20, 30], target = 30"""
    nums = [10, 20, 30]
    target = 30
    result = solution.twoSum(nums, target)
    assert sorted(result) == [0, 1]


def test_two_sum_at_end():
    """测试解在结尾: nums = [10, 20, 30], target = 50"""
    nums = [10, 20, 30]
    target = 50
    result = solution.twoSum(nums, target)
    assert sorted(result) == [1, 2]


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
