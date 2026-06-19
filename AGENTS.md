# AGENTS.md

## Purpose

This project is a learning-focused retro FPS built in Rust. Work should optimize for clarity, testability, and visible progress over premature abstraction.

## Project Direction

- Build a 90s-style FPS inspired by Wolfenstein 3D and Doom.
- Start with a Wolfenstein-style raycaster, not a Doom-style sector engine.
- Use Rust with `macroquad` for the first prototype.
- Keep the first goal to one tiny playable level rather than a general engine.

## Rust Experience Assumption

Assume the project owner is new to Rust.

That means:

- Prefer simple, idiomatic Rust over advanced patterns.
- Explain Rust-specific choices in plain language when they affect structure or tradeoffs.
- Avoid unnecessary traits, lifetimes, macros, generics, and architectural indirection unless they are clearly justified by the current step.

## Core Working Rules

- Every coding task should be the smallest useful slice that creates both a testable software change and a visible result a human can run.
- Prefer vertical slices over internal-only groundwork.
- Avoid speculative abstractions until a second real use case exists.
- Keep rendering, input, simulation, map data, collision, and game rules separated unless there is a strong reason not to.
- APIs should stay small, explicit, and easy to test without the full game loop when practical.

## Planning

- Before implementation, define the task using the canonical format in [docs/task-template.md](docs/task-template.md).
- Planning should be understandable to a Rust beginner.
- Questions and recommendations should explain the practical tradeoff in plain language.
- Task specs should be specific enough that another engineer could implement the step without guessing ownership, API shape, tests, or manual QA.

## Subagents

- Use subagents selectively when they reduce risk or clarify a design decision.
- Prefer the architecture scout for new subsystems or ownership-boundary questions.
- Prefer the Rust pedagogy reviewer when a plan or implementation may be harder for a Rust beginner to follow than necessary.
- Prefer the correctness reviewer for math-heavy, state-heavy, or edge-case-heavy logic.
- Prefer the playability observer for visible movement, controls, rendering readability, or feedback changes.
- Subagents are advisory by default unless the task explicitly says their review is blocking.
- Reviewer prompts and expected output formats live in [docs/subagents.md](docs/subagents.md).

## Documentation

- `docs/ai-project-snapshot.md` is the current-state reference. Keep it short and update it when behavior, module ownership, tests, or known limits change.
- `docs/development-log.md` is append-only history. Add an entry when a completed step changes visible behavior or meaningfully changes project structure or workflow.

## Definition Of Done

A task is done only when all of the following are true:

- The planned behavior exists in code.
- There is an observable result in the running project, or the task explicitly explains why that is not applicable.
- Relevant unit tests pass, or the task explains why no automated test applies yet.
- Manual QA steps are short, concrete, and repeatable.
- Relevant documentation is updated when current behavior, ownership boundaries, tests, known limits, or workflow expectations change.
- Before declaring a coding task complete, run `./scripts/check.sh`.
- Do not claim success if `./scripts/check.sh` fails. Report the failure and its likely cause.

## Current References

- Current project state: [docs/ai-project-snapshot.md](docs/ai-project-snapshot.md)
- Development history: [docs/development-log.md](docs/development-log.md)
- Task planning template: [docs/task-template.md](docs/task-template.md)
