fd --no-ignore "input.txt|problem.md" | rm
bfg --delete-files "input.txt" "$(pwd)"
bfg --delete-files "problem.md" "$(pwd)"
git reflog expire --expire=now --all && git gc --prune=now --aggressive
