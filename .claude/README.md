# Perseus Sycamore 0.8 → 0.9 Migration Workflow

**Production-Ready Migration Package for Claude Code CLI**

## Overview

This package provides a comprehensive, structured workflow for migrating the Perseus web framework from Sycamore 0.8 to 0.9. Designed specifically for Claude Code CLI with MCP (Model Context Protocol) integration, it includes automated analysis, batch processing, safety mechanisms, and detailed documentation.

### What's Included

```
perseus-migration-workflow/
├── CLAUDE.md                          # Comprehensive project context
├── README.md                          # This file
│
├── scripts/                           # Automation scripts
│   ├── analyze-codebase.sh           # Phase 1: Initial analysis
│   ├── update-dependencies.sh        # Phase 2: Dependency updates
│   ├── batch-replace.sh              # Phase 3: Pattern replacements
│   ├── run-tests.sh                  # Testing automation
│   ├── validate-migration.sh         # Final validation
│   └── create-checkpoint.sh          # Git checkpoint creation
│
├── skills/                            # Claude Code skills
│   ├── analyze-sycamore-usage.md     # Pattern detection
│   ├── update-component-signature.md # Signature transformation
│   ├── migrate-signals.md            # Signal API migration
│   ├── update-view-macro.md          # View macro updates
│   └── test-compilation.md           # Compilation verification
│
├── mcp-configs/                       # MCP server configurations
│   └── claude-code-config.json       # Complete MCP setup
│
├── subagents/                         # Specialized task definitions
│   ├── perseus-core-migration.md     # Core package migration
│   ├── perseus-router-migration.md   # Router migration
│   ├── perseus-macro-migration.md    # Macro package migration
│   └── testing-validation.md         # Testing subagent
│
└── docs/                              # Additional documentation
    ├── breaking-changes.md           # Detailed breaking changes list
    ├── troubleshooting.md            # Common issues and solutions
    └── timeline.md                   # Suggested schedule
```

## Quick Start

### 1. Setup

```bash
# Clone Perseus repository
git clone https://github.com/afidegnum/perseus.git
cd perseus

git checkout update-v0.5

# Make scripts executable
chmod +x .claude/scripts/*.sh
```

### 2. Configure MCP Servers

Edit `.claude/mcp-configs/claude-code-config.json`:

```json
{
    "mcpServers": {
        "github": {
            "env": {
                "GITHUB_PERSONAL_ACCESS_TOKEN": "your-token-here"
            }
        },
        "brave-search": {
            "env": {
                "BRAVE_API_KEY": "your-api-key-here"
            }
        }
    }
}
```

Then configure Claude Code CLI:

```bash
# Copy MCP config to Claude Code directory
cp .claude/mcp-configs/claude-code-config.json \
   ~/.config/claude-code/mcp-config.json
```

### 3. Run Phase 1: Analysis

```bash
./claude/scripts/analyze-codebase.sh
```

This generates a comprehensive report in `migration-reports/analysis_*.md`

### 4. Review & Plan

Read the analysis report to understand:

- Total scope of changes
- Most affected files
- Risk assessment
- Time estimates

### 5. Begin Migration

```bash
# Phase 2: Update dependencies
./claude/scripts/update-dependencies.sh

# Phase 3: Automated replacements
./claude/scripts/batch-replace.sh --dry-run  # Preview
./claude/scripts/batch-replace.sh             # Apply

# Or use Claude Code for targeted migration
claude-code "Migrate perseus-core package using the subagent task definition"
```

## Migration Phases

### Phase 1: Discovery & Analysis (1-2 hours)

**Goal:** Understand the full scope of migration

**Actions:**

- Run `analyze-codebase.sh`
- Review generated report
- Identify critical files
- Assess risks
- Plan timeline

**Output:** Comprehensive analysis report

### Phase 2: Dependency Updates (30 minutes)

**Goal:** Update Cargo.toml files to Sycamore 0.9

**Actions:**

- Backup all Cargo.toml files
- Update Sycamore versions
- Run `cargo update`
- Document initial errors

**Output:** Updated dependencies, error log

### Phase 3: Systematic Code Migration (7-10 days)

**Goal:** Transform code to Sycamore 0.9 API

**Approach Options:**

#### Option A: Automated Batch Processing

```bash
# Preview changes
./scripts/batch-replace.sh --dry-run

# Apply automated patterns
./scripts/batch-replace.sh

# Manual review and fixes
git diff
# Fix remaining issues
```

#### Option B: Module-by-Module with Subagents

```bash
# Use Claude Code with subagent tasks
claude-code --task subagents/perseus-core-migration.md
claude-code --task subagents/perseus-router-migration.md
claude-code --task subagents/perseus-macro-migration.md
```

#### Option C: Hybrid Approach (Recommended)

1. Run automated batch replacements for common patterns
2. Use subagents for complex modules
3. Manual review and refinement

**Migration Order:**

1. perseus-core (2-3 days)
2. perseus-macro (1 day)
3. perseus-router (1-2 days)
4. perseus-engine (1 day)
5. perseus-warp (0.5 day)
6. perseus-axum (0.5 day)
7. Examples (1 day)

### Phase 4: Validation & Testing (1-2 days)

**Goal:** Ensure everything works correctly

**Actions:**

```bash
# Run validation script
./scripts/validate-migration.sh

# Comprehensive testing
./scripts/run-tests.sh

# Performance benchmarks
cargo bench
```

**Output:** Test reports, performance metrics

### Phase 5: Documentation (1 day)

**Goal:** Update documentation and create migration guide

**Actions:**

- Update API documentation
- Create user migration guide
- Update examples in docs
- Write release notes

