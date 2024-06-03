## Contributing to ms-rs
Thank you for considering contributing to ms-rs! Your help is greatly appreciated.

### Getting Started
1. Fork the repository on GitHub.
2. Clone your fork locally:

```sh
git clone https://github.com/yourusername/ms-rs.git
```

3.Create a new branch for your contribution:

```sh
git checkout -b my-feature-branch
```

4. Make your changes to the codebase.
5. Commit your changes using conventional commits (see below).
6. Push your changes to your fork:
```sh
git push origin my-feature-branch
```

7. Open a pull request on GitHub.


### Conventional Commits
We try to use the Conventional Commits specification for our commit messages. This makes it easier to understand the history of the project and automate the release process.

#### Format
Each commit message should include a type, a scope (optional), and a subject:

```
<type>(<scope>): <subject>
```

##### Types
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools and libraries

##### Examples
- `feat(parser): add support for single unit times`
- `fix(format): correct formatting of milliseconds`
- `docs(readme): update usage examples`
- `style(lib): reformat code according to rustfmt`

#### Pull Request Guidelines
Ensure your pull request description clearly describes the problem and solution.
Include any relevant issue numbers.
Add tests to cover your changes if applicable.
Ensure all tests pass before submitting your pull request.

#### Code of Conduct
By participating in this project, you agree to abide by the Contributor Covenant Code of Conduct.

Thank you for contributing! `:-)`
