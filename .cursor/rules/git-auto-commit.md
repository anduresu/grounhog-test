# Git Auto-Commit Rule

## Purpose
Automatically commit files changed by Cursor IDE AI after each modification, using conventional git commit conventions with detailed explanations derived from the user prompts.

## Requirements

### Commit Timing
- Commit immediately after completing any file changes requested by the user
- Ensure all modified files are staged before committing
- Never commit partial or incomplete changes

### Commit Message Format
Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Commit Types
- **feat**: New features or functionality
- **fix**: Bug fixes
- **docs**: Documentation changes
- **style**: Code style changes (formatting, missing semicolons, etc.)
- **refactor**: Code refactoring without changing functionality
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Build process, dependency updates, or auxiliary tool changes
- **ci**: CI/CD configuration changes
- **build**: Build system changes

### Scope Guidelines
Use specific scopes based on the area of change:
- **cli**: Command-line interface changes
- **logging**: Logging and tracing modifications
- **config**: Configuration-related changes
- **specs**: Specification document updates
- **rules**: MDC rule changes
- **deps**: Dependency changes

### Commit Analysis Process
1. **Analyze the user prompt** to understand the intent and requirements
2. **Identify the type of change** performed (feature, fix, documentation, etc.)
3. **Determine the scope** of the change (which part of the system was affected)
4. **Summarize what was changed** in the description
5. **Explain why the change was made** in the body (derived from user prompt context)

## Examples

### Feature Addition
```
feat(cli): add explain command with tracing instrumentation

- Implemented basic explain command that prints "hello world"
- Added comprehensive tracing with structured logging
- Configured CLI argument parsing with clap derive macros
- Set up verbosity levels and quiet mode support

The user requested creation of an AI coding assistant CLI with
the first operation being "groundhog explain" that prints hello world.
The implementation includes proper observability as specified in the
project requirements.
```

### Documentation Update
```
docs(specs): remove GitHub-specific references from specifications

- Replaced GitHub Actions workflows with generic CI/CD pipeline descriptions
- Updated repository URLs to use placeholder values
- Changed dependency monitoring from Dependabot-specific to platform-agnostic
- Modified Homebrew formula to use generic URLs

The user requested removal of GitHub CI/CD pointers from the specs folder
to make the documentation platform-agnostic and usable with any Git
hosting service or CI/CD platform.
```

### Rule Creation
```
chore(rules): add git auto-commit MDC rule

- Created new MDC rule for automatic git commits after Cursor changes
- Defined conventional commit format requirements
- Specified commit timing and analysis process
- Added examples for different types of changes

The user requested a new Cursor IDE MDC rule to automatically commit
changed files after each Cursor operation, using conventional git
commit conventions with explanations derived from user prompts.
```

### Configuration Changes
```
feat(config): add dependency management and tracing setup

- Updated Cargo.toml with clap, tracing, and tokio dependencies
- Configured proper feature flags for CLI and observability
- Added development dependencies for testing
- Set up project metadata and description

The user requested setup of a Rust CLI application called "groundhog"
with tracing for logging/metrics and clap for command line parsing.
```

## Implementation Instructions

### Pre-Commit Checklist
1. ✅ Verify all intended files are modified
2. ✅ Check that no unintended files were changed
3. ✅ Ensure code compiles (if applicable)
4. ✅ Validate that changes align with user request

### Commit Execution Process
```bash
# Stage all modified files
git add <modified-files>

# Create commit with conventional format
git commit -m "<type>[scope]: <description>

<body explaining what was changed>

<body explaining why based on user prompt>"
```

### Message Generation Guidelines
- **Description**: Concise summary (50 chars or less)
- **Body**: Detailed explanation of changes made
- **Context**: Why the changes were needed (from user prompt)
- **Impact**: What functionality is affected or improved

### Scope Selection Rules
- Use the most specific applicable scope
- If multiple areas affected, use the primary scope
- For cross-cutting changes, use broader scope (e.g., "app", "core")
- For new features spanning multiple areas, use the main feature scope

## Exceptions

- **Emergency fixes**: May use shorter commit messages if time-critical
- **Work-in-progress**: Prefix with "wip:" if changes are incomplete
- **Experimental changes**: Use "experiment:" prefix for exploratory work
- **Revert commits**: Use "revert:" prefix with reference to original commit

## Quality Assurance

### Message Quality Check
- Is the type appropriate for the change?
- Does the scope accurately reflect the affected area?
- Is the description clear and concise?
- Does the body explain both what and why?
- Are there any typos or grammatical errors?

### Change Validation
- Do the changes match the user's request?
- Are all related files included?
- Are there any unintended side effects?
- Is the change complete and functional? 