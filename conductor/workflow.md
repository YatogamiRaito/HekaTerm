# HekaTerm — Development Workflow

## Overview

This workflow governs how Conductor agents implement tracks, manage commits, and verify quality at each step.

---

## Task Workflow

For every task in a `plan.md`, the agent MUST follow these steps in order:

### 1. Read Before Writing
- Read all files relevant to the task before making any changes.
- Understand existing code patterns and conventions before adding new code.

### 2. Implement
- Make the minimal change necessary to complete the task.
- Follow the guidelines in `product-guidelines.md` strictly.
- Do not refactor code outside the task scope.
- Do not add unused abstractions or future-proofing.

### 3. Lint & Format
Run and pass both checks before committing:
```bash
cargo fmt
cargo clippy -- -D warnings
```
If either fails, fix all issues before proceeding.

### 4. Test
- Write unit tests for new logic in the same file (`#[cfg(test)]` module).
- Run the test suite:
```bash
cargo test 2>&1
```
- All tests must pass. Do not proceed if any test fails.

### 5. Commit
Commit using Conventional Commits format (see `product-guidelines.md`):
```bash
git add <specific files>
git commit -m "<type>(<scope>): <description>"
```
- Stage only files relevant to the task. Never use `git add -A` or `git add .` blindly.
- Commit after **every completed task** (not phase).

### 6. Update Plan
After a successful commit:
- Mark the task as complete in `plan.md`: change `- [ ]` → `- [x]`
- Append the short commit SHA to the task line: `- [x] Task: ... <sha>`
- Commit the plan update:
```bash
git commit -m "conductor(plan): Mark task '<description>' as complete"
```

---

## Phase Completion Verification and Checkpointing Protocol

At the end of each **Phase** in a track's `plan.md`, a meta-task will be present:

```
- [ ] Task: Conductor - User Manual Verification '<Phase Name>' (Protocol in workflow.md)
```

When this task is reached, the agent MUST:

1. **Summarize** what was implemented in the phase (files changed, tests added, commits made).
2. **Run** the full test suite one final time: `cargo test`
3. **Ask the user** to manually verify the phase using the `ask_user` tool:
   - Describe what was built and what to verify.
   - Wait for explicit approval before proceeding to the next phase.
4. **On approval:** Mark the meta-task as complete and commit.
5. **On rejection:** Ask what needs to be fixed and loop back to the relevant task.

---

## Commit Frequency

- **Per task:** Commit after every successfully completed and tested task.
- **Per phase verification:** Additional commit for the phase meta-task.

## Test Coverage

- **Target:** >80% line coverage for new code.
- Every new module must have at least one unit test.
- Rendering and GPU code: integration tests where feasible; mock GPU state where not.

## Forbidden Actions

- Do not skip `cargo fmt` or `cargo clippy` checks.
- Do not commit with `--no-verify`.
- Do not reintroduce `glium`, `glutin`, or EGL dependencies.
- Do not add `unsafe` without a `// SAFETY:` comment.
- Do not amend already-pushed commits.
