find . -maxdepth 1 ! -name "build.sh" ! -name "remove.sh" ! -name "." ! -name ".." ! -name ".git" | xargs -I {} rm -rf {}
