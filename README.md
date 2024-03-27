# Giterator
Ever wondered why Git histories exist if we just ignore them? Giterator is a powerful command-line tool designed to simplify your Git workflow by providing seamless iteration over commit histories. Explore your project's evolution effortlessly, perform analyses, execute scripts, and generate output in various formats.

## Usage
### Count Lines of Code (LoC) Over Time
```shell
# Count lines of code in Rust files over time
giterator 'find . -type f -name "*.rs" -exec cat {} + | wc -l'
```

### Explore Commits in Another Repository
```shell
giterator command other_repo
```

### Execute Custom Script over each Commit
```shell
# Run a custom script (myscript.sh) on each commit in a repository
giterator --script myscript.sh on_my_repo
```

### Output in JSON Format
```shell
# Generate JSON output for commit analysis
giterator --json command
```

### Output in CSV Format
```shell
# Generate CSV output for commit analysis
giterator --csv command
```

## Environment Variables
When running a custom script or command with Giterator, the following environment variables are automatically exposed:
- `$GIT_REPO`: The name of the repository being iterated over.
- `$COMMIT_HASH`: The hash of the current commit being processed.
- `$COMMIT_DATETIME`: The date and time of the current commit being processed.
- `$COMMIT`: The name of the current commit being processed.

These environment variables allow your custom script or command to access important information about the repository and the current commit being iterated over. You can use these variables to customize the behavior of your script or command.
