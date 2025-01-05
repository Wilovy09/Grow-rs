# Contributing to this Project

Thank you for your interest in contributing to this project! These guidelines will help you collaborate effectively and ensure the project grows in an organized manner.

## Types of Contributions

You can contribute in the following ways:

- **Reporting Issues**: If you find a bug or have a suggestion, open an [issue](https://github.com/your-repo/issues).
- **Proposing New Features**: Share your ideas by creating an issue to discuss them before implementing significant changes.
- **Contributing Code**: Fix bugs, add new features, or improve the documentation.
- **Improving Documentation**: Keep code comments and documentation files clear and up to date.

## Setting up the Development Environment

1. Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:

```bash
git clone https://github.com/Wilovy09/Grow.git
cd Grow
```

3. Build the project:

```bash
cargo build
```

## Contribution Workflow

1. Fork the repository.
2. Create a descriptive branch:

```bash
git checkout -b fix-error-message
```

3. Make your changes: Ensure your code follows the Rust conventions and passes the linters:

```bash
cargo fmt
cargo clippy
```

4. Write commits following the Conventional Commits standard. Examples:

   - `fix: corrected input validation error`
   - `feat: added file upload functionality`
   - `docs: updated README with additional instructions`

5. Push your changes to your fork:

```bash
git push origin fix-error-message
```

6. Open a Pull Request: From your fork, create a PR to the main branch of this repository. Be sure to clearly describe the changes you made.

## Code Standards

- Follow Rust best practices using rustfmt and clippy.
- Write tests for any new changes in tests/.
- Keep your code modular and easy to read.

## Commit Standards

We use the Conventional Commits standard. Here are the most common types:

- `feat`: For new features.
- `fix`: For bug fixes.
- `docs`: For documentation changes.
- `style`: For code formatting changes (without affecting logic).
- `refactor`: For code changes that neither fix a bug nor add a feature.
- `test`: For adding or correcting tests.
- `chore`: For updates to tools or auxiliary tasks.

**Example commit:**

```bash
git commity -m "feat: added support for multiple users"
```

## Pull Request Review

- Contributions will be reviewed by a team member.
- If needed, you’ll be asked to make adjustments before the PR is approved.
- Use descriptive labels and reference related issues (e.g., Fixes #123).
