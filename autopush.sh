#!/bin/bash

# === Fully Auto Git Init + GitHub Push Script ===

# Get folder name as repo name
REPO_NAME=$(basename "$PWD")
BRANCH_NAME="main"

# Change this to your GitHub username
GITHUB_USERNAME="tonycondone"
GIT_EMAIL="touyboateng339@gmail.com"

# Check GitHub auth
if ! gh auth status &>/dev/null; then
  echo "🔒 GitHub CLI not authenticated. Run: gh auth login"
  exit 1
fi

# Init repo if not already
if [ ! -d ".git" ]; then
  echo "📦 Initializing Git..."
  git init
  git config user.name "$GITHUB_USERNAME"
  git config user.email "$GIT_EMAIL"
  git add .
  git commit -m "Initial commit"
fi

# Rename branch if it's still 'master'
CURRENT_BRANCH=$(git symbolic-ref --short HEAD)
if [ "$CURRENT_BRANCH" = "master" ]; then
  echo "⚙️ Renaming 'master' branch to '$BRANCH_NAME'..."
  git branch -M "$BRANCH_NAME"
  CURRENT_BRANCH="$BRANCH_NAME"
fi

# Add remote origin if missing
if ! git remote get-url origin &>/dev/null; then
  echo "🌐 Creating GitHub repo '$REPO_NAME'..."
  gh repo create "$GITHUB_USERNAME/$REPO_NAME" --public --source=. --remote=origin --push
else
  echo "✅ Remote origin already set"
fi

# Auto-push loop
echo "🚀 Starting auto-push loop for $REPO_NAME..."
while true; do
  git add .
  git diff --cached --quiet && sleep 2 && continue
  git commit -m "auto: $(date)"
  git push origin "$BRANCH_NAME"
  sleep 2
done