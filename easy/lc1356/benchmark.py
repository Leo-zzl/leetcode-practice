"""
LeetCode 1356 解法性能测试
对比不同 bit_count 实现
"""

import time
import random
from typing import List


# ============ 各种 bit_count 实现 ============

def count_bits_builtin(n: int) -> int:
    """Python 3.10+ 内置方法（最快）"""
    return n.bit_count()


def count_bits_bin(n: int) -> int:
    """bin字符串计数（最简洁）"""
    return bin(n).count('1')


def count_bits_kernighan(n: int) -> int:
    """Brian Kernighan算法"""
    count = 0
    while n:
        n &= n - 1
        count += 1
    return count


# 查表法（预计算）
BIT_COUNT_TABLE = [bin(i).count('1') for i in range(256)]


def count_bits_lookup(n: int) -> int:
    """查表法"""
    return (BIT_COUNT_TABLE[n & 0xFF] +
            BIT_COUNT_TABLE[(n >> 8) & 0xFF] +
            BIT_COUNT_TABLE[(n >> 16) & 0xFF])


def count_bits_shift(n: int) -> int:
    """逐位检查"""
    count = 0
    while n:
        count += n & 1
        n >>= 1
    return count


# ============ 完整排序解法 ============

def sort_by_bits(arr: List[int], counter) -> List[int]:
    """通用排序函数"""
    return sorted(arr, key=lambda x: (counter(x), x))


# ============ 计时测试 ============

def benchmark():
    print("=" * 60)
    print("LeetCode 1356 - 按二进制1的个数排序 性能测试")
    print("=" * 60)
    
    # 测试数据
    sizes = [100, 1000, 10000]
    methods = [
        ("bit_count() 内置", count_bits_builtin),
        ("bin().count()", count_bits_bin),
        ("Kernighan算法", count_bits_kernighan),
        ("查表法", count_bits_lookup),
        ("逐位移位", count_bits_shift),
    ]
    
    for size in sizes:
        print(f"\n📊 数组大小: {size} 个元素")
        print("-" * 60)
        
        # 生成随机测试数据 (0-10000)
        arr = [random.randint(0, 10000) for _ in range(size)]
        iterations = max(1, 10000 // size)
        
        results = []
        for name, counter in methods:
            start = time.perf_counter()
            for _ in range(iterations):
                sort_by_bits(arr.copy(), counter)
            elapsed = time.perf_counter() - start
            results.append((name, elapsed))
        
        # 排序显示
        results.sort(key=lambda x: x[1])
        best_time = results[0][1]
        
        for i, (name, elapsed) in enumerate(results):
            ratio = elapsed / best_time
            bar_len = int(25 / ratio) if ratio > 0 else 0
            bar = "█" * bar_len
            print(f"  {i+1}. {name:18s}: {elapsed*1000:8.3f}ms ({ratio:5.2f}x) {bar}")
    
    # 单独测试 bit_count 函数本身
    print("\n" + "=" * 60)
    print("纯 bit_count 函数调用测试 (100万个数)")
    print("=" * 60)
    
    nums = [random.randint(0, 10000) for _ in range(1_000_000)]
    
    func_results = []
    for name, counter in methods:
        start = time.perf_counter()
        for n in nums:
            counter(n)
        elapsed = time.perf_counter() - start
        func_results.append((name, elapsed))
    
    func_results.sort(key=lambda x: x[1])
    best = func_results[0][1]
    
    for i, (name, elapsed) in enumerate(func_results):
        ratio = elapsed / best
        print(f"  {i+1}. {name:18s}: {elapsed:.4f}s ({ratio:.2f}x)")


def test_correctness():
    """验证所有方法结果一致"""
    print("\n" + "=" * 60)
    print("正确性验证")
    print("=" * 60)
    
    arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 100, 1000, 10000]
    
    methods = [
        count_bits_builtin,
        count_bits_bin,
        count_bits_kernighan,
        count_bits_lookup,
        count_bits_shift,
    ]
    
    for n in arr:
        results = [f(n) for f in methods]
        binary = bin(n)
        if len(set(results)) != 1:
            print(f"❌ n={n}: 结果不一致! {results}")
            return False
    
    print("✅ 所有方法结果一致")
    return True


if __name__ == "__main__":
    if test_correctness():
        benchmark()
