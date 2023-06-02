Contributing
======================

Pull requests (PRs)
---------------------

- Create draft pull requests and change the status to ready when the PR is ready.
- PRs must pass certain automated CI checks. You can use the local xtask crate to check
  them before commiting (`./target/debug/xtask ci --help`, `xtask/README.md`), however,
  note that the local coverage report might be different to the remote CI coverage
  report (don't know how to fix that).
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
- Use the *imperative voice*: "Fix bug" rather than "Fixed bug" or "Fixes bug."
- Try to keep the first line under **50** characters and the following lines under
  **72** characters.
- A blank line must follow the subject.

[conventional_commits]: https://www.conventionalcommits.org
[git-feature-branch]: https://www.atlassian.com/git/tutorials/comparing-workflows
