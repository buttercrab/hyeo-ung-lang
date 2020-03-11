<h1 align="center">Hyeo-ung Programming Language</h1>

<p align="center">
<a href="https://travis-ci.com/buttercrab/hyeo-ung-lang">
<img alt="Travis CI" src="https://img.shields.io/travis/com/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
<a href="https://codecov.io/gh/buttercrab/hyeo-ung-lang">
<img alt="Codecov" src="https://img.shields.io/codecov/c/github/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
<a href="https://github.com/buttercrab/hyeo-ung-lang/releases">
<img alt="Release" src="https://img.shields.io/github/v/release/buttercrab/hyeo-ung-lang?include_prereleases&style=flat-square"/>
</a>
<a href="https://github.com/buttercrab/hyeo-ung-lang/blob/master/LICENSE">
<img alt="License" src="https://img.shields.io/github/license/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
</p>

[Hyeo-ung Programming Language](https://gist.github.com/xnuk/d9f883ede568d97caa158255e4b4d069) compiler written in rust.
(Developing)

**version 0.1.0 beta released**

# How to install

One of these commands **must** be executed.

## Build from source

1. Windows
```cmd
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.cmd" | cmd
```

2. Mac, Linux
```shell script
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh" | /bin/bash
```

## Install without build (don't install binary)

1. Windows
```cmd
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_without_build.cmd" | cmd
```

2. Mac, Linux
```shell script
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_without_build.sh" | /bin/bash
```

# Features

## `hyeong`

Runs interpreter

## `hyeong build FILE -O2 -o output_file`

Builds hyeong code to binary file, can optimize hyeong code

## `hyeong check FILE`

Check FILE and print each command 

## `hyeong debug FILE`

Debug file line by line, breakpoints, going previous

## `hyeong run FILE -O2`

Runs directly without making binary file