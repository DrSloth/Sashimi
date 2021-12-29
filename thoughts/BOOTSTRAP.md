Bootstrapping the language is a real main goal.
However there really are two ways to go about this depending on when macros are implemented.

1. Macros aren't implemented at first:
    - The language is implemented completely minimally
    - No macros and thus no interpreter
    - Compiler in Rust -> Compiler in itself -> Interpreter in itself -> More compiler features
2. Macros are directly implemented:
   - Interpreter in Rust -> Interpreter in itself -> Compiler in itself -> More features  
