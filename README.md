# play-with-tree-sitter
Example of AST parsing Ruby/Python/PHP/TypeScript/JavaScript code

### Prerequisites

Needs to install tree-sitter CLI for easy debugging.

```bash
# Install npm dependency (tree-sitter CLI)
npm i

# generate configuration
tree-sitter init-config

# Needs to configure "parser-directories" to follow your desired directory structure of grammar repo.

# If you set "parser-directories" as `/Users/xxx/repo`.
cd ~/repo

# Clone language grammar repositories to be used and build each by running "npm i".
git clone https://github.com/tree-sitter/tree-sitter-typescript
cd ~/repo/tree-sitter-typescript && npm i

git clone https://github.com/tree-sitter/tree-sitter-javascript
cd ~/repo/tree-sitter-javascript && npm i

git clone https://github.com/tree-sitter/tree-sitter-python
cd ~/repo/tree-sitter-python && npm i

git clone https://github.com/tree-sitter/tree-sitter-php
cd ~/repo/tree-sitter-php && npm i

git clone https://github.com/tree-sitter/tree-sitter-ruby
cd ~/repo/tree-sitter-ruby && npm i
```

### References

- [Using Tree-sitter Parsers in Rust](https://rfdonnelly.github.io/posts/using-tree-sitter-parsers-in-rust/)