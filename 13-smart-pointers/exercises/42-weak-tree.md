# Exercise 42 — Weak Tree

- **Title:** Weak Tree
- **Objective:** Build a parent/child tree where children own nothing upward, using `Weak<T>` to avoid reference cycles.
- **Difficulty:** Advanced
- **Concepts practiced:** `Rc<T>`, `Weak<T>`, `RefCell<T>`, `Rc::downgrade`, `Weak::upgrade`, `Rc::strong_count`, `Rc::weak_count`

## Instructions

1. Define:
   ```rust
   struct Node {
       name: String,
       parent: RefCell<Weak<Node>>,
       children: RefCell<Vec<Rc<Node>>>,
   }
   ```
2. Implement:
   - `fn Node::new(name: &str) -> Rc<Node>`
   - `fn parent(&self) -> Option<Rc<Node>>` (upgrade the weak link)
   - `fn path(&self) -> String` — e.g. `"root/usr/bin"`
   - `fn depth(&self) -> usize` — root is `0`
   - `fn count_descendants(&self) -> usize`
3. Implement `fn add_child(parent: &Rc<Node>, child: Rc<Node>)` that sets the child's weak parent link and stores the child.
4. In `main`, print `Rc::strong_count` / `Rc::weak_count` before and after building the tree, and after the parents go out of scope.

## Expected behavior

- For `root → usr → bin`: `bin.path() == "root/usr/bin"`, `bin.depth() == 2`, `root.count_descendants() == 2`.
- `root` has `strong = 1, weak = 1` (one child points back at it weakly).
- After `root` and `usr` are dropped, `bin.parent()` is `None` — no leak, no cycle.

## Optional hints

- If `parent` were `Rc<Node>`, parent and child would keep each other alive forever (a cycle).
- `Rc::downgrade(&rc)` creates a `Weak`; `weak.upgrade()` returns `Option<Rc<T>>`.

## Bonus challenge

Add `fn find(root: &Rc<Node>, name: &str) -> Option<Rc<Node>>` (DFS) and `fn remove_child(parent: &Rc<Node>, name: &str)` that also clears the removed child's parent link.
