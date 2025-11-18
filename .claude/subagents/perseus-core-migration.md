# Subagent Task: Perseus Core Migration

## Task Identifier
**ID:** `perseus-core-migration`  
**Priority:** HIGH  
**Complexity:** HIGH  
**Estimated Duration:** 6-8 hours

## Objective
Migrate the `perseus-core` package from Sycamore 0.8 to 0.9, focusing on core framework functionality including templates, state management, and component rendering.

## Scope

### Files in Scope
```
packages/perseus-core/src/
├── template.rs         # HIGH PRIORITY - Template definitions
├── state.rs            # HIGH PRIORITY - State management
├── render.rs           # MEDIUM - Rendering logic
├── component.rs        # MEDIUM - Component helpers
├── error_pages.rs      # LOW - Error handling
├── lib.rs              # LOW - Module exports
└── utils.rs            # LOW - Utility functions
```

### Key Integration Points
- Template system (heavily uses Sycamore components)
- State generation (reactive primitives)
- Server-side rendering (View types)
- Component lifecycle (Scope management)

## Prerequisites
- [ ] Phase 1 analysis complete
- [ ] Phase 2 dependencies updated
- [ ] Git checkpoint created: `migration/pre-perseus-core`
- [ ] Backup created: `packages/perseus-core/.backup/`

## Migration Strategy

### Step 1: Analysis (30 minutes)
Run targeted analysis on perseus-core:
```bash
cd packages/perseus-core
rg "cx: Scope" src/ --stats
rg "<.*G: Html.*>" src/ --stats
rg "create_signal\(cx," src/ --stats
rg "View<G>" src/ --stats
```

Document findings:
- Count of each pattern type
- Most affected files
- Complex lifetime scenarios
- Generic constraint challenges

### Step 2: Template System Migration (2-3 hours)

**File:** `template.rs`

**Key Transformations:**

1. **Template Struct Generic Removal:**
```rust
// BEFORE
pub struct Template<G: Html> {
    path: String,
    template_fn: Box<dyn Fn(Scope, &StateRx) -> View<G>>,
}

// AFTER
pub struct Template {
    path: String,
    template_fn: Box<dyn Fn(&StateRx) -> View>,
}
```

2. **Template Builder Methods:**
```rust
// BEFORE
impl<G: Html> Template<G> {
    pub fn new(path: String) -> Self { ... }
    pub fn template<F>(mut self, f: F) -> Self
    where
        F: Fn(Scope, &StateRx) -> View<G> + 'static,
    { ... }
}

// AFTER
impl Template {
    pub fn new(path: String) -> Self { ... }
    pub fn template<F>(mut self, f: F) -> Self
    where
        F: Fn(&StateRx) -> View + 'static,
    { ... }
}
```

3. **View Functions:**
```rust
// BEFORE
pub fn error_page<G: Html>(cx: Scope, error: &ErrorRx) -> View<G> {
    view! { cx,
        div(class="error") {
            h1 { "Error" }
            p { (error.message.get()) }
        }
    }
}

// AFTER
pub fn error_page(error: &ErrorRx) -> View {
    view! {
        div(class="error") {
            h1 { "Error" }
            p { (error.message.get_clone()) }
        }
    }
}
```

**Validation:**
```bash
cargo check -p perseus-core
cargo test -p perseus-core --lib -- template
```

### Step 3: State Management Migration (2 hours)

**File:** `state.rs`

**Key Transformations:**

1. **State Generator Types:**
```rust
// BEFORE
pub type StateGeneratorFn<S> = 
    Box<dyn Fn(StateGeneratorInfo<'_>) -> BoxFuture<'static, Result<S, Error>>>;

// AFTER - Preserve the lifetime! It's for data, not Scope
pub type StateGeneratorFn<S> = 
    Box<dyn Fn(StateGeneratorInfo<'_>) -> BoxFuture<'static, Result<S, Error>>>;
```

**CRITICAL:** The `'_` lifetime in `StateGeneratorInfo` is for Perseus data structures, NOT Sycamore. Keep it!

2. **Reactive State Handling:**
```rust
// BEFORE
pub fn make_rx<T: Clone + 'static>(cx: Scope, state: T) -> Signal<T> {
    create_signal(cx, state)
}

// AFTER
pub fn make_rx<T: Clone + 'static>(state: T) -> Signal<T> {
    create_signal(state)
}
```

**Validation:**
```bash
cargo check -p perseus-core
cargo test -p perseus-core --lib -- state
```

### Step 4: Rendering Logic Migration (1 hour)

