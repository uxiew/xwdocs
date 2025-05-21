---
mode: 'agent'
tools: ['githubRepo', 'codebase','websearch','terminal']
description: "Convert a Ruby project to Rust"
---

## Project Background
- Original project path: `/Users/bing/Projects/devdocs_rust/devdocs-original`
- Documentation location: `[Original project path]/docs`
- Core definitions: `[Original project path]/lib/core`
- Core filter operations: `[Original project path]/lib/filters/core`
- Scraper definitions: `[Original project path]/lib/scrapers`
- Multiple development document filters and scrapers: `[Original project path]/lib/filters/*` and `[Original project path]/lib/scrapers/*`
- test files: `[Original project path]/test/*`

## Current Task
- Focus on converting the Babel documentation scraping logic located in `[Original project path]lib/filters/babel`.

## Copilot Usage Requirements
- **Maintain original project logic consistency**: The conversion process must strictly follow the original project's logical structure and avoid hardcoding.
- **Follow best practices of the target language**: The converted code should comply with the best practices and conventions of the target programming language, ensuring the code is clean, optimized, and maintainable.
- **Avoid feature assumptions**: Do not add or modify any features that are not present in the original project.
- **Isolated testing**: If testing new functionality is necessary, create a separate small project in the root directory to avoid contaminating the current project.
- **Automatic fixes**: Automatically apply necessary fixes when confirmed consistent with the original project logic without requiring user confirmation.