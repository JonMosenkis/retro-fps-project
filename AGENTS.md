# AGENTS.md

## Purpose

This project is a learning-focused retro FPS built in Rust. Work should optimize for clarity, testability, and visible progress rather than abstract infrastructure or premature generalization.

## Rust Experience Assumption

Assume the project owner is new to Rust.

That assumption should affect both planning and implementation:

- Prefer simple, idiomatic Rust over advanced patterns.
- Introduce Rust concepts gradually.
- Do not require Rust-specific background knowledge to understand a plan.
- Explain Rust-specific design choices in plain language when they affect structure or tradeoffs.
- Favor approaches that are easier to read, debug, and learn from, even if they are slightly less abstract or less optimized.
- Avoid unnecessary traits, lifetimes, macros, generics, and architectural indirection unless they are clearly justified by the current step.

## Project Direction

- Build a 90s-style FPS inspired by Wolfenstein 3D and Doom.
- Start with a Wolfenstein-style raycaster, not a Doom-style sector engine.
- Use Rust.
- Use `macroquad` for the first prototype.
- Keep the first goal to one tiny playable level rather than a general engine.

## Core Delivery Rule

Every coding task must be the smallest useful slice that produces both:

1. A testable software change.
2. A noticeable result a human can run and verify manually.

If a task does not create an observable change, it is probably too large, too abstract, or split at the wrong boundary.

## Task Design Guidelines

When planning or implementing work:

- Define the user-visible outcome first.
- Start from the highest-level description before choosing implementation details.
- Keep each step narrow enough to finish, run, and inspect.
- Prefer vertical slices over internal-only groundwork.
- Do not batch multiple gameplay systems into one task unless the result would otherwise be invisible.
- Avoid speculative abstractions until a second real use case exists.
- Plan both what will be built and how it will be built.

Each task should name:

- The high-level goal.
- The code change.
- The intended implementation approach.
- The expected visible/manual-QA result.
- The unit-test target.
- The acceptance criteria.

## Coding Guidelines

Code should be:

- Modular
- Easy to read
- Narrow in responsibility
- Explicit in behavior
- Practical rather than over-engineered

Prefer:

- Small modules with clear ownership boundaries
- Functions that do one thing well
- Types that represent domain concepts clearly
- Simple control flow over cleverness
- Names that explain intent without relying on comments
- Rust code that uses straightforward ownership patterns and keeps borrowing rules easy to follow

Avoid:

- Large files that mix unrelated concerns
- Functions that perform multiple layers of work at once
- Hidden coupling between gameplay, rendering, input, and data
- Premature general frameworks
- Reuse-by-copy when a stable abstraction is justified
- Rust designs that are technically elegant but hard for a beginner to read or modify

## Planning Requirements

Before implementation, planning should capture enough structure that the work is reviewable and testable.

Every task proposal should be written in one canonical format.

Each planned task should include:

1. High-level description
2. Implementation approach
3. Definition of done
4. Automated test cases
5. Manual QA cases
6. Code to be written
7. API design

Planning must also be understandable to someone who has never coded in Rust before.

- Explain each Rust-specific choice in plain language.
- State why a particular structure is being chosen, not just what it is.
- Prefer concrete examples over jargon when describing APIs or module boundaries.
- If a plan introduces a Rust concept that a beginner may not know, include a short explanation of why it is needed in this task.

### Planning Template

Use this exact structure when defining a task:

1. High-level description
   - What user-visible capability or behavior is being added?
2. Implementation approach
   - How will the change be built?
   - Which modules own the behavior?
   - What tradeoffs or constraints matter?
3. Definition of done
   - What must be true for the task to count as complete?
4. Automated test cases
   - Which logic will be verified by unit tests?
5. Manual QA cases
   - What should a human run and observe?
6. Code to be written
   - Which structs, enums, traits, functions, and modules are expected?
7. API design
   - What interfaces will exist between modules?
   - What inputs, outputs, and responsibilities will those interfaces have?

### Canonical Task Spec

