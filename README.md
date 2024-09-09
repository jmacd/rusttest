Rust learner asks for help!

## Problem

In [src/main.rs](./src/main.rs), see a mock-up of the problem I ran into (from https://github.com/jmacd/duckpond).

The types are renamed for clarity.

- *System* represents a file system, representing the global state and containing a root directory. The program is single-threaded and control-flow always has an associated `&mut System`.
- *Folder* is a trait for directories, real or dynamic, with a `subdir()` method
- *Realdir* is a real directory
- *Dynadir* is a dynamic directory.
- *WD* is a working directory a single handle to a pair of `&mut System` and `Rc<RefCell<dyn Folder>>` with a `lookup` method

The problem is that I do not understand how to return a `Rc<RefCell<dyn Folder>>` from `subdir()` with proper lifetime annotations (with or without `'_`, as suggested by the compiler, or a variety of other rearrangements I've tried).

The error is:

```rust
error: lifetime may not live long enough
  --> src/main.rs:43:9
   |
42 |     fn subdir(&mut self, sys: &mut System, _name: &str) -> Option<Rc<RefCell<dyn Folder>>> {
   |                               - let's call the lifetime of this reference `'1`
43 |         Some(sys.wd().lookup(Path::new("d/e/f")))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type `RefCell<dyn Folder>`, which makes the generic argument `dyn Folder` invariant
   = note: the struct `RefCell<T>` is invariant over the parameter `T`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
help: to declare that the trait object captures data from argument `sys`, you can add an explicit `'_` lifetime bound
   |
42 |     fn subdir(&mut self, sys: &mut System, _name: &str) -> Option<Rc<RefCell<dyn Folder + '_>>> {
   |                                                                                         ++++

```
