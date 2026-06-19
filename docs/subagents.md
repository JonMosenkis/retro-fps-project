# Subagents

This file defines optional reviewer roles for focused planning and review work.

Use them when they reduce risk, not by default on every task.

## Architecture Scout

### When to use it

- Before adding a new gameplay subsystem such as doors, sprites, pickups, enemies, map loading, or save data
- When a task changes module ownership or API boundaries
- When the main question is "where should this logic live?"

### When not to use it

- Small local refactors
- Pure bug fixes that do not change structure
- Cosmetic-only changes

### Prompt

```md
Spawn a read-only architecture scout.

Inspect the current implementation and answer:
1. Where should <feature> state live?
2. Which existing modules would need to change?
3. Which logic can remain pure and unit-testable?
4. What is the smallest visible vertical slice?
5. What accidental complexity should we avoid?

Do not edit files. Return a concise recommendation with file references.
```

### Expected output

- Recommendation
- Files to change
- Pure or testable logic to isolate
- Smallest visible slice
- Complexity to avoid

## Rust Pedagogy Reviewer

### When to use it

- When a plan introduces a new Rust concept that may be harder than necessary
- When an implementation uses ownership, borrowing, iterators, or abstractions in a non-obvious way
- Before merging code that works but may be harder for a Rust beginner to maintain

### When not to use it

- Simple data-only additions
- Trivial renames or formatting-only changes

### Prompt

```md
Spawn a reviewer focused on Rust clarity.

Review the proposed implementation for:
- ownership or borrowing that is harder than necessary,
- overly clever iterators or abstractions,
- unnecessary traits, generics, or indirection,
- places where a beginner-friendly representation would be clearer.

Do not modify the code. Explain concrete alternatives with file references.
```

### Expected output

- Findings ordered by impact
- Why each point is harder than necessary
- Simpler concrete alternatives
- File references

## Correctness Reviewer

### When to use it

- For raycasting, collision, projection, angle math, fixed-step simulation, bounds checks, or deterministic state changes
- When a task adds branch-heavy logic or edge-case-heavy behavior
- Before calling a math-heavy task complete

### When not to use it

- Pure documentation changes
- Small presentation-only adjustments with no logic changes

### Prompt

```md
Spawn a read-only correctness reviewer.

Inspect the current implementation and review it for:
- edge cases and boundary conditions,
- divide-by-zero or invalid math risks,
- out-of-bounds behavior,
- state transitions that may break determinism,
- missing or weak unit tests for stable logic.

Do not edit files. Return concise findings with severity, reasoning, and file references.
Distinguish likely bugs from lower-confidence edge cases.
```

### Expected output

- Findings ordered by severity
- Why each issue matters
- Suggested missing tests
- File references

## Playability Observer

### When to use it

- For movement, controls, camera behavior, rendering readability, or feedback changes
- When the visible result may technically work but still be hard to read or verify
- When defining or reviewing manual QA for a user-facing gameplay change

### When not to use it

- Backend-only refactors
- Internal code cleanup with no visible behavior change

### Prompt

```md
Spawn a reviewer focused on observable playability.

Review the proposed implementation for:
- input responsiveness that may feel delayed or inconsistent,
- movement or rotation tuning that may be hard to read during play,
- visual feedback that may be too subtle to notice,
- manual QA steps that may be unclear or too broad,
- user-visible changes that are too small for a good vertical slice.

Do not modify the code. Focus on concrete observable issues and improvements.
Return a concise recommendation with file references when relevant.
```

### Expected output

- Observable issues
- Why they matter in manual play
- Concrete QA or tuning suggestions
- File references when applicable
