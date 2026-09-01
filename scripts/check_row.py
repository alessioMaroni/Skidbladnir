"""Script to verify that line lengths in source files do not exceed the specified limit."""

import sys
from pathlib import Path

# Maximum allowed line length in characters (including spaces and tabs)
MAX_ROW_LENGTH = 100

# Extensions of source files to check
EXTENSIONS = {".rs", ".adb", ".ads", ".c", ".h", ".s", ".nix"}

# Directories to skip during recursive scanning
SKIP_DIRS = {"target", "obj", ".git", ".direnv", ".venv"}


def check() -> None:
    """Recursively scan the current directory and report lines exceeding the maximum length."""
    failed = False

    # Scan all files and directories starting from the current location
    for path in Path(".").rglob("*"):
        # Skip files inside excluded directories
        if any(part in path.parts for part in SKIP_DIRS):
            continue

        # Filter for regular files matching the specified extensions
        if path.is_file() and path.suffix in EXTENSIONS:
            with open(path, "r", encoding="utf-8", errors="ignore") as f:
                for idx, line in enumerate(f, 1):
                    # Strip trailing newline characters (\n or \r\n) while preserving indentation
                    length = len(line.rstrip("\r\n"))

                    # Report an error if the line exceeds the maximum length
                    if length > MAX_ROW_LENGTH:
                        print(
                            f"{path}:{idx}: line exceeds {MAX_ROW_LENGTH} chars ({length} chars)"
                        )
                        failed = True

    # Return exit code 1 to fail Git hooks or CI/CD pipelines if errors are found
    if failed:
        sys.exit(1)


if __name__ == "__main__":
    check()