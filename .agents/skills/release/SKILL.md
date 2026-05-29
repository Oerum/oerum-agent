---
name: release
description: The mandatory workflow for creating and publishing a new release of the oerum-agent. Make sure to use this skill whenever the user mentions creating a release, publishing a new version, bumping the version, or cutting a release.
---

# Release Skill

This skill defines the mandatory workflow for creating and publishing a new release of `oerum-agent`.

## Workflow

When asked to create a new release, follow these steps exactly:

1. **Check and Bump Version**: 
   - Check `Cargo.toml` for the current version.
   - If needed, bump the workspace version (e.g., from `0.2.4` to `0.2.5`) and ensure `brain-cli` and `brain-core` inherit it.
2. **Commit and PR**:
   - Create a new branch (e.g., `release/v0.2.5`).
   - Commit the version bump and push.
   - Create a Pull Request using `gh pr create`.
3. **CI Checks**:
   - Wait for the PR CI checks to pass using `gh pr checks`. Do not merge if checks fail.
4. **Merge**:
   - Merge the PR with `gh pr merge --merge --delete-branch`.
5. **Create Release**:
   - Checkout the base branch (e.g., `main`).
   - Create the release as a **draft** using the GitHub CLI: `gh release create v<VERSION> --draft --generate-notes`.
     *(CRITICAL: It must be a draft, otherwise the background workflow cannot upload assets to it because published releases are immutable!)*
6. **CRITICAL: Wait for Release Assets**:
   - Creating the release triggers a background GitHub Actions workflow that builds and uploads binaries (e.g., `brain-windows-x64.zip`).
   - You MUST run `gh run list` and wait for the release workflow to successfully complete (`completed success`).
   - Do NOT inform the user that the release is ready to download until this workflow finishes. If the user tries to download before the assets are uploaded, the installation scripts will fail with a "Release asset not found" error.
7. **Notify User**:
   - Only after the assets are fully uploaded, notify the user that the draft release is ready.
   - Instruct the user to review the draft and publish it via the GitHub UI, or use `gh release edit v<VERSION> --draft=false` to publish it.
