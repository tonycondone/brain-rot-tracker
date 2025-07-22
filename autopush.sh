#!/bin/bash

# === Fully Auto Git Init + GitHub Push Script ===

# Get folder name as repo name
REPO_NAME=$(basename "$PWD")
BRANCH_NAME="main"

# Change this to your GitHub username
GITHUB_USERNAME="tonycondone"

# GitHub login required
if ! gh auth status &>/dev/null; then
  echo "🔒 GitHub CLI not authenticated. Run: gh auth login"
  exit 1
fi

# Step 1: Init Git if needed
if [ ! -d ".git" ]; then
  echo "📦 Initializing Git..."
  git init
  git config user.name "tonycondone"
  git config user.email "touyboateng339@gmail.com"
  git add .
  git commit -m "Initial commit"

  echo "🌐 Creating GitHub repo '$REPO_NAME'..."

  # Create repo with GitHub CLI
  gh repo create "$GITHUB_USERNAME/$REPO_NAME" --public --source=. --remote=origin --push
  git branch -M "$BRANCH_NAME"
  git push -u origin "$BRANCH_NAME"
fi

# Step 2: Auto-push loop
echo "🚀 Starting auto-push loop for $REPO_NAME..."
while true; do
  git add .
  git diff --cached --quiet && sleep 2 && continue
  git commit -m "auto: $(date)"
  git push origin "$BRANCH_NAME"
  sleep 2
done
