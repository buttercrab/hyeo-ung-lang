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

<p align="center">
<a href="https://github.com/buttercrab/hyeo-ung-lang/wiki/Documentation">Documentation</a> | 
<a href="https://github.com/buttercrab/hyeo-ung-lang/wiki/How-to-Install">Install</a> | 
<a href="https://github.com/buttercrab/hyeo-ung-lang/wiki/Language-Tutorial">Tutorial</a> | 
<a href="https://gist.github.com/xnuk/d9f883ede568d97caa158255e4b4d069">Original Gist</a>
</p>

# Features

More explanation on [Documentation](https://github.com/buttercrab/hyeo-ung-lang/wiki/Documentation).

```
Hyeong 0.1.0
hyeo-ung programming language tool

USAGE:
    hyeong [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build        Compiles hyeong code
    check        Parse your code and check if you are right
    debug        Debug your code command by command
    help         Prints this message or the help of the given subcommand(s)
    install      Install hyeong before build (need once)
    run          Run hyeong code directly
    uninstall    Uninstall hyeong before build
```

# How to install

## Homebrew

```shell script
brew install buttercrab/tools/hyeo-ung-lang
hyeong install
```

## Build from source

Followings doesn't need to execute `hyeong install`

1. Windows
```cmd
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.cmd" | cmd
```

2. Mac, Linux
```shell script
bash <(curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh")
```
