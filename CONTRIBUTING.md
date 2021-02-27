# Welcome!

## How to contribute to Challenger

#### **Setting up a development environment**

* First things first, clone the Challenger repository.
* Make sure you can compile the project. If you can't run `cargo --version`
  then you likely need to [install Rust](https://www.rust-lang.org/tools/install).
* **The most important step by far** is to understand the code that is already
  in place. Attempting to develop and add new features without understanding
  the codebase will likely result in adding code that will only introduce bugs
  or performance issues to your development branch.

#### **Is there an optimization you would like to introduce to challenger?**

**Create a branch** and optimize away! We're still working out a way to
benchmark challenger as a while so stay tuned on how to prove your
optimization improves on previous implementation. Benchmark tests that show
improvements within one or more functions will be accepted as would
demonstrating an improvment in the current level of stockfish beaten.

#### **Did you find a bug?**

1. **Ensure the bug was not already reported** by searching the current
[Challenger Issues](https://github.com/folksgl/challenger-rs/issues).

If you're unable to find an open issue addressing the problem,
[open a new one](https://github.com/folksgl/challenger-rs/issues/new).
Be sure to include a **title, clear description**, as much relevant information
as possible, and a **code sample** or an **executable test case** demonstrating
the expected behavior that is not occurring.

#### **Did you write a patch that fixes a bug?**

* Open a new GitHub pull request with the patch by heading to the
  [pull request page](https://github.com/folksgl/challenger/pulls).

* Ensure the PR description clearly describes the problem and solution.
  Include the relevant issue number if applicable.

#### **Do you intend to add a new feature or change an existing one?**

* Open a new GitHub pull request on the
  [pull request page](https://github.com/folksgl/challenger/pulls).
  Make sure to write and submit the relevant tests for your code as this will
  help speed the request along.

