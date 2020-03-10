<h1 align="center">Hyeo-ung Programming Language</h1>

<p align="center">
<a href="https://travis-ci.com/buttercrab/hyeo-ung-lang">
<img alt="Travis CI" src="https://img.shields.io/travis/com/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
<a href="https://codecov.io/gh/buttercrab/hyeo-ung-lang">
<img alt="Codecov" src="https://img.shields.io/codecov/c/github/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
<a href="https://github.com/buttercrab/hyeo-ung-lang/blob/master/LICENSE">
<img alt="License" src="https://img.shields.io/github/license/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
</p>

[Hyeo-ung Programming Language](https://gist.github.com/xnuk/d9f883ede568d97caa158255e4b4d069) compiler written in rust.
(Developing)

## Goal

- [x] Big Number
- [x] Interpreter
  + `$ hyeong`
- [x] Code runner
  + `$ hyeong run FILE`
- [x] Compiler
  + `$ hyeong build FILE --flags...`
- [x] Optimization
- [x] Debugger
  + `$ hyeong debug FILE --flag...`
- [ ] String to Hyeo-ung Code Generator
  + `$ hyeong string "Hello, World!"`
- [ ] Prove whether it is Turing Complete
- [x] No unsafe rust code
- [ ] Documented code
- [ ] Language tutorial (English)
- [x] Examples

## Implementation

```
 Complier:      | Interpreter:
                | 
 Hyeo-ung code  |  Hyeo-ung code
                | 
       |        |        |
       V        |        V
                | 
   Rust code    | Execute by line
                | 
       |        | 
       V        | 
                | 
 Binary Program | 
```

## Implemented

- Big Number:
  + Arithmetic operators, `to_string` and `from_string` method
- Compiler:
  + parser: O(n) algorithm **iterates whole code exactly twice**
- Code Runner:
  + optimized & unoptimized code runner
- Interpreter:
  + interactive code runner
- Optimizer:
  + level 1 optimize
  + level 2 optimize
- Debugger:
  + line by line (with going back feature)
- Compiler:
  + commands onlY (area will be supported soon)
- Examples:
  + `Hello, world!`, `a + b`, `a * b`
  
---

## Feature

- Compile hyeo-ung code
  + `$ hyeong build FILE -O2 -Wall`
- Parse hyeo-ung code
  + `$ hyeong check FILE`
- Debug hyeo-ung code
  + `$ hyeong debug FILE --from 12`
- Run directly without making binary file
  + `$ hyeong run FILE -O2 -Wall`
- Update this tool
  + `$ hyeong update`
- Interpreter
  + `$ hyeong`
  
## How to install

*(will be published when stable version comes out)*

```shell script
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh" | /bin/bash
```