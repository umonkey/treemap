---
name: markdown
description: rules for formatting markdown documents and documentation. Use this whenever working with any files in the docs/ folder or any .md files.
---

## Markdown Formatting Rules

When creating or modifying markdown documents, you MUST adhere strictly to the following rules:

1. Minimal bold: never use bold in text unless absolutely necessary.
2. No numbered headers: do not use numbered headers (e.g., use `## Header` instead of `## 1. Header`).
3. Lowercase after colon in lists: in list items, the first letter immediately after a colon must be lowercase (e.g., "- key: value"). Capitalization in the rest of the sentence remains unchanged.
   - Bad: "- Foo: Bar"
   - Good: "- Foo: bar"
4. List spacing: always leave a blank line above and below a list, except for nested lists.
5. Post-processing: run `make format-docs` after any changes to files in the `docs/` folder.

