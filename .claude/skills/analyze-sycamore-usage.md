# Skill: Analyze Sycamore Usage

## Purpose

Scan a Rust file and identify all Sycamore 0.8 API usage patterns that need migration to 0.9.2.

## Inputs

- File path to analyze

## Process

1. **Read the file** and scan for these patterns:

### Pattern Detection

```bash
# Scope parameters in functions
rg "cx:\s*Scope" "$file"

# Generic Html constraints
rg "<.*G:\s*Html.*>" "$file"

# View generic types
rg "View<G>" "$file"

# Lifetime parameters with Scope
rg "<'[a-z],.*Scope" "$file"

# Signal creation with cx
rg "create_signal\(cx," "$file"
rg "create_rc_signal\(" "$file"

# Effect creation with cx
rg "create_effect\(cx," "$file"
rg "create_memo\(cx," "$file"

# View macro with cx
rg "view!\s*{\s*cx," "$file"

# Indexed/Keyed iterable
rg "iterable\s*=" "$file"

# Old attribute syntax
rg "\bref\s*=" "$file"
rg "\btype\s*=" "$file"
```

## Output Format

Generate a structured report:

```markdown
## Sycamore 0.8 → 0.9.2 Migration Analysis: {filename}

### Scope Parameters Found: {count}

- Line {num}: `{code snippet}`
- ...

### Generic Html Constraints: {count}

- Line {num}: `{code snippet}`
- ...

### Signal API Calls: {count}

- Line {num}: `{code snippet}`
- ...

### View Macro Calls: {count}

- Line {num}: `{code snippet}`
- ...

### Risk Assessment

- **Complexity:** Low/Medium/High
- **Estimated Effort:** {hours} hours
- **Dependencies:** {files that import this one}

### Recommended Actions

1. {action item}
2. {action item}
```

## Example Usage

```bash
# Analyze single file
claude-code skill analyze-sycamore-usage packages/perseus-core/src/template.rs

# Analyze all Rust files in a directory
find packages/perseus-core/src -name "*.rs" -exec \
  claude-code skill analyze-sycamore-usage {} \;
```

## Success Criteria

- All Sycamore 0.8 patterns identified
- Line numbers provided for each match
- Complexity assessment generated
- No false positives in safe patterns
