"""
LeetCode 1404. 将二进制表示减到 1 的步骤数
"""


class Solution:
    def numSteps(self, s: str) -> int:
        """
        参数: s - 二进制字符串（如 "1101" 表示 13）
        返回: 将数字减到 1 所需的步骤数
        
        规则：
        - 偶数（末位是 '0'）→ 除以 2
        - 奇数（末位是 '1'）→ 加 1
        
        提示：
        1. 可以把字符串转成列表方便修改
        2. 偶数：直接去掉最后一个字符（pop）
        3. 奇数：加1会产生进位，从右往左处理连续的 '1'
        4. 循环直到字符串变成 "1"
        
        TODO: 在这里写你的代码
        """
        num = int(s, 2)
        cnt = 0
        if num < 1:
            return cnt
        while num != 1:
            if num % 2 == 0:
                num = num // 2
            else:
                num += 1
            cnt += 1
        return cnt


def main():
    """主函数：用于本地调试和测试"""
    solution = Solution()
    
    # 测试用例：（输入，期望输出）
    test_cases = [
        ("1101", 6),   # 13 → 14 → 7 → 8 → 4 → 2 → 1
        ("10", 1),     # 2 → 1
        ("1", 0),      # 已经是 1
        ("111", 4),    # 7 → 8 → 4 → 2 → 1
        ("101", 5),    # 5 → 6 → 3 → 4 → 2 → 1
        ("1111", 5),   # 15 → 16 → 8 → 4 → 2 → 1
    ]
    
    print("=" * 50)
    print("LeetCode 1404 - 练习版本")
    print("=" * 50)
    
    all_pass = True
    for s, expected in test_cases:
        result = solution.numSteps(s)
        status = "✓ PASS" if result == expected else "✗ FAIL"
        if result != expected:
            all_pass = False
        print(f"{status} | 输入: \"{s}\" | 期望: {expected} | 结果: {result}")
    
    print("=" * 50)
    if all_pass:
        print("🎉 全部通过！")
    else:
        print("❌ 有错误，请检查逻辑")


if __name__ == "__main__":
    main()
