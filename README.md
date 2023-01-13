# play-with-tree-sitter
Example of AST parsing Ruby/Python/PHP/TypeScript/JavaScript code

### How to install

```bash
# Install npm dependency (tree-sitter CLI) for debugging
npm i

# generate CLI configuration
npx tree-sitter init-config

# Needs to configure "parser-directories" to follow your desired directory structure of grammar repo.

# If you set "parser-directories" as `/Users/xxx/repo`.
cd ~/repo

# Clone language grammar repositories to be used and build each by running "npm i".
git clone https://github.com/tree-sitter/tree-sitter-typescript
cd ~/repo/tree-sitter-typescript && npm i

git clone https://github.com/tree-sitter/tree-sitter-javascript
git clone https://github.com/tree-sitter/tree-sitter-python
git clone https://github.com/tree-sitter/tree-sitter-php
git clone https://github.com/tree-sitter/tree-sitter-ruby
```

### How to develop

```bash
# Try running "Tagging" by CLI for each language
npx tree-sitter tags fixtures/Animal.js
npx tree-sitter tags fixtures/Post.ts
npx tree-sitter tags fixtures/user.rb
npx tree-sitter tags fixtures/User.php
npx tree-sitter tags fixtures/models.py

# Run "Tagging" tests from Rust code.
cargo test
```

### References

- [Using Tree-sitter Parsers in Rust](https://rfdonnelly.github.io/posts/using-tree-sitter-parsers-in-rust/)