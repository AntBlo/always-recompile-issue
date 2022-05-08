# Always recompile issue
When running `cargo test` from a test function (using `-C instrument-coverage`), external libraries are always recompiled.

The below results are from secondary runs after one initial run to generate the cache in <git_repo_root>/target

The interesting part is the compilation of crate a, which has a dependency on the tokio crate.

You can see that the two first runs reports `finished in 0.00s`

And the third one reports `finished in 0.48s`, with two re-compilations.

Is this expected behaviour?

### Running tests manually in shell function

```
(base) ab@desktop:~/Documents/GitLab/always-recompile-issue$ RUSTFLAGS="-C instrument-coverage" cargo test --workspace
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/a-c4c8128d38ad438f)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/b-dc9641536de97df3)

running 1 test
test test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/c-5d405b9c7d96c75f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests a

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

(base) ab@desktop:~/Documents/GitLab/always-recompile-issue$
```

### Running tests programmatically in main function

```
> Executing task: cargo run --package b --bin b <

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/b`
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/a-c4c8128d38ad438f)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/b-dc9641536de97df3)

running 1 test
test test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/c-5d405b9c7d96c75f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[b/src/main.rs:13] cargo_test_command.get_current_dir() = Some(
    "/home/ab/Documents/GitLab/always-recompile-issue",
)

Terminal will be reused by tasks, press any key to close it.
```

### Running tests programmatically in test function
```
> Executing task: cargo test --package b --bin b -- test --exact --nocapture --ignored <

   Compiling b v0.1.0 (/home/ab/Documents/GitLab/always-recompile-issue/b)
    Finished test [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests (target/debug/deps/b-dc9641536de97df3)

running 1 test
   Compiling b v0.1.0 (/home/ab/Documents/GitLab/always-recompile-issue/b)
    Finished test [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests (target/debug/deps/a-c4c8128d38ad438f)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/b-dc9641536de97df3)

running 1 test
test test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/c-5d405b9c7d96c75f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[b/src/main.rs:28] cargo_test_command.get_current_dir() = Some(
    "/home/ab/Documents/GitLab/always-recompile-issue",
)
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s


Terminal will be reused by tasks, press any key to close it.
```
