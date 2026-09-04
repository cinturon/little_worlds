---
name: ship
description: Completes the current Linear ticket (commit, push, merge to develop), then opens the next Little Worlds ticket on a Gitflow feature branch and walks through it as a mentor. Use when the user says "ship", "ship it", "ship this ticket", or asks to commit, push, open the next ticket, and walk through it in one step.
---

# Ship

Close the current ticket and start the next one. Do not implement the next ticket.

Follow `.cursor/rules/gitflow.mdc` and `.cursor/rules/AGENT.md`.

## Workflow

Copy and complete:

```
Ship:
- [ ] Verify
- [ ] Commit
- [ ] Push feature branch
- [ ] Merge into develop (not main)
- [ ] Close Linear ticket
- [ ] Open next ticket
- [ ] Gitflow branch
- [ ] Mentor walkthrough
```

### 1. Verify

Run `cargo test --workspace`. Stop if it fails.

Identify the current Linear ticket from the branch name (`feature/jib-487-…`) or the In Progress issue on project **Little Worlds — AI Civilization Simulator**.

### 2. Commit

Commit only ticket files on the current `feature/` branch. Do not commit secrets.

```bash
git add <ticket-files>
git commit -m "$(cat <<'EOF'
Why this change exists.

EOF
)"
```

Skip if the working tree is already clean with the ticket committed.

### 3. Push and merge to develop

```bash
BRANCH=$(git branch --show-current)
git push -u origin HEAD
gh pr create --base develop --head "$BRANCH" --title "<ticket-id>: <title>" --body "$(cat <<'EOF'
## Summary
- Ships <ticket-id>

EOF
)"
gh pr merge --merge --delete-branch
```

If a PR already exists, merge that instead of opening a second one. Never merge to `main`.

### 4. Close current, open next

Mark the current issue **Done**.

Next ticket = the next incomplete issue in the Little Worlds project by `createdAt` (skip Done/Canceled).

Set it **In Progress**, assignee `me`, and open its Linear URL.

### 5. Branch

```bash
git checkout develop
git pull
git checkout -b feature/<ticket-id>-<short-kebab-description>
```

Example: `feature/jib-488-introduce-personid`. Call SetActiveBranch.

### 6. Walk through (do not code)

Use the AGENT.md orientation:

- What we're building
- Why
- What you'll learn
- What not to worry about yet
- Done when (acceptance criteria)
- First small action plus one question

Do not implement the ticket unless the user explicitly asks.

## Example

User: `ship`

Agent: verifies tests, commits `World::tick`, PRs into `develop`, marks JIB-487 Done, opens JIB-488, checks out `feature/jib-488-introduce-personid`, walks through PersonId.
