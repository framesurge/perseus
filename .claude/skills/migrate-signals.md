# Skill: Migrate Signals

## Purpose
Update all Sycamore 0.8 signal API calls to the 0.9 Signal API, including creation, access, and RcSignal replacement.

## Inputs
- File path containing signal usage
- Optional: Aggressive mode for bulk replacements

## Key Changes Overview

### 1. Signal Creation
```rust
// OLD (0.8)
let signal = create_signal(cx, initial_value);
let rc_signal = create_rc_signal(initial_value);

// NEW (0.9)
let signal = create_signal(initial_value);
let signal = create_signal(initial_value);  // No more RcSignal
```

### 2. Signal Access
```rust
// OLD (0.8) - Copy types
let value = signal.get();

// OLD (0.8) - Non-Copy types  
let value = signal.get().clone();
let value = (*signal.get()).clone();

// NEW (0.9) - Copy types
let value = signal.get();

// NEW (0.9) - Non-Copy types
let value = signal.get_clone();
```

### 3. RcSignal Elimination
```rust
// OLD (0.8)
use sycamore::reactive::RcSignal;
let signal: RcSignal<String> = create_rc_signal("hello".to_string());

// NEW (0.9)
use sycamore::reactive::Signal;
let signal: Signal<String> = create_signal("hello".to_string());
```

## Migration Process

### Step 1: Remove `cx` from Signal Creation

**Pattern:**
```regex
create_signal\(cx,\s*(.+?)\)
```

**Replacement:**
```rust
create_signal($1)
```

### Step 2: Replace RcSignal with Signal

**Imports:**
```rust
// Remove
use sycamore::reactive::RcSignal;

// Ensure present
use sycamore::reactive::Signal;
```

**Type Annotations:**
```regex
RcSignal<(.+?)>
```
Replace with:
```rust
Signal<$1>
```

**Creation:**
```regex
create_rc_signal\((.+?)\)
```
Replace with:
```rust
create_signal($1)
```

### Step 3: Update Signal Access for Non-Copy Types

**Identify Non-Copy Types:**
Common non-Copy types in Perseus:
- `String`
- `Vec<T>`
- `HashMap<K, V>`
- Custom structs without `Copy`
- `Box<T>`

**Pattern to Find:**
```regex
signal\.get\(\)\.clone\(\)
\(\*signal\.get\(\)\)\.clone\(\)
```

**Replace with:**
```rust
signal.get_clone()
```

### Step 4: Update Derived Signals

**create_memo:**
```rust
// OLD
let derived = create_memo(cx, || signal.get() * 2);

// NEW
let derived = create_memo(|| signal.get() * 2);
```

**create_selector:**
```rust
// OLD
let selected = create_selector(cx, || signal.get());

// NEW
let selected = create_selector(|| signal.get());
```

### Step 5: Update Effects with Signals

**create_effect:**
```rust
// OLD
create_effect(cx, || {
    let value = signal.get().clone();
    do_something(value);
});

// NEW
create_effect(|| {
    let value = signal.get_clone();
    do_something(value);
});
```

## Type-Specific Handling

### String Signals
```rust
// OLD
let name: &'a ReadSignal<String> = create_signal(cx, String::new());
let value = name.get().clone();

// NEW
let name: Signal<String> = create_signal(String::new());
let value = name.get_clone();
```

### Numeric Signals (Copy Types)
```rust
// OLD
let count = create_signal(cx, 0i32);
let value = *count.get();  // or count.get()

// NEW
let count = create_signal(0i32);
let value = count.get();  // No deref needed
```

### Complex State Signals
```rust
// OLD
let state: RcSignal<AppState> = create_rc_signal(AppState::default());
let current = state.get().clone();

// NEW
let state: Signal<AppState> = create_signal(AppState::default());
let current = state.get_clone();
```

## Perseus-Specific Patterns

### Template State
```rust
// OLD
fn template<'a, G: Html>(cx: Scope<'a>, state: &'a StateRx) -> View<G> {
    let title = state.title.get().clone();
    ...
}

// NEW
fn template(state: &StateRx) -> View {
    let title = state.title.get_clone();
    ...
}
```

