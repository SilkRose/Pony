# Trot

Trait library for chaining of vector operations, and for comparison for sorting.

## Example usage:

Sorting a vector:
```rust
let vec = vec![2, 1, 1, 2, 0, 3].sort_vec();
println!("{:?}", vec); // [0, 1, 1, 2, 2, 3]
```

Deduping a vector:
```rust
let vec = vec![2, 1, 1, 0, 0, 3].dedup_vec();
println!("{:?}", vec); // [2, 1, 0, 3]
```

Has more functions for sorting and deduping at the same time, and extending a vector, both returning `self`, like the two above examples.

Comparing within a tuple:
```rust
let mut list = vec![(3, 1), (2, 2), (1, 3)];
list.sort_by(|a, b| trot::compare(&a.0, &b.0));
println!("{:?}", list); // [(1, 3), (2, 2), (3, 1)]
```
