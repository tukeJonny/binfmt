# binfmt

[![build](https://github.com/tukeJonny/binfmt/actions/workflows/release.yml/badge.svg)](https://github.com/tukeJonny/binfmt/actions/workflows/release.yml)
[![test](https://github.com/tukeJonny/binfmt/actions/workflows/test.yml/badge.svg)](https://github.com/tukeJonny/binfmt/actions/workflows/test.yml)

## What

binfmt dumps binary spec format.

## Usage

```
$ binfmt --help
binfmt 0.1.0

USAGE:
    binfmt [FLAGS/OPTIONS] [<file-path>]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file-path>    path to spec yaml file
```

## Run

First, we writes spec file as below.

```yaml
fields:
  - name: Part1 (32b)
    bitsize: 32
  - name: Part2 (8b)
    bitsize: 8
  - name: Part3 (128b)
    bitsize: 128
  - name: Part4 (16b)
    bitsize: 16
  - name: Part5 (40b)
    bitsize: 40
  - name: Part6 (16b)
    bitsize: 16
```

Finally, we executes binfmt with spec file.

```
$ binfmt ./examples/test1.yaml
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Part1 (32b)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|  Part2 (8b)   |                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Part3 (128b)                                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|               |         Part4 (16b)           |               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                 Part5 (40b)                                   |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Part6 (16b)           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```