### Global State
```rust
// Perseus global state is typically RcSignal in 0.8
// OLD
use perseus::state::GlobalStateCreator;
let global = GlobalStateCreator::new().build();
let state: RcSignal<AppState> = create_rc_signal(AppState::default());

// NEW - Now just use Signal
let state: Signal<AppState> = create_signal(AppState::default());
```

## Automated Replacement Script

```bash
#!/bin/bash
# Safe signal migration with backups

FILE="$1"
BACKUP="${FILE}.backup"

# Create backup
cp "$FILE" "$BACKUP"

# 1. Remove cx from create_signal
sed -i 's/create_signal(cx,\s*/create_signal(/g' "$FILE"

# 2. Replace RcSignal type annotations
sed -i 's/RcSignal</Signal</g' "$FILE"

# 3. Replace create_rc_signal
sed -i 's/create_rc_signal(/create_signal(/g' "$FILE"

# 4. Replace .get().clone() with .get_clone()
sed -i 's/\.get()\.clone()/\.get_clone()/g' "$FILE"
sed -i 's/(\*\([a-z_]*\)\.get())\.clone()/\1.get_clone()/g' "$FILE"

# 5. Remove RcSignal imports
sed -i '/use.*RcSignal/d' "$FILE"

# Verify changes compile
if cargo check -p $(basename $(dirname "$FILE")) 2>/dev/null; then
    echo "✅ Migration successful for $FILE"
    rm "$BACKUP"
else
    echo "❌ Compilation failed, restoring backup"
    mv "$BACKUP" "$FILE"
fi
```

## Manual Review Checklist

For each signal migration:

- [ ] `create_signal(cx,` removed
- [ ] `create_rc_signal` replaced with `create_signal`
- [ ] `RcSignal` type annotations changed to `Signal`
- [ ] `.get().clone()` changed to `.get_clone()` for non-Copy types
- [ ] Copy types still use `.get()` without clone
- [ ] All reactive effects updated (no `cx` parameter)
- [ ] Imports updated (no `RcSignal`)
- [ ] Code compiles
- [ ] Tests pass
- [ ] No lifetime errors

## Validation

```bash
# Check for remaining old patterns
rg "create_signal\(cx," "$FILE"
rg "create_rc_signal" "$FILE"
rg "RcSignal" "$FILE"
rg "\.get\(\)\.clone\(\)" "$FILE"

# Compile check
cargo check -p {package}

# Test
cargo test -p {package}
```

## Common Pitfalls

### Pitfall 1: Missing get_clone()
```rust
// WRONG - Won't compile for non-Copy types
let name: String = signal.get();

// CORRECT
let name: String = signal.get_clone();
```

### Pitfall 2: Unnecessary get_clone()
```rust
// WRONG - Inefficient for Copy types
let count: i32 = signal.get_clone();  // i32 is Copy!

// CORRECT
let count: i32 = signal.get();
```

### Pitfall 3: Lifetime Confusion
```rust
// WRONG - Signals are 'static now
let signal: &'a Signal<T> = ...;

// CORRECT
let signal: Signal<T> = ...;  // Just Signal, no reference
```

## Output Report

```markdown
## Signal Migration: {filename}

### Statistics
- `create_signal(cx,` calls updated: {count}
- `RcSignal` → `Signal` conversions: {count}
- `.get().clone()` → `.get_clone()`: {count}
- Import statements updated: {count}

### Conversions
#### Line {num}: {original} → {updated}
...

### Validation
- Compilation: ✅/❌
- Tests: ✅/❌
- Remaining old patterns: {count}

### Manual Review Required
- [ ] Verify Copy vs non-Copy type handling
- [ ] Check complex signal compositions
- [ ] Validate reactive effect closures
```

## Usage

```bash
# Migrate single file
claude-code skill migrate-signals src/state.rs

# Migrate with verification
claude-code skill migrate-signals --verify src/state.rs

# Batch migration
find src -name "*.rs" -exec claude-code skill migrate-signals {} \;
```

## Rollback

```bash
# Restore from backup
git restore {file}

# Or manual backup
cp {file}.backup {file}
```

## Success Criteria
- All signal creations use new API (no `cx` parameter)
- No `RcSignal` usage remains
- All non-Copy types use `.get_clone()`
- All Copy types use `.get()`
- Code compiles without errors
- All tests pass
- No deprecation warnings