**File:** `render.rs`

**Focus Areas:**
- SSR rendering functions
- Hydration support
- View serialization

**Pattern:**
```rust
// BEFORE
pub fn render_to_string<G: Html>(cx: Scope, view: View<G>) -> String {
    sycamore::render_to_string(|cx| view)
}

// AFTER
pub fn render_to_string(view: View) -> String {
    sycamore::render_to_string(|| view)
}
```

### Step 5: Component Helpers Migration (1 hour)

**File:** `component.rs`

**Transformations:**
- Remove Scope parameters from helper functions
- Update generic constraints
- Fix reactive primitive usage

### Step 6: Low-Priority Files (30 minutes)

**Files:** `error_pages.rs`, `lib.rs`, `utils.rs`

Quick pass to:
- Remove Scope parameters
- Update imports
- Fix any remaining patterns

### Step 7: Comprehensive Testing (1 hour)

```bash
# Unit tests
cargo test -p perseus-core

# Integration tests
cargo test --test integration_tests

# Check for warnings
cargo clippy -p perseus-core -- -D warnings

# Documentation build
cargo doc -p perseus-core --no-deps
```

## Safety Checks

### Before Each File Modification
- [ ] Create file backup: `cp {file} {file}.bak`
- [ ] Review current state: `git diff {file}`
- [ ] Understand all lifetimes (Perseus vs Sycamore)

### After Each File Modification
- [ ] Compile check: `cargo check -p perseus-core`
- [ ] Run relevant tests
- [ ] Review diff for unintended changes
- [ ] Commit if stable: `git add {file} && git commit -m "[Migration] Core: {description}"`

### After Complete Migration
- [ ] Full test suite passes
- [ ] No compilation warnings
- [ ] Documentation builds
- [ ] Examples using core compile
- [ ] Git checkpoint: `migration/post-perseus-core`

## Common Pitfalls

### Pitfall 1: Lifetime Confusion
```rust
// WRONG - Removed Perseus data lifetime
pub struct StateGeneratorInfo<T> {  // Missing lifetime!
    path: &str,  // Error: missing lifetime
}

// CORRECT - Preserved Perseus data lifetime
pub struct StateGeneratorInfo<'a, T> {
    path: &'a str,
}
```

### Pitfall 2: Over-Aggressive Generic Removal
```rust
// WRONG - Removed needed generic
pub struct Template {  // Lost state type information!
    state_fn: Box<dyn Fn() -> ???>,  // What type?
}

// CORRECT - Preserved state generic
pub struct Template<S> {
    state_fn: Box<dyn Fn() -> S>,
}
```

### Pitfall 3: Signal Access for Non-Copy Types
```rust
// WRONG - String is not Copy
let title: String = title_signal.get();

// CORRECT
let title: String = title_signal.get_clone();
```

## Rollback Procedure

If critical issues arise:

```bash
# Option 1: Git reset to checkpoint
git reset --hard migration/pre-perseus-core

# Option 2: Restore from backups
find packages/perseus-core -name "*.bak" -exec sh -c \
  'mv "$1" "${1%.bak}"' _ {} \;

# Option 3: Restore specific file
cp packages/perseus-core/src/template.rs.bak \
   packages/perseus-core/src/template.rs
```

## Success Criteria
- ✅ All files compile without errors
- ✅ All unit tests pass
- ✅ No Sycamore 0.8 patterns remain
- ✅ No new clippy warnings
- ✅ Documentation builds successfully
- ✅ Perseus-specific lifetimes preserved
- ✅ State generic parameters intact
- ✅ Git history clean and well-documented

## Communication

### Progress Updates
Report to main migration process after each major step:
- File completed
- Tests passing
- Issues encountered
- Estimated time to completion

### Issue Escalation
If stuck for >30 minutes on any single issue:
1. Document the problem clearly
2. Create minimal reproduction
3. Search for similar issues in Sycamore/Perseus
4. Ask for guidance from main migration agent

## Deliverables
1. ✅ Migrated `perseus-core` package
2. ✅ Test results report
3. ✅ Migration notes documenting challenges
4. ✅ Git commits with clear messages
5. ✅ Updated internal documentation if API changed

## Next Steps After Completion
1. Create comprehensive migration report
2. Move to next package: `perseus-macro`
3. Update migration progress tracker
4. Share learnings with team

---

**Subagent Instructions:**
- Work systematically file-by-file
- Test frequently
- Commit often
- Document everything unusual
- Ask for help when needed
- Celebrate small wins! 🎉
