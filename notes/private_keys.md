### Private Key Scanner for Repositories (JavaScript/Node.js)

This script is designed to scan the current working directory (and all its subdirectories) in a repository (e.g., a Git repo) to detect potential private keys or sensitive credential files. It helps prevent accidental commits of security-sensitive materials by identifying files that match common patterns associated with private keys. The script is implemented in JavaScript using Node.js for cross-platform compatibility and command-line execution.

#### Purpose
- **Security Check**: Ensures no private keys (e.g., SSH, RSA, ECDSA, or PEM-formatted keys) are present in the repo, which could lead to exposure if pushed to a remote like GitHub.
- **Scope**: Recursively scans the current directory (`.`) and all subdirectories, ignoring common non-code directories like `.git` to avoid false positives.
- **Detection Methods**:
  - **File Name Patterns**: Looks for files with extensions or names like `id_rsa`, `id_dsa`, `id_ecdsa`, `.pem`, `.key`, `.ppk` (PuTTY private key), or files named `private_key` (case-insensitive).
  - **Content Patterns**: Scans text files for strings indicating private keys, such as `-----BEGIN PRIVATE KEY-----`, `-----BEGIN RSA PRIVATE KEY-----`, `-----BEGIN EC PRIVATE KEY-----`, or similar PEM headers.
- **Output**: Reports any suspicious files with their paths, and optionally suggests actions like adding them to `.gitignore` or removing them.
- **Exit Codes**: Exits with 0 if no issues found (clean repo), 1 if potential private keys are detected (for integration with CI/CD pipelines).

#### Key Features
- **Recursive Traversal**: Uses Node.js's `fs` module with recursive directory reading to check every file in the tree.
- **File Filtering**: Skips binary files (e.g., images, executables) to focus on text-based files where keys are typically stored. Determines file type by attempting to read as text or checking extensions.
- **Ignore Patterns**: Configurable to ignore directories like `.git`, `node_modules`, `venv`, or `__pycache__` to reduce noise.
- **Customization**: Accepts command-line arguments via `process.argv` or a library like `commander` for:
  - Custom ignore lists.
  - Verbose mode for detailed logging.
  - Dry-run mode to simulate without changes.
- **Performance**: Efficient for large repos by limiting content scans to small files (<1MB) and using regex for pattern matching.
- **Dependencies**: Minimal – relies on built-in Node.js modules like `fs`, `path`, and `process`. For argument parsing, optionally uses `commander` or `yargs` (assume installed via npm).

#### How It Works (High-Level Logic)
1. **Initialization**: Parse command-line arguments (e.g., starting directory defaults to `.`).
2. **Directory Walk**: Traverse the directory tree recursively using a custom function or `fs.readdirSync` with recursion.
3. **File Checks**:
   - For each file, check if the name matches suspicious patterns (regex: `/(id_(rsa|dsa|ecdsa)|.*\.(pem|key|ppk))$/i`).
   - If it's a text file, read contents and scan for PEM headers (regex: `/-----BEGIN\s+(RSA|EC|DSA|PRIVATE)\s+KEY-----/`).
4. **Reporting**: Collect and log paths of matching files. If any found, warn the user and exit with error code.
5. **Optional Enhancements**: Integrate with Git pre-commit hooks to run automatically before commits.

#### Sample Usage
- Run in terminal: `node scan_private_keys.js` (scans current dir).
- With options: `node scan_private_keys.js --verbose --ignore=node_modules`.
