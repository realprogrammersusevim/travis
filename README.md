# Travis

This is a little travesty generator. It fills in the probable next word using a
text file that you supply. I used
[Gutenberg](https://github.com/pgcorpus/gutenberg) and combined everything into
one file with and cleaned it with some shell script but you are free to use
whatever text file you like.

## Usage

```help
sage: travis [OPTIONS]

Options:
  -l, --length <LENGTH>    [default: 10]
  -v, --verbose
  -i, --input <INPUT>      [default: /Volumes/Storage/git/clean.txt]
  -t, --threads <THREADS>  [default: 8]
  -h, --help               Print help
  -V, --version            Print version
```
