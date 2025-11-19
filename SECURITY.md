# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via GitHub's private vulnerability reporting feature or email the maintainers directly.

You should receive a response within 48 hours. If for some reason you do not, please follow up to ensure we received your original message.

Please include the following information:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit the issue

## Security Considerations

### Secrets Management

* **Never commit secrets** to the repository
* Use environment variables or configuration files (excluded from git)
* The `config.toml` file is in `.gitignore` - keep it that way
* Rotate tokens regularly
* Use minimal permissions for bot tokens

### Dependencies

* We use Dependabot to keep dependencies up to date
* Security vulnerabilities are addressed as soon as possible
* All dependencies are regularly audited with `cargo audit`

### Code Security

* All user input is validated
* No arbitrary code execution
* Minimal permissions principle
* Regular security audits via CI/CD

## Migration from Legacy Python Bot

The legacy Python bot had 14 known vulnerabilities:
- 3 high severity
- 8 moderate severity
- 3 low severity

These were in outdated Python dependencies (discord.py 1.5.0, websockets 8.1, etc.).

The Rust rewrite completely eliminates these vulnerabilities by:
1. Using modern, actively maintained Rust libraries
2. Leveraging Rust's memory safety guarantees
3. Implementing proper error handling
4. Regular dependency updates via Dependabot
5. Automated security scanning in CI/CD

## Best Practices

### For Contributors

* Run `cargo clippy` before submitting PRs
* Never log sensitive information
* Validate all external input
* Use safe Rust patterns (avoid `unsafe` unless absolutely necessary)
* Write tests that include security scenarios

### For Operators

* Keep the bot updated to the latest version
* Use the Docker container for isolation
* Run with minimal privileges (non-root user)
* Monitor logs for suspicious activity
* Use read-only configuration mounts
* Enable security features in your platform (Discord, Telegram, etc.)

## Acknowledgments

We appreciate the security research community and will acknowledge researchers who responsibly disclose vulnerabilities (unless they prefer to remain anonymous).
