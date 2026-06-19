# Task Template

Use this exact structure when defining a planned task.

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

## Review Checklist

- The high-level description is visible and user-facing.
- The step is small enough to complete and inspect in one pass.
- The implementation approach explains how the code will be structured.
- The definition of done is concrete and checkable.
- Automated tests cover stable logic.
- Manual QA is short and repeatable.
- Expected modules, types, and functions are named.
- API boundaries are explicit.
- The step avoids speculative abstraction.
- The plan is understandable without prior Rust knowledge.