Copy this template for every planned task:

```md
## Task: <short task name>

### 1. High-Level Description
- User-visible outcome:
- Why this step exists now:

### 2. Implementation Approach
- Approach summary:
- Modules involved:
- Key constraints or tradeoffs:

### 3. Definition of Done
- [ ]
- [ ]
- [ ]

### 4. Automated Test Cases
- Unit test:
- Unit test:

### 5. Manual QA Cases
- Run:
- Expect:

### 6. Code to Be Written
- Module(s):
- Struct(s)/enum(s)/trait(s):
- Function(s)/method(s):

### 7. API Design
- Public API:
- Internal API:
- Inputs and outputs:
- Ownership/responsibility boundaries:
```

### Task Review Checklist

Before approving a task for implementation, verify:

- The high-level description is visible and user-facing.
- The step is small enough to complete and inspect in one pass.
- The implementation approach explains how the code will be structured.
- The definition of done is concrete and checkable.
- Automated tests cover stable logic.
- Manual QA is short and repeatable.
- Expected modules, types, and functions are named.
- API boundaries are explicit.
- The step respects SOLID and does not introduce speculative abstractions.
- The task produces a noticeable result.
- The plan is understandable without prior Rust knowledge.

### Required Level of Specificity

The plan must be specific enough that another engineer could implement it without guessing:

- what module owns the behavior
- what public API is expected
- what pure logic should be unit tested
- what a human should see or do during QA

If those points are unclear, the task is not ready to implement.

## Question and Decision-Making Rules

Questions asked during planning must be written for a Rust beginner.

Each question should:

- use plain language first
- avoid unexplained Rust jargon
- explain why the decision matters
- describe the practical consequence of each option
- make the recommended option explicit when one exists

Do not ask a choice like:

- "Should we use a workspace?"
- "Should we introduce a trait here?"
- "Should this live behind an abstraction?"

unless the question also explains:

- what that means
- why it matters now
- what changes depending on the choice

When asking about implementation structure, phrase choices in terms of outcome and maintenance impact, for example:

- simpler now vs easier to extend later
- one file now vs one small module split now
- direct code now vs a reusable abstraction because duplication already exists

## Decision Explanation Requirement

When presenting options or recommendations:

- explain the recommendation in plain language
- state the tradeoff being optimized for
- state what extra complexity is being avoided
- state what future refactor may be expected if the simpler option is chosen

The goal is not just to collect an answer. The goal is to let the project owner make an informed decision.

## API and Design Expectations

- APIs should be identified during planning, not discovered only after coding starts.
- Planning should define the module boundaries and the main interfaces between them.
- API design does not need every detail frozen up front, but core responsibilities and call patterns should be explicit.
- Public interfaces should be small, coherent, and intention-revealing.
- Prefer passing well-structured data over loosely related primitive arguments when it improves clarity.
- Design APIs so pure logic can be tested without requiring the full game loop or renderer.

When a task introduces a new module or subsystem, planning should state:

- What the module owns
- What it exposes
- What it depends on
- What must remain private

## Reuse and Refactoring Rule

- Do not create abstractions only because something might be reused later.
- If a real reusable pattern appears, refactor to the smallest necessary abstraction.
- The abstraction should remove duplication or clarify ownership, not add indirection for its own sake.
- Refactoring is encouraged when it improves readability, testability, or API clarity.
- Keep the refactor scoped to the real use case that justified it.

## SOLID Principles

All code should follow SOLID principles. In this project that means:

### Single Responsibility Principle

- A module, type, or function should have one clear reason to change.
- Separate rendering, input, simulation, map data, collision, and game rules unless there is a strong reason not to.

### Open/Closed Principle

- Extend behavior through composition, data, or new modules rather than repeatedly editing fragile central logic.
- Prefer designs that allow adding a new weapon, entity behavior, or map rule without rewriting unrelated systems.

### Liskov Substitution Principle

- If traits or polymorphic behavior are introduced, implementations must honor the contract cleanly.
- Do not create interchangeable interfaces if callers still need type-specific exceptions to use them safely.

