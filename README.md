# Comparing Sequential and Parallel Solutions for the "Detect Capital" Problem

This repository explores and compares the performance of sequential and parallel implementations for solving the [Detect Capital](https://leetcode.com/problems/detect-capital/description/) problem on LeetCode.

## Problem Overview

For a detailed problem description, please refer directly to the LeetCode link above. In summary, the problem requires verifying if the use of capital letters in a given word is correct according to specific rules.

## Solution Approach

The solution hinges on analyzing the first two characters to determine the case pattern. Once identified, the rest of the string must be checked to ensure it adheres to this pattern. The primary check is performed using the `.all` method to verify the correctness of the characters.

### Sequential Solution

The sequential version of the solution involves:
- Determining if the remainder of the word should be in all uppercase or all lowercase.
- Using the `.all` method to confirm that all subsequent characters match the expected case.

### Parallel Solutions

To improve performance, parallel versions using Rayon were implemented. Rayon offers an efficient parallel `.all` method, which serves as a baseline for comparison.

#### Implementations

1. **Parallel `.all` Method**:
   Utilizes Rayon’s built-in parallel `.all` to assess if the remaining characters meet the expected case.

2. **Map and Reduce Approach**:
   - Processes the word in parallel by mapping each character to a boolean indicating correctness and then reducing the results.
   - While logically straightforward, this approach is slightly slower than `.all` since it evaluates all characters even if an early termination is possible.

3. **`try_fold` Method**:
   - Stops as soon as an inconsistency is found, offering potential early exits.
   - Demonstrates competitive performance with the parallel `.all` method, sometimes slightly outperforming it.

4. **`rayon_all` Custom Implementation**:
   - An explicit implementation mimicking `.all` that uses `map` and `find_any` for parallel processing.
   - Proved to be the most performant in many cases.

## Performance Analysis

The performance metrics revealed:
- **Parallel `.all`**, **`rayon_all`**, and **try_fold`** generally exhibit similar, highly competitive performance.
- The **map and reduce** approach is consistently the slowest due to its lack of early termination.
- Results vary across test cases, with differences typically within milliseconds.

### Key Observations:
- For a simple problem like "Detect Capital," the differences between these approaches are minimal.
- The ranking of performance changes frequently with small margins, influenced by thread management and work-stealing behavior.
- When looking at the diagrams, most are extremely similiar, with the only one different being the **map and reduce**. The following images are of the `rayon_all` and **map and reduce** 

  -`rayon_all`
![Explict rayon all Diagram](./results/diagrams/rayon_all_half.svg)

  -`map` and `reduce` 
![Map Diagram](./results/diagrams/map_half.svg)

- One small detail about the diagrams, I don't really understand why some threads stay idle right before the division of labor

## Conclusion

The comparison shows that, for this problem, all implementations offer similiar results with the **map and reduce** being the only one slightly behind. The differences between top contenders are minimal, suggesting that any of these approaches could be suitable based on context and specific use cases.

For detailed measurement data and performance diagrams, please refer to the `/results` directory.
