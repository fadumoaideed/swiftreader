## Version Control and CI/CD

### Branch Structure

- `main`: Production-ready code
- `feature/*`: Feature branches (e.g., feature/new-login, feature/api-update)

### Automated Release Setup

1. Install required tools:

```bash
npm install -g semantic-release @semantic-release/git @semantic-release/changelog
```

2. Create `.github/workflows/release.yml`:

```yaml
name: Release
on:
  push:
    branches:
      - main
      - 'feature/**'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Semantic Release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
        run: npx semantic-release
```

3. Create `.releaserc.json`:

```json
{
  "branches": ["main", { "name": "feature/*", "prerelease": "alpha" }],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release/npm",
    [
      "@semantic-release/git",
      {
        "assets": ["CHANGELOG.md", "package.json"],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ],
    "@semantic-release/github"
  ]
}
```

### Commit Message Convention

```bash
# Major version (breaking change)
git commit -m "feat!: completely new API"

# Minor version (new feature)
git commit -m "feat: add new function"

# Patch version (bug fix)
git commit -m "fix: correct calculation error"
```

### Release Process

1. **Automatic Versioning**:

   - Merging to `main`: Creates production release (e.g., 1.2.3)
   - Pushing to `feature/*`: Creates alpha release (e.g., 1.2.3-alpha.1)

2. **Installation**:

```bash
# Install latest stable version
npm install your-package-name

# Install latest alpha version from feature branch
npm install your-package-name@alpha
```
