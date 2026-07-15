# Problem: Towers

CSES URL: https://cses.fi/problemset/task/1073

## Statement

You are given a sequence of cubes. Build towers by placing each cube on top of an existing tower only if the top cube of that tower is **strictly larger** than the new cube. Otherwise, start a new tower.

Determine the minimum number of towers after processing all cubes.

See the official CSES page for complete details.

Source: https://cses.fi/problemset/task/1073

---

## Solution

Pattern: Greedy + Patience Sorting

Technique: Maintain a sorted list containing the top cube of every tower. For each incoming cube, use binary search (`partition_point`) to find the first tower whose top is **greater** than the cube. Replace that tower's top with the new cube. If no such tower exists, start a new tower.

Time: **O(n log n)**

Space: **O(n)**

---

## Insight

The only information that matters about a tower is its current top cube.

To maximize future placement opportunities, always place a cube on the eligible tower with the **smallest** top cube that is still larger than the incoming cube. Replacing a larger top with a smaller one keeps the number of towers unchanged while making that tower easier to use later.

The vector of tower tops remains sorted throughout the algorithm, allowing binary search for every insertion.

In Rust, this is naturally expressed as:

```rust
let pos = towers.partition_point(|&top| top <= cube);
```

`partition_point` returns the first element for which the predicate is false. Since the predicate is `top <= cube`, the returned position is the first tower whose top is **greater** than the incoming cube—the exact tower required by the problem.

This is equivalent to C++:

```cpp
auto it = upper_bound(towers.begin(), towers.end(), cube);
```

---

## Edge Cases Checklist

- Single cube (answer is 1).
- All cubes equal (every cube starts a new tower).
- Strictly increasing sequence (every cube starts a new tower).
- Strictly decreasing sequence (all cubes fit into one tower).
- Duplicate values intermixed with unique values.
- Maximum input size (verify O(n log n) performance).

---

## Key Takeaways

- This is another application of the **patience sorting** technique.
- The algorithm stores **tower tops**, not the towers themselves.
- Binary search is possible because the tower tops remain sorted after every update.
- The choice is locally greedy but globally optimal.
- Remember the distinction:
    - Towers → first value **> x** (`upper_bound`, `partition_point(|t| t <= x)`)
    - LIS → first value **>= x** (`lower_bound`, `partition_point(|t| t < x)`)