### Interface Segregation Principle

- Keep interfaces small and specific.
- Do not force a module to depend on methods or data it does not need.
- Prefer several focused traits or APIs over one broad catch-all interface.

### Dependency Inversion Principle

- High-level gameplay logic should depend on stable abstractions where appropriate, not directly on volatile details.
- Keep pure logic isolated from framework-specific rendering or input code when practical.
- Use dependency inversion to improve testability, not as ceremony.

## Testing Expectations

Every step should be testable in two ways.

### 1. Automated tests

- Add or update unit tests where the behavior is stable and isolated.
- Prefer testing pure logic such as map queries, collision checks, ray hit calculations, damage rules, and state transitions.
- Keep tests close to the code they verify.

### 2. Manual QA

- Every completed step must be runnable by a human.
- Manual QA should be short, concrete, and repeatable.
- The result should be visible on screen or directly interactive.

Examples of acceptable manual-QA outcomes:

- A window opens and shows the map.
- The player can move and rotation is visible.
- Rays render wall columns on screen.
- Collision prevents walking through walls.
- A sprite appears and reacts to hits.

## Documentation Maintenance

Project documentation should be updated as part of finishing development work, not as optional cleanup afterward.

This project currently maintains two documentation files for AI collaboration:

- `docs/ai-project-snapshot.md`
  - This is the current-state document.
  - It should describe what exists in the codebase right now.
  - Keep it short, high-signal, and easy for an AI agent to scan quickly.
  - Update it when user-visible behavior, module ownership, important invariants, tests, or known limits change.

- `docs/development-log.md`
  - This is the append-only history document.
  - It should record meaningful development stages after they are completed.
  - Each entry should focus on:
    - the user-visible result
    - the main code or architecture change
    - the scope boundary at the end of that stage

Documentation updates should stay proportional to the size of the change:

- Small internal changes may only require a small snapshot update.
- A visible or structurally meaningful step should usually update both files.
- Do not let documentation become a second implementation task. Prefer short factual updates over long prose.

If code changes make documentation inaccurate, fixing the documentation is part of completing the task.

## Definition of Done for a Step

A task is done only when all of the following are true:

- The planned high-level description and implementation approach were actually followed, or the deviation is documented.
- The feature or behavior exists in code.
- There is an observable result in the running project.
- Manual QA instructions are clear and short.
- Relevant unit tests pass, or the task explicitly documents why a unit test is not appropriate yet.
- Relevant project documentation is updated when the change affects current behavior, ownership boundaries, tests, or known limits.
- The intended APIs or module boundaries are present and coherent.
- The change does not quietly expand project scope.

## Recommended First Steps

Use this order unless there is a strong reason to change it:

1. Create the Rust project and open a window.
2. Represent a fixed 2D grid map and render a top-down debug view.
3. Add player position, movement, and rotation.
4. Cast rays into the grid.
5. Draw vertical wall columns from ray hits.

This sequence is preferred because each step is visible, manually verifiable, and supports focused unit tests.

## Choosing the First Step

Do not lock in the first implementation task until it has been described using the planning template above.

The first step should still satisfy all of the core rules:

- It must produce a noticeable result.
- It must support manual QA.
- It must create a base for focused unit tests.
- Its API and module boundaries should be clear before coding starts.

## Scope Discipline

- Prefer placeholder art, colors, and simple geometry early.
- Do not delay a playable slice for asset polish.
- Do not jump to textured floors, sector rendering, complex AI, or engine architecture before the raycaster prototype works.
- Reconsider bigger-engine choices like Godot only after a working prototype exists.

## AI Collaboration Rule

AI is useful here for:

- Rust scaffolding
- Rendering and math explanations
- Borrow-checker debugging
- Small tool creation
- Placeholder content workflows

AI should not drive:

- Scope decisions
- Game feel
- Level design judgment
- Finishing decisions

Keep the project small, playable, and inspectable at every stage.
