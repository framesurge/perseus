# Perseus Framework Migration Guide
## Project Context for Claude Code CLI

**Migration Objective:** Upgrade Perseus web framework from Sycamore 0.8 to Sycamore 0.9

**Repository:** https://github.com/afidegnum/perseus (fork of framesurge/perseus)  
**Working Branch:** `update-v0.5`  
**Timeline:** 10-14 day comprehensive migration

---

## Perseus Architecture Overview

### What is Perseus?

Perseus is a **state-driven web development framework** for Rust, analogous to Next.js for React. It provides:

- **Server-Side Rendering (SSR)** - Dynamic content generation
- **Static Site Generation (SSG)** - Build-time page rendering
- **Incremental Regeneration** - On-demand page building
- **Revalidation** - Timed or logic-based content updates
- **Hot State Reloading** - Development-time state persistence

### Core Architecture

```
Perseus Framework
├── perseus-core/          # Core framework logic
├── perseus-macro/         # Procedural macros for components
├── perseus-engine/        # Build-time engine
├── perseus-router/        # Client-side routing
├── perseus-warp/          # Warp server integration
├── perseus-axum/          # Axum server integration
└── perseus-cli/           # Command-line interface
```

### Sycamore Integration Points

Perseus **heavily depends** on Sycamore for:

1. **View Layer** - All UI components use Sycamore's `view!` macro
2. **Reactivity System** - State management via Sycamore signals
3. **Component Model** - `#[component]` macro from Sycamore
4. **SSR Support** - Server-side rendering capabilities
5. **Hydration** - Client-side takeover of server-rendered HTML

**Critical Constraint:** Perseus must maintain its own public API stability while adapting to Sycamore's internal changes.

---

## Sycamore 0.8 → 0.9 Breaking Changes

### 1. Reactivity v3: Scope Removal

**Most Impactful Change** - Affects ~90% of codebase

**Before (0.8):**
```rust
#[component]
fn MyComponent<'a, G: Html>(cx: Scope<'a>, props: MyProps) -> View<G> {
    let signal = create_signal(cx, 123);
    create_effect(cx, || {
        // effect logic
    });
    view! { cx,
        div { "Hello" }
    }
}
```

**After (0.9):**
```rust
#[component]
fn MyComponent(props: MyProps) -> View {
    let signal = create_signal(123);
    create_effect(|| {
        // effect logic
    });
    view! {
        div { "Hello" }
    }
}
```

**Key Changes:**
- Remove `cx: Scope` parameter from ALL functions
- Remove `<'a>` lifetime from component signatures
- Remove `cx,` from `view!` macro calls
- Remove `, cx` from reactive primitive calls
- Signals are now `'static` and `Copy`

### 2. View v2: Generic Elimination

**Before (0.8):**
```rust
fn component<G: Html>(cx: Scope) -> View<G> { ... }
```

**After (0.9):**
```rust
fn component() -> View { ... }
```

**Key Changes:**
- Remove `<G: Html>` generic parameter
- Remove `<'a, G: Html>` combined patterns
- Change `View<G>` to just `View`
- Target detection now automatic (wasm32 = DOM, other = SSR)

### 3. Signal API Updates

**Before (0.8):**
```rust
let signal = create_signal(cx, value);
let rc_signal = create_rc_signal(value);  // for 'static signals
let value = signal.get();  // Copy types
let value = signal.get().clone();  // Non-Copy types
```

**After (0.9):**
```rust
let signal = create_signal(value);  // Always 'static now
// No more RcSignal needed
let value = signal.get();  // Copy types
let value = signal.get_clone();  // Non-Copy types
```

**Key Changes:**
- `RcSignal` removed - use `Signal` everywhere
- `create_rc_signal` → `create_signal`
- `.get().clone()` → `.get_clone()` for non-Copy types
- All signals are `'static` and `Copy` by default

### 4. View Macro Syntax Changes

**Indexed/Keyed Lists:**
```rust
// Before
Indexed(iterable=list, view=|item| ...)

// After  
Indexed(list=list, view=|item| ...)
```

**Rust Keywords:**
```rust
// Before
ref=node_ref, type="button"

// After
r#ref=node_ref, r#type="button"
```

**Signal Interpolation:**
```rust
// Before
view! { cx, div { (signal.get()) } }

// After
view! { div { (signal) } }  // Signals auto-convert to views
```

### 5. Builder API Changes

**Before (0.8):**
```rust
div()
    .c(h1().t("Hello"))
    .bind_value(signal)
    .view()
```

**After (0.9):**
```rust
div()
    .children(h1().children("Hello"))
    .bind(bind::value, signal)
    .into()
```

