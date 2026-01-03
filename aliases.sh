#!/bin/bash
# unix
alias sizes="find . -type f -exec du -h {} + | sort -r -h | head"
alias ls="ls -alh"
alias c="cursor"
alias pcr="pre-commit run --all-files"
# git
alias add="git add -A"
alias push="git push"
alias pull="git pull"
alias s="git status"
alias diff="git diff"
alias log="git log --graph --abbrev-commit --decorate --format=format:'%C(bold blue)%h%C(reset) - %C(bold green)(%ar)%C(reset) %C(white)%s%C(reset) %C(dim white)- %an%C(reset)%C(auto)%d%C(reset)' --all"
# directories
alias dt="cd ~/Desktop/"
alias dl="cd ~/Downloads/"
alias gd="cd ~/Google\ Drive/My\ Drive/"
alias ry="cd ~/repository/"
