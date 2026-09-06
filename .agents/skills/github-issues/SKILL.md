---
name: github-issues
description: Creating and formatting GitHub issues via the gh CLI. Use when reporting bugs or drafting new issues. Do NOT use for git commit, push, or PR workflows.
---

# GitHub Issues

Instructions for creating GitHub issues in the repository.

## Workflow

- Tooling: use the existing `gh` CLI only. Assume it is authenticated and ready to work.
- Duplicate check: inspect existing issues using `gh issue list` or `gh issue list --search "<keywords>"` to see if the issue is already reported.
- Duplicates found: if related issues exist, still provide the suggested title and description, list the matching issue numbers and titles, and ask the user to confirm whether to proceed with submitting a similar issue.
- Style: avoid fluff, conversational filler, and decorative elements.
- Emojis: do not use emojis in issue titles, descriptions, or comments.
- Confirmation: always present the drafted title and description to the user and obtain confirmation before submitting.
- Submission: execute `gh issue create --title "<title>" --body "<description>"`.
- Result: return the resulting issue URL to the user upon completion.

## Structure

- Title: concise, imperative summary of the bug or feature request.
- Summary: brief problem statement.
- Reproduction steps: ordered steps to reproduce the issue (for bugs).
- Expected behavior: clear description of the expected outcome.
- Actual behavior: description of what currently happens.
- Proposed solution: concrete technical suggestions or relevant component references.
