# locate
locate is command-line tool that recursively searches for filenames that match regex pattern.

### Why locate exists
locate serves two purposes:
1. provide a easy-to-use filename search tool, an alternative to `find` (to some extent);
2. be an example project for Rust application development and continuous integration (CI) tools

### Usage
To use locate, simply open any terminal and enter `locate --help` to list all availaible options.

For example, if you want to list all Python executables:
```
locate "^python[\d](\.\d){0,1}$" -e
```
