# Solutions

This folder contains reference implementations for exercises.

Suggested workflow:
1. Attempt exercise on your own first.
2. Compile and test your answer.
3. Compare with the matching solution file here.
4. Refactor your answer using what you learned.

Running a solution and its tests:

```bash
rustc --edition 2021 solutions/05-structs-enums/11-user-model.rs -o app && ./app
rustc --edition 2021 --test solutions/05-structs-enums/11-user-model.rs -o tests && ./tests
```

Verify every starter and solution at once with `python3 scripts/check_exercises.py`.
You can also browse all solutions on the course website: <https://yisusvii.github.io/RustExcersices/>.
