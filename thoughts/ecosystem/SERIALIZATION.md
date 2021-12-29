Rusts serde library is already flexible in generic, serializing into concrete types is as efficient
as it gets. Serializing into dynamic types "trait objects" or here "impl types" is quite hard.
It is possible to serialize into trait objects but not into generic ones. 
Also not supporting generic "traits" is quite bad though this could be solvable in Rust (i think).
As only generically parsing into boxed trait objects is supported there is a lot of 
unnecassary inndirection is introduced. It is not possible to monomorphize the generics in Rust
because there is no way to list all implementors of a trait or do anything similiar.

In theory there are three ways to implement that generic serialization:
1. Monomorphization
    + Fast to execute, no overhead, very optimisiable
    - Bloats the code a lot because it generates code for every combination of types
2. Enums
    + Not as much code bloat
    + Still easily optimisable
    + Not much indirection
    - Big structure (size of biggest subpart, without optimisation)
3. Dyn
    + Least binary size addition
    - Optimisation hurdle
    - Pointer indirection
    - Less cache locality

There is no definitiv way of solving this problem. In most cases one indirection is necessary,
but that is only on the top level, everything after that could be solved with one of the mentioned
solutions (the top level also but that is less trivial). Introducing multiple indirections could be 
bad however. For instance something like `f -> g -> h` would require four complete pointer
indirections before being able to access the implementor of `h`.
The only real way to solve this is by presenting options to choose from the above mentioned 
solutions, this becomes even more useful if you are able to exclude types from the list 
of possible options and only include certain ones, this should also be doable through macros AND
the type system and decidable by the top level code consumer. Most of this should however
be automagically solved by optimisation options for the compiler.
