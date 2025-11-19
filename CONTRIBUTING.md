# Contributing to Yoshi Bot

First off, thank you for considering contributing to Yoshi Bot! It's people like you that make Yoshi Bot such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by respect and professionalism. Please be kind and constructive.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why**
* **Include logs if applicable**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and explain which behavior you expected to see instead**
* **Explain why this enhancement would be useful**

### Adding New Commands

Adding new commands is super easy! Just:

1. Create a new file in `src/commands/your_command.rs`
2. Implement the `Command` trait
3. Register it in `src/commands/mod.rs`
4. Write tests for your command
5. Update documentation

See the README for a complete example.

### Adding New Platforms

To add support for a new platform:

1. Create a new module in `src/platforms/your_platform/`
2. Implement the `Platform` trait
3. Add it to `src/platforms/mod.rs` with conditional compilation
4. Update `Cargo.toml` with the platform's dependencies
5. Write integration tests
6. Update documentation

### Pull Requests

* Fill in the required template
* Do not include issue numbers in the PR title
* Include screenshots and animated GIFs in your pull request whenever possible
* Follow the Rust style guide (rustfmt)
* Include tests
* End all files with a newline
* Avoid platform-specific code where possible

## Development Process

1. Fork the repo
2. Create a new branch from `main`
3. Make your changes
4. Run tests: `cargo test --all-features`
5. Run formatting: `cargo fmt --all`
6. Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
7. Commit your changes
8. Push to your fork
9. Create a Pull Request

## Testing

All new features should include tests:

```bash
# Run all tests
cargo test --all-features

# Run tests with coverage
cargo tarpaulin --verbose --all-features --workspace

# Run specific test
cargo test test_name
```

Aim for at least 80% coverage for new code.

## Style Guide

* Use `rustfmt` for formatting (included in CI)
* Use `clippy` for linting (included in CI)
* Write descriptive commit messages
* Add documentation comments (`///`) for public APIs
* Keep functions small and focused
* Prefer composition over inheritance
* Use meaningful variable names

## Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

## Documentation

* Update the README.md if you change functionality
* Add doc comments to public APIs
* Include examples in doc comments where helpful
* Update CHANGELOG.md for notable changes

## Questions?

Feel free to open an issue with your question or reach out to the maintainers.

Thank you for contributing! 🦖
