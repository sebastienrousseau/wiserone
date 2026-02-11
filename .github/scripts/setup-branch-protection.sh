#!/bin/bash
# Setup branch protection rules to enforce CI gates
# This script configures branch protection via GitHub CLI

set -euo pipefail

# Configuration
REPO_OWNER="sebastienrousseau"
REPO_NAME="wiserone"
MAIN_BRANCH="main"

echo "🔒 Setting up branch protection for ${REPO_OWNER}/${REPO_NAME}"

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is required but not installed"
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check authentication
if ! gh auth status &> /dev/null; then
    echo "❌ Not authenticated with GitHub CLI"
    echo "Run: gh auth login"
    exit 1
fi

echo "✅ GitHub CLI authenticated"

# Create branch protection rule
echo "🛡️ Configuring branch protection for '$MAIN_BRANCH' branch..."

gh api repos/${REPO_OWNER}/${REPO_NAME}/branches/${MAIN_BRANCH}/protection \
  --method PUT \
  --field required_status_checks='{
    "strict": true,
    "contexts": [
      "🔍 Pre-flight Checks",
      "🧪 Test Suite (ubuntu-latest, stable)",
      "🧪 Test Suite (ubuntu-latest, nightly)",
      "🧪 Test Suite (macos-latest, stable)",
      "🧪 Test Suite (macos-latest, nightly)",
      "🔒 Security Audit",
      "📚 Documentation & Examples",
      "📊 Performance Benchmarks",
      "🎯 CI Gate"
    ]
  }' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false,
    "require_last_push_approval": false
  }' \
  --field restrictions=null \
  --field required_linear_history=false \
  --field allow_force_pushes=false \
  --field allow_deletions=false \
  --field block_creations=false \
  --field required_conversation_resolution=true

echo "✅ Branch protection configured successfully!"

echo ""
echo "📋 Branch protection summary:"
echo "  • All CI checks must pass before merge"
echo "  • At least 1 approving review required"
echo "  • Stale reviews dismissed on new changes"
echo "  • Force pushes blocked"
echo "  • Branch deletions blocked"
echo "  • Conversations must be resolved"
echo "  • Administrators are subject to these rules"

echo ""
echo "🎯 Required status checks:"
echo "  • Pre-flight Checks (formatting, clippy, type checking)"
echo "  • Test Suite (Ubuntu + macOS, stable + nightly)"
echo "  • Security Audit (vulnerability scan, license check)"
echo "  • Documentation & Examples"
echo "  • Performance Benchmarks"
echo "  • CI Gate (overall status)"

echo ""
echo "✨ Branch protection is now active!"
echo "⚠️  No merging to main is possible until ALL CI checks pass."