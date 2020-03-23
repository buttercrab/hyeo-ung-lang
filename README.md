<p align="center">
<a href="https://github.com/buttercrab/hyeo-ung-lang/blob/master/imgs/hyeong-light.png">
<img alt="혀엉..." src="https://github.com/buttercrab/hyeo-ung-lang/raw/master/imgs/hyeong-light.png" width="640"/>
</a>
</p>
<h1 align="center">Hyeo-ung Programming Language</h1>

<p align="center">
<a href="https://travis-ci.com/buttercrab/hyeo-ung-lang">
<img alt="Travis CI" src="https://img.shields.io/travis/com/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
<a href="https://hub.docker.com/r/buttercrab/hyeong">
<img alt="Docker Hub" src="https://img.shields.io/docker/cloud/build/buttercrab/hyeong?style=flat-square"/>
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
hyeong 0.1.2
hyeo-ung programming language tool

USAGE:
    hyeong [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --color <color>    whether prints color [default: auto]  [possible values: never, auto, always]

SUBCOMMANDS:
    build        Compiles hyeong code
    check        Parse your code and check if you are right
    debug        Debug your code command by command
    help         Prints this message or the help of the given subcommand(s)
    install      Install hyeong before build (need once)
    run          Run hyeong code directly
    uninstall    Uninstall hyeong temporary build path
```

# How to install

## Brew

```shell script
brew install buttercrab/tools/hyeong
```

## Cargo

```shell script
cargo install hyeong
```

## Docker

```shell script
docker run -it buttercrab/hyeong /bin/bash
```

Then, `hyeong` to execute.

## Download Binary

Go to [latest release](https://github.com/buttercrab/hyeo-ung-lang/releases/latest) to download.

## Build from source

Followings doesn't need to execute `hyeong install`

### Script file
1. Windows
```cmd
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.cmd" | cmd
```

2. Mac, Linux
```shell script
bash <(curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh")
```

### Make
```shell script
git clone https://github.com/buttercrab/hyeo-ung-lang.git
cd hyeo-ung-lang
make install
```

Add to your `PATH` to use.

## Uninstall

First,

```shell script
hyeong uninstall
```

Then if brew,

```shell script
brew uninstall hyeong
```