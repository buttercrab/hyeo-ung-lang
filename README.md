<h1 align="center">Hyeo-ung Programming Language</h1>

<p align="center">
<a href="https://github.com/buttercrab/hyeo-ung-lang/actions?query=workflow%3ARust">
<img alt="Github Action" src="https://img.shields.io/github/workflow/status/buttercrab/hyeo-ung-lang/Rust?style=flat-square"/>
</a>
<a href="https://github.com/buttercrab/hyeo-ung-lang/blob/master/LICENSE">
<img alt="License" src="https://img.shields.io/github/license/buttercrab/hyeo-ung-lang?style=flat-square"/>
</a>
</p>

[Hyeo-ung Programming Language](https://gist.github.com/xnuk/d9f883ede568d97caa158255e4b4d069) compiler written in rust.

## Goal

- Big Number
- Interpreter
  + `$ hyeong`
- Code runner
  + `$ hyeong run FILE`
- Compiler
  + `$ hyeong build FILE --flags...`
- Fast Speed (Algorithm)
- Optimization
- Debugger
  + `$ hyeong debug FILE --flag...`
- Nice Compile Error
- String to Hyeo-ung Code Generator
  + `$ hyeong string "Hello, World!"`
- Prove whether it is Turing Complete
- No unsafe rust code

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