# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a LeetCode algorithm practice repository organized by difficulty level (`easy/`, `medium/`, `hard/`). Each problem resides in its own directory following the pattern `lc{problem_number}/`.

## Directory Structure

```
leetcode/
├── easy/
│   ├── lc1/              # Problem 1: Two Sum
│   ├── lc1356/           # Problem 1356: Sort Integers by Number of 1 Bits
│   └── lc1404/           # Problem 1404: Binary Steps (current directory)
├── medium/               # Medium difficulty problems
├── hard/                 # Hard difficulty problems
├── AGENTS.md             # Project guide (gitignored, but serves as reference)
├── README.md             # Project documentation (in Chinese)
└── .gitignore            # Unified ignore rules for Python/Rust
```

## Common Commands

### Python Projects

**Run solution locally:**
```bash
python3 solution.py
```

**Run tests with pytest (preferred):**
```bash
python3 -m pytest test_solution.py -v
pytest -v  # From problem directory
```

**Run tests with unittest:**
```bash
python3 -m unittest test_solution.py -v
```

### Rust Projects

**Build and run tests:**
```bash
cargo test
cargo test -- --nocapture  # With output
cargo build                # Build
cargo build --release      # Release build
```

## Code Architecture

### Python Solution Pattern

Each Python problem follows this structure:

```python
"""
LeetCode {number}. {Problem Name}
"""

class Solution:
    def problemMethod(self, params) -> ReturnType:
        """Implementation with docstring in Chinese"""
        pass

def main():
    """Main function for local debugging with test cases"""
    pass

if __name__ == "__main__":
    main()
```

**Key file: `solution.py` or `main.py`** - Contains the `Solution` class with the required LeetCode method signature and an optional `main()` function for local testing.

**Test file: `test_solution.py` or `test_main.py`** - Uses `unittest` framework with test classes like `TestSolution`. Include:
- Example tests from LeetCode
- Edge cases (empty inputs, single elements, boundary values)
- Problem-specific test cases

### Rust Solution Pattern

Each Rust problem uses Cargo with a library structure:

```rust
pub struct Solution;

impl Solution {
    pub fn method_name(params) -> ReturnType {
        // Implementation
    }
}
```

**Key file: `src/lib.rs`** - Contains the `Solution` struct implementation and inline tests under `#[cfg(test)]`.

**Configuration: `Cargo.toml`** - Defines the library name matching the problem directory.

## Development Guidelines

1. **Directory naming**: Use `lc{number}/` format (e.g., `lc1404/`)
2. **Documentation**: All docstrings and README files are written in Chinese
3. **Type hints**: Use Python type hints for all function signatures
4. **Test coverage**: Always include test cases for problem examples and edge cases
5. **Multiple solutions**: When implementing alternative solutions, use `SolutionV2`, `SolutionV3` naming pattern

## Git Ignore Rules

The root `.gitignore` excludes:
- Rust: `**/target/`, `**/Cargo.lock`
- Python: `**/__pycache__/`, `**/*.py[cod]`, `**/.pytest_cache/`
- IDE: `.idea/`, `.vscode/`
- OS: `.DS_Store`, `Thumbs.db`
- Virtual environments: `**/venv/`, `**/.venv/`

## Current Context

This directory (`lc1404/`) contains:
- **Problem**: LeetCode 1404 - "Number of Steps to Reduce a Number in Binary Representation to One"
- **Language**: Python
- **Current implementation**: Simple brute force using integer conversion (`int(s, 2)`)
- **Test file references**: `SolutionV2` class exists in test imports but may not be defined in solution.py

The solution converts binary string to integer, then iteratively applies:
- If even: divide by 2 (`num //= 2`)
- If odd: add 1 (`num += 1`)
- Count steps until reaching 1