**Key Changes:**
- `.c()` → `.children()`
- `.t()` → `.children()` for text
- `.view()` → `.into()`
- `bind_value()` → `bind(bind::value, ...)`
- All attributes now type-checked

---

## Perseus-Specific Considerations

### Lifetime Preservation

⚠️ **CRITICAL:** Some lifetimes in Perseus are **Perseus-specific**, not Sycamore-specific.

**Example - DO NOT REMOVE:**
```rust
// Perseus state management requires this lifetime
pub struct StateGeneratorInfo<'a, T> {
    path: &'a str,
    locale: &'a str,
    extra: T,
}
```

**Decision Tree:**
1. Is the lifetime tied to `Scope`? → **REMOVE IT**
2. Is the lifetime for Perseus data structures? → **KEEP IT**
3. Is it a function parameter lifetime? → **EVALUATE CAREFULLY**

### Generic Parameter Preservation

Perseus uses generics for:

1. **State Type Parameters** - Keep these:
   ```rust
   pub struct Template<T> { ... }
   ```

2. **Error Type Parameters** - Keep these:
   ```rust
   pub type Result<T, E = ServerError> = std::result::Result<T, E>;
   ```

3. **Sycamore View Generics** - Remove these:
   ```rust
   // Remove <G: Html> - it's Sycamore-specific
   ```

### Macro Invocations

Perseus has several procedural macros that may be affected:

1. `#[perseus::main]` - Engine entry point
2. `#[engine_only_fn]` - Server-only functions
3. `#[browser_only_fn]` - Client-only functions
4. `#[component]` - From Sycamore, needs updating

---

## Module-by-Module Risk Assessment

### High Risk (Complex Integration)

**perseus-core/src/template.rs**
- Heavy Sycamore integration
- Many component signatures
- Complex generic constraints
- **Estimated effort:** 6-8 hours

**perseus-core/src/state.rs**
- Reactive state management
- Signal handling throughout
- **Estimated effort:** 4-6 hours

**perseus-router/src/lib.rs**
- Client-side routing with reactivity
- Component rendering logic
- **Estimated effort:** 4-6 hours

### Medium Risk (Moderate Changes)

**perseus-macro/src/lib.rs**
- Macro code generation
- May need output adjustment
- **Estimated effort:** 3-4 hours

