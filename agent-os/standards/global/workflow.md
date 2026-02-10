# Development Workflow

## TDD Cycle

Red-Green-Refactor:
1. **Red** - Write failing test first
2. **Green** - Write minimal code to pass
3. **Refactor** - Clean up, maintain tests passing

## Test Policy

**Blocking requirement**: Never commit failing tests.

```bash
# Before every commit:
cargo test
# Only commit if all tests pass
```

## Trunk-Based Development

- Work on `master` branch (or short-lived feature branches)
- Integrate early and often
- Small, incremental commits
- Keep main branch always deployable

## Commit Message Style

Descriptive imperative with phase/scope prefix:
```
Phase 0.5: Minimal viable Vulkan layer - POC complete
Update .gitignore for local planning files
Initial commit: Goopinator - ARC Raiders Tactical Overlay POC
```

Pattern: `[Phase N:] <imperative verb> <what> [- context]`

## Implementation Approach

- Create each component as simply as possible
- Integrate early
- Use standard patterns
- Test coverage for all code
