# Exercise 28 — Generic Max

- **Title:** Generic Max
- **Objective:** Write generic functions that find the largest value in a pair and in a slice.
- **Difficulty:** Intermediate
- **Concepts practiced:** generic functions, trait bounds (`PartialOrd`, `Ord`, `Copy`), `Option`, slices, `where` clauses

## Instructions

1. Implement `max_of_two<T: PartialOrd>(a: T, b: T) -> T`.
   - If the values are equal (or incomparable, like `NaN`), return `a`.
2. Implement `largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T>`.
   - Return `None` for an empty slice.
3. Implement `largest_ref<T: PartialOrd>(items: &[T]) -> Option<&T>`.
   - Same as `largest`, but works for non-`Copy` types (like `String`) by returning a reference.
4. Implement `max_by_key_generic<T, K, F>(items: &[T], key: F) -> Option<&T>` using a `where` clause:
   - `K: Ord`, `F: Fn(&T) -> K`.
   - Return the item whose key is the largest (first one on ties).
5. Call all functions from `main` with integers, floats, chars and `String`s.

## Expected behavior

- `max_of_two(3, 7)` => `7`
- `max_of_two('a', 'z')` => `'z'`
- `largest(&[1, 5, 3])` => `Some(5)`
- `largest::<i32>(&[])` => `None`
- `largest_ref(&["pear".to_string(), "apple".to_string()])` => `Some(&"pear")`
- `max_by_key_generic(&["hi", "hello", "hey"], |s| s.len())` => `Some(&"hello")`

## Optional hints

- `PartialOrd` gives you `>` / `<` even for `f64`, but `Ord` is required for total ordering.
- Iterate with `for item in items` (yields `&T`) and keep a "best so far" variable.
- `Iterator::max_by_key` returns the *last* max on ties — handle ties manually if you want the first.

## Bonus challenge

Write `min_max<T: PartialOrd + Copy>(items: &[T]) -> Option<(T, T)>` that finds both in a single pass.
