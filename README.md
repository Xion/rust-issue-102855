# rust-lang/rust #102855

Minimal (?) repro.

```
$ cargo build

Compiling rust-issue-102855 v0.1.0 (/home/xion/Code/Experiments/rust-issue-102855)
error[E0432]: unresolved import `crate::common::animation::AnimationPlugin`
 --> src/app.rs:3:5
  |
3 | use crate::common::animation::AnimationPlugin;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rust-issue-102855` due to previous error

$ rustc --version
rustc 1.64.0 (a55dd71d5 2022-09-19)

```
