# Skill: Update Component Signature

## Purpose

Migrate Sycamore 0.8 component function signatures to 0.9.2 format by removing scope parameters, lifetimes, and generic constraints.

## Inputs

- File path containing component definitions
- Optional: Specific function name to target

## Process

### Step 1: Identify Components

Look for functions with these characteristics:

- Has `#[component]` attribute
- Contains `cx: Scope` parameter
- Has `<G: Html>` generic
- Has `'a` lifetime

### Step 2: Transform Signature

**Before Pattern:**

```rust
#[component]
fn MyComponent<'a, G: Html>(cx: Scope<'a>, props: MyProps) -> View<G> {
    ...
}
```

**After Pattern:**

```rust
#[component]
fn MyComponent(props: MyProps) -> View {
    ...
}
```

### Step 3: Update Function Body

Remove `cx` references:

- `view! { cx,` → `view! {`
- `create_signal(cx,` → `create_signal(`
- `create_effect(cx,` → `create_effect(`
- `create_memo(cx,` → `create_memo(`

### Step 4: Preserve Perseus Lifetimes

**DO NOT REMOVE** lifetimes that are Perseus-specific:

```rust
// KEEP THIS - Perseus data structure
pub struct StateGeneratorInfo<'a, T> {
    path: &'a str,
    locale: &'a str,
    extra: T,
}

// REMOVE THIS - Sycamore component
fn old_component<'a, G: Html>(cx: Scope<'a>) -> View<G> { ... }
```

**Decision Rules:**

1. If lifetime is on `Scope` → REMOVE
2. If lifetime is on data reference → EVALUATE
3. If lifetime is on Perseus type → KEEP

## Safety Checks

Before making changes, verify:

- [ ] Function is a Sycamore component (has `#[component]`)
- [ ] Not a trait method (trait methods have different rules)
- [ ] Not using lifetime for non-Sycamore purposes
- [ ] Check for caller sites that might break

## Example Transformations

### Example 1: Simple Component

```rust
// BEFORE
#[component]
fn Counter<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let count = create_signal(cx, 0);

    view! { cx,
        button(on:click=|_| count.set(*count.get() + 1)) {
            "Count: " (count.get())
        }
    }
}

// AFTER
#[component]
fn Counter() -> View {
    let count = create_signal(0);

    view! {
        button(on:click=|_| count.set(*count.get() + 1)) {
            "Count: " (count.get())
        }
    }
}
```

### Example 2: Component with Props

```rust
// BEFORE
#[component(inline_props)]
fn Greeter<'a, G: Html>(cx: Scope<'a>, name: &'a str) -> View<G> {
    view! { cx,
        p { "Hello, " (name) "!" }
    }
}

// AFTER
#[component(inline_props)]
fn Greeter(name: String) -> View {
    view! {
        p { "Hello, " (name) "!" }
    }
}
```

Note: Changed `&'a str` to `String` as lifetimes are now 'static.

### Example 3: Preserve Perseus Types

```rust
// BEFORE - Perseus engine function
async fn get_build_state<'a>(
    info: StateGeneratorInfo<'a, ()>
) -> Result<MyState, Error> {
    // Use info.path, info.locale
}

// AFTER - Keep the lifetime, just remove Sycamore parts if any
async fn get_build_state<'a>(
    info: StateGeneratorInfo<'a, ()>
) -> Result<MyState, Error> {
    // Same - this lifetime is for Perseus, not Sycamore
}
```

## Automated Find & Replace

Use these regex patterns carefully:

```bash
# Pattern 1: Remove Scope parameter
# SEARCH: , cx: Scope<'[a-z]>
# REPLACE: (empty)

# Pattern 2: Remove lifetime and Html generic
# SEARCH: <'[a-z], G: Html>
# REPLACE: (empty)

# Pattern 3: Update View return type
# SEARCH: -> View<G>
# REPLACE: -> View

# Pattern 4: Remove cx from view! macro
# SEARCH: view! { cx,
# REPLACE: view! {
```

⚠️ **WARNING:** These are starter patterns. Manual review required for each change!

## Validation

After transformation:

```bash
# Compile the specific file's module
cargo check -p {package-name}

# Run tests if available
cargo test -p {package-name} {function-name}

# Check for remaining issues
rg "cx: Scope" {file}
rg "<.*G: Html.*>" {file}
```

## Error Handling

### Common Errors After Transformation

**Error:** `cannot find value 'cx' in this scope`

```rust
// Still has: some_function(cx, args)
// Fix: some_function(args)
```

**Error:** `expected 0 lifetime parameters`

```rust
// Still has: &'a ReadSignal<T>
// Fix: ReadSignal<T>  (signals are Copy now)
```

**Error:** `cannot infer type for type parameter 'G'`

```rust
// Still has: View<G> somewhere
// Fix: View
```

## Output Format

Generate a migration report:

```markdown
## Component Signature Update: {filename}

### Transformations Applied: {count}

#### Function: {function_name}

- **Before:** `{old_signature}`
- **After:** `{new_signature}`
- **Status:** ✅ Success / ⚠️ Review Needed / ❌ Error
- **Notes:** {any special considerations}

### Validation Results

- Compilation: ✅/❌
- Tests: ✅/❌
- Manual Review Needed: Yes/No

### Next Steps

- [ ] {action item}
```

## Usage Examples

```bash
# Update single file
claude-code skill update-component-signature src/components/header.rs

# Update all component files
find src/components -name "*.rs" -exec \
  claude-code skill update-component-signature {} \;

# Update with dry-run (show changes without applying)
claude-code skill update-component-signature --dry-run src/template.rs
```

## Rollback

If something goes wrong:

```bash
# Git restore the file
git restore {file}

# Or restore from backup (created automatically)
cp {file}.backup {file}
```

## Success Criteria

- All component signatures updated to 0.9.2 format
- No `cx: Scope` parameters remain
- No `<G: Html>` generics remain (except in type definitions)
- Code compiles without errors
- Tests pass
- Perseus-specific lifetimes preserved
