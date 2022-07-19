# entitify

`entitify` is a cli utility for converting utf-8 text to html entities.

# Usage

```
Usage:
  target/release/entitify [OPTIONS]


Optional arguments:
  -h,--help             Show this help message and exit
  -h,--hex              Print all entities as hex
  -d,--decimal          Print all entities as decimal
```

# Examples

```shell
$ echo 'Entré' | entitify
Entr&eacute;

$ echo 'Entré' | entitify -h
Entr&#xe9;

$ echo 'Entré' | entitify -d
Entr&#233;
```