## Key Sycamore 0.9 Changes

### 1. No More Scope! 🎉

```rust
// OLD (0.8)
fn component<'a, G: Html>(cx: Scope<'a>, props: Props) -> View<G> {
    let signal = create_signal(cx, 0);
    view! { cx, div { "Hello" } }
}

// NEW (0.9)
fn component(props: Props) -> View {
    let signal = create_signal(0);
    view! { div { "Hello" } }
}
```

### 2. No More Generic Html Trait! 🎉

```rust
// OLD: View<G>
// NEW: View (automatic backend detection)
```

### 3. No More RcSignal! 🎉

```rust
// OLD: RcSignal<T>
// NEW: Signal<T> (all signals are 'static and Copy now)
```

### 4. New Signal Access Pattern

```rust
// For non-Copy types:
let value = signal.get_clone();  // Not .get().clone()
```

See `CLAUDE.md` for complete details.

## MCP Server Usage

### Filesystem MCP

```bash
# Pattern scanning
"Find all files with 'cx: Scope' pattern"
"List all Rust files in perseus-core"
"Search for 'create_signal(cx,' in the codebase"
```

### Git MCP

```bash
# Checkpoint creation
"Create git checkpoint named 'migration/pre-core-migration'"
"Commit current changes with message '[Migration] Core: Removed scope parameters'"
"Show diff for packages/perseus-core/src/template.rs"
```

### Sequential Thinking MCP

```bash
# Strategy planning
"Analyze the impact of removing lifetime 'a from Template<'a, G>"
"Plan the migration strategy for perseus-core package"
"Determine if this lifetime is Perseus-specific or Sycamore-specific"
```

### Brave Search MCP

```bash
# Solution finding
"Search for 'Sycamore 0.9 migration examples'"
"Find solutions for 'cannot infer type for type parameter G'"
"Look up best practices for Perseus state management"
```

## Safety Features

### 1. Automatic Backups

All scripts create backups before modification:

- Cargo.toml → Cargo.toml.backup
- Source files → {file}.pre-migration

### 2. Git Checkpoints

Frequent checkpoints enable easy rollback:

```bash
# Create checkpoint
./scripts/create-checkpoint.sh phase-3-complete

# Rollback if needed
git reset --hard migration/phase-3-complete
```

### 3. Compilation Gates

Scripts verify compilation before proceeding:

```bash
cargo check -p {package} || exit 1
```

### 4. Dry-Run Mode

Test changes before applying:

```bash
./scripts/batch-replace.sh --dry-run
```

## Troubleshooting

### Common Issues

**Issue:** `cannot find value 'cx' in this scope`

- **Cause:** Removed `cx: Scope` but forgot to remove `cx` from calls
- **Fix:** Remove `cx,` from function calls and macro invocations

**Issue:** `expected 0 lifetime parameters`

- **Cause:** Removed lifetime but type still references it
- **Fix:** Check if lifetime is Perseus-specific before removing

**Issue:** `cannot infer type for type parameter 'G'`

- **Cause:** Removed `<G: Html>` but still using `View<G>`
- **Fix:** Change all `View<G>` to `View`

See `docs/troubleshooting.md` for comprehensive guide.

## Rollback Procedures

### Rollback Everything

```bash
# Option 1: Git reset to baseline
git reset --hard migration/baseline

# Option 2: Restore all backups
find . -name "*.pre-migration" -exec sh -c \
  'mv "$1" "${1%.pre-migration}"' _ {} \;
```

### Rollback Specific Package

```bash
# Restore from git
git restore packages/perseus-core/

# Or from backup
find packages/perseus-core -name "*.pre-migration" -exec sh -c \
  'mv "$1" "${1%.pre-migration}"' _ {} \;
```

## Success Criteria

- ✅ All workspace members compile without errors
- ✅ 100% test pass rate
- ✅ All examples build and run
- ✅ No Sycamore 0.8 dependencies in Cargo.lock
- ✅ No deprecation warnings from Sycamore
- ✅ Perseus public API stable (or changes documented)
- ✅ Performance maintained or improved
- ✅ Documentation complete

## Estimated Timeline

- **Fast track** (with automation): 7-10 days
- **Standard** (careful manual review): 10-14 days
- **Conservative** (extensive testing): 14-20 days

Factors affecting timeline:

- Team familiarity with codebase
- Testing requirements
- Documentation needs
- Concurrent bug fixes

## Support & Resources

### Documentation

- `CLAUDE.md` - Complete project context
- `docs/breaking-changes.md` - Detailed change list
- `docs/troubleshooting.md` - Solutions to common issues
- `docs/timeline.md` - Suggested schedule

### External Resources

- [Sycamore Migration Guide](https://sycamore.dev/book/migration/0-8-to-0-9)
- [Sycamore v0.9 Announcement](https://sycamore.dev/post/announcing-v0-9-0)
- [Perseus Documentation](https://framesurge.sh/perseus/en-US)
- [Sycamore Discord](https://discord.gg/sycamore)

### Getting Help

1. **Check documentation** in this package
2. **Review analysis report** for specific issues
3. **Search Sycamore issues** for similar problems
4. **Ask on Sycamore Discord** (#perseus channel)
5. **Use Sequential Thinking MCP** for complex problems

## Contributing

Found an issue or improvement? This workflow is designed to be iterative:

1. Document the issue
2. Test the fix
3. Update relevant files
4. Share with the team

## License

This migration workflow package follows the Perseus project license.

---

**Good luck with your migration!** 🚀

Remember:

- Take it one step at a time
- Test frequently
- Commit often
- Don't hesitate to ask for help
- Celebrate progress along the way!
