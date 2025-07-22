#!/bin/bash

REPO_NAME=$(basename "$(pwd)")
echo "🚀 Starting auto-push loop for $REPO_NAME..."

# Auto-initialize if .git doesn't exist
if [ ! -d ".git" ]; then
  echo "Initializing Git for repo: $REPO_NAME"
  git init
  git add .
  
  git commit -m "Initial commit"
  echo "➤ Now manually connect your GitHub repo:"
  echo "    1. Create a repo on GitHub named: $REPO_NAME"
  echo "    2. Run:"
  echo "       git remote add origin https://github.com/<your-tonycondone>/$REPO_NAME.git"
  echo "       git branch -M main"
  echo "       git push -u origin main"
  echo "➤ Exiting to let you connect remote."
  exit 0
fi

# Detect and rename branch if it's still 'master'
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" = "master" ]; then
  echo "⚙️ Renaming branch 'master' to 'main'"
  git branch -M main
  CURRENT_BRANCH="main"
fi

# Auto-push loop
while true; do
  git add .
  git commit -m "auto: $(date)"
  git push origin "$CURRENT_BRANCH"
  sleep 10
done
