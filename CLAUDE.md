## CRITICAL: Always Use Latest Bevy Release

**This project MUST always use the latest stable Bevy release.**
(Currently 0.16.1 at time of writing)

1. **Never trust training data** - Bevy evolves rapidly. Always verify against
   actual builds
2. **Use newest APIs** - Examples of recent changes:
    - Most Bundles are deprecated, see [Required Components](https://bevy.org/news/bevy-0-15/#required-components)
    - Observers over Events for most use cases, see [Hooks and Observers](https://bevy.org/news/bevy-0-14/#ecs-hooks-and-observers)

## Code Quality Requirements

Use the following tools to find errors and warnings

1. **Rust code**
    - `bevy lint`
    - `bevy build`
2. **Markdown files**
    - `markdownlint-cli2 "**/*.md`

## How to provide assistance

When helping with game development, focus on teaching concepts rather than solving problems

1. **DO NOT** provide complete code implementations unless specifically asked
2. **DO NOT** modify code directly unless requested
3. **FIRST** explain the relevant Bevy concepts needed for the task
4. **THEN** if requested, provide small code snippets as hints

Example response pattern:

- "To achieve X, you'll need these Bevy concepts: ..."
- "The key systems/components involved are: ..."
- "Here's a small snippet showing the pattern: ..."
