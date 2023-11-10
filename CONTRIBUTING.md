Contributing
======================

Pull requests (PRs)
---------------------

- Create draft pull requests and change the status to ready when the PR is ready.
- PRs must should certain automated CI checks. You can use the local xtask crate to check
  them before commiting (`./target/debug/xtask ci --help`, `xtask/README.md`), however,
  note that the local coverage report might be different to the remote CI coverage
  report (don't know how to fix that).
- Try to include tests.
- Use a [feature branch][git-feature-branch] instead of the master branch.

### Commit messages

Follow the [conventional commits guidelines][conventional_commits] to make the git logs
more valuable. The general structure is:
```
<type>([optional scope]): <short-description>

[optional body]

[optional footer(s)]
```
- The *description* shouldn't start with a capital letter or end in a period.
- Use the **imperative voice** for the *description*: "Fix bug" rather than "Fixed bug"
  or "Fixes bug."
- Try to keep the *description* under **50** characters and the following lines under
  **72** characters.
- A blank line must follow the *description*.

### Adding more Clifford gates to the tracker

In the following list, you may only do the first item, and then ask for help from the
developer, if you don't want to get into the source code. However, note that this may
take of course more time (and implementing it is usually just some simple boilerplate
code).

I hope the following list includes all steps ...

1. Proof the mapping in [conjugation-rules] as it is done for the other gates (i.e.,
   show how it can be represented through the Clifford generators (or gates derive from
   them) and how they conjugate the X and Z Paulis.
2. Provide a default method in the `Tracker` trait based on the required Clifford gates
   (or gates derived from them).
3. If the method is not trivial, i.e., not just one function call, and it is a single
   qubit gate, provide a default method for the `Pauli` trait in similar fashion. In
   this case also do the following: Implement the method directly for the `Live`
   tracker, by calling `Pauli`s method (you probably just need to add it to the
   `single!` macro call). Implement a more efficient direct implementation in
   `PauliEnum`'s, `PauliTuple`'s and `PauliDense`'s `Pauli` implementation. Do it
   analog for `PauliStack` and `Frames`, i.e., provide an efficient method
   implementation for `PauliStack` and call this function in `Frames` tracker
   implementation (`single!` macro ...).
4. If the method is not trivial, but more than qubit is involved, also implement the method
   directly, and more efficiently, for the `Live` and the `Frames` tracker.
5. In `tracker::tests::utils` add the method to the `single_actions`/`double_actions`
   macro and the expected results, based on the proof in (1.), to the
   `SINGLE_GENERATORS`/`DOUBLE_GENERATORS` (updating `N_SINGLES`/`N_DOUBLES`).
6. In the `circuit` module: Add the gate to the `CliffordCircuit` trait, to the
   `impl_gates` macro and to the `single_gate!`/`double_gate!` macro call.
7. In `tests/roundtrips/tracking.rs` add the gate to the `Operation` enum and to
   `prop_oneof` call in `operation` similar to the other gates (the number before "=>"
   is the probability (before normalization of the probabilities). In the
   `Instructor::apply`'s match call, add the case similar to the other (normal)
   single/double qubit gate cases.

[conventional_commits]: https://www.conventionalcommits.org
[git-feature-branch]: https://www.atlassian.com/git/tutorials/comparing-workflows
[conjugation-rules]: ./docs/conjugation_rules.md