**perseus-engine/**
- Build-time rendering
- Server-side execution
- **Estimated effort:** 3-5 hours

### Low Risk (Minimal Changes)

**perseus-cli/**
- CLI tooling
- Minimal Sycamore usage
- **Estimated effort:** 1-2 hours

**Server integrations (warp/axum)**
- HTTP server logic
- Limited view code
- **Estimated effort:** 2-3 hours each

---

## Testing Strategy

### Unit Tests
```bash
# Run all workspace tests
cargo test --workspace --all-features

# Test specific package
cargo test -p perseus-core
```

### Integration Tests
```bash
# Build all examples
cargo build --examples --workspace

# Run specific example
cd examples/basic && perseus serve
```

### Validation Checklist

- [ ] All workspace members compile (`cargo check --workspace`)
- [ ] No compilation errors
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Examples build and run
- [ ] No Sycamore 0.8 dependencies remain
- [ ] No deprecation warnings from Sycamore
- [ ] Perseus public API unchanged (or documented)

---

## Common Patterns & Solutions

### Pattern 1: Component Function Signatures

**Search for:**
```regex
fn\s+\w+<.*G:\s*Html.*>\s*\(.*cx:\s*Scope
```

**Replace with:**
```rust
// Remove: <'a, G: Html>, cx: Scope parameters
// Keep: Other generics, other parameters
```

### Pattern 2: View Macro Calls

**Search for:**
```regex
view!\s*{\s*cx,
```

**Replace with:**
```rust
view! {
```

### Pattern 3: Signal Creation

**Search for:**
```regex
create_signal\(cx,\s*(.+)\)
create_rc_signal\((.+)\)
```

**Replace with:**
```rust
create_signal($1)
create_signal($1)
```

### Pattern 4: Reactive Effects

**Search for:**
```regex
create_effect\(cx,\s*\|\|
create_memo\(cx,\s*\|\|
```

**Replace with:**
```rust
create_effect(||
create_memo(||
```

---

## Git Workflow Strategy

### Branch Structure
```
main (or update-v0.5)
├── migration/phase-1-analysis
├── migration/phase-2-dependencies
├── migration/phase-3-core
├── migration/phase-4-macros
├── migration/phase-5-router
├── migration/phase-6-engines
├── migration/phase-7-servers
├── migration/phase-8-examples
└── migration/phase-9-testing
```

### Commit Message Convention
```
[Migration] Category: Brief description

- Detailed change 1
- Detailed change 2

Refs: #issue-number
```

**Categories:**
- `Scope` - Scope parameter removal
- `Generics` - Generic type parameter updates
- `Signals` - Signal API changes
- `View` - View macro updates
- `Builder` - Builder API changes
- `Tests` - Test updates
- `Deps` - Dependency updates

---

## MCP Server Integration

This migration uses **Model Context Protocol (MCP)** servers for enhanced automation:

### 1. Filesystem MCP
- Intelligent file analysis
- Batch modification capabilities
- Pattern recognition across files

### 2. Git MCP
- Automated commit creation
- Branch management
- Change tracking

### 3. GitHub MCP
- Documentation quick access
- Issue tracking
- PR management

### 4. Brave Search MCP
- Automatic solution finding
- Error message lookup
- Best practice research

### 5. Sequential Thinking MCP
- Complex problem decomposition
- Migration strategy planning
- Refactoring impact analysis

---

## Skills Available

The following Claude Code skills are defined for this migration:

1. **analyze-sycamore-usage** - Find all Sycamore API calls
2. **update-component-signature** - Remove scope and generics
3. **migrate-signals** - Update signal API calls
4. **update-view-macro** - Fix view! macro syntax
5. **test-compilation** - Compile and report errors
6. **validate-migration** - Run comprehensive tests

See `skills/` directory for detailed implementations.

---

## Subagent Tasks

Complex modules are handled by specialized subagents:

1. **perseus-core-migration** - Core framework changes
2. **perseus-router-migration** - Router updates
3. **perseus-macro-migration** - Macro adjustments
4. **perseus-engine-migration** - Engine updates
5. **testing-validation** - Comprehensive testing

See `subagents/` directory for task definitions.

---

## Automation Scripts

Shell scripts for common operations:

- `scripts/analyze-codebase.sh` - Initial analysis
- `scripts/update-dependencies.sh` - Cargo.toml updates
- `scripts/batch-replace.sh` - Pattern replacements
- `scripts/run-tests.sh` - Test execution
- `scripts/validate-migration.sh` - Final validation
- `scripts/create-checkpoint.sh` - Git checkpoints

---

## Quick Reference

### Essential Commands

```bash
# Analysis
cargo tree -p perseus | grep sycamore
rg "cx: Scope" --type rust
rg "<.*G: Html.*>" --type rust

# Compilation
cargo check --workspace
cargo clippy --workspace

# Testing
cargo test --workspace --all-features
cargo test -p perseus-core

# Cleanup
cargo clean
cargo update
```

### Key Files to Monitor

- `Cargo.toml` (workspace root)
- `packages/*/Cargo.toml` (member crates)
- `packages/perseus-core/src/template.rs`
- `packages/perseus-core/src/state.rs`
- `packages/perseus-router/src/lib.rs`
- `packages/perseus-macro/src/lib.rs`

---

## Troubleshooting

### Common Errors

**Error:** `cannot find value 'cx' in this scope`
- **Cause:** Removed `cx: Scope` but forgot to update usage
- **Fix:** Remove `cx` from function calls, macro invocations

**Error:** `expected 1 lifetime parameter`
- **Cause:** Removed lifetime but type still expects it
- **Fix:** Check if lifetime is Perseus-specific, not Sycamore-specific

**Error:** `cannot infer type for type parameter 'G'`
- **Cause:** Removed `<G: Html>` but still using `View<G>`
- **Fix:** Change `View<G>` to `View`

**Error:** `no method named 'get' found for type 'Signal<String>'`
- **Cause:** Non-Copy type needs `.get_clone()`
- **Fix:** Change `.get()` to `.get_clone()` for non-Copy types

---

## Success Metrics

- ✅ Zero compilation errors across all workspace members
- ✅ 100% test pass rate
- ✅ All examples functional
- ✅ No Sycamore 0.8 in dependency tree
- ✅ Zero Sycamore-related deprecation warnings
- ✅ Documentation updated
- ✅ Migration guide created for Perseus users
- ✅ Performance benchmarks maintained or improved

---

## Resources

- [Sycamore Migration Guide](https://sycamore.dev/book/migration/0-8-to-0-9)
- [Sycamore v0.9 Announcement](https://sycamore.dev/post/announcing-v0-9-0)
- [Perseus Documentation](https://framesurge.sh/perseus/en-US)
- [Perseus Repository](https://github.com/afidegnum/perseus)
- [Sycamore API Docs](https://docs.rs/sycamore/latest/sycamore/)

---

**Last Updated:** Migration Workflow Package v1.0  
**For:** Claude Code CLI with MCP Integration
