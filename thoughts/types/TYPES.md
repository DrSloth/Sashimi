All types that can exist are namable as anonymous type. 
Named types are only type aliases.
Type templates are just named types which generate anonymous types through type substitution.

## Kinds of types:
Given U, V, W are already defined types.

### Primitive types:
Unsigned ints: `U8, U16, U32, U64, U128, USize`
Signed ints: `I8, I16, I32, I64, I128, ISize`
Floats: `F32, F64, F128` (maybe supporting F16 makes sense)
Void: `Void` empty type with no size
char: `char` utf8 character

### Direct type alias: 
`type T = U;`

### View:
`type T = &U`
Extends to `View<U>`

### Mutable View:
`type T = &mut U;`
Extends to `ViewMut<U>`

### Pointer:
`type T = @U;`
Extends to `Ptr<U>`

### Mutable Pointer:
`type T = @mut U;`
Extends to `PtrMut<U>`

### Map:
`type T = {a: U, b: V, c: W};`

### Tuple:
`type T = (U, V, W);`

### Fixed size Array:
`type T = [U;N];`
where N is an integer number.
Extends to `Array<U, N>`

### Vector:
`type T = [U];`
Extends to `Vec<T>`

### Sum type:
`type T = U | V | W;`

### Simple Enum:
Given for instance `type U = U64;`
`type T = U{0 = Variant0, 1 = Variant1};`
This is equivalent to:
`type T = U{Variant0, Variant1};`
The first with actual values is needed with for instance: 
`type T = U{0 = Variant0, 2 = Variant1};`

### Sumtype Enum:
Given for instance `type U = U64;`
`type T = U{0 = Variant0: U, 1 = Variant1: V, 2 = Variant2: W};`
The same rules with tag values apply as with simple enums

### Unions:
`type T = {a: U | b: V | c: W};` 

### Callable types:
`type T = Fn(U, V) -> W;`
expands to `impl call<T, (U,V), W>`

`type T = FnMut(U, V) -> W;`
expands to `impl call_mut<T, (U,V), W>`

`type T = FnOnce(U, V) -> W;`
expands to `impl call_once<T, (U,V), W>`

### Function types:
Considering: `fn foo() {...}` there will be a type introduced called `\Fn_path_foo` which is zero sized
and implements all the `call` functions. The function itself will be a constant of that type.

A special case to this are closures. There are generic closure types that hold the function 
implementation as a callable type. The generated callable type includes captured variables:
`Closure<F: impl call<F, (ENV, ARGS)>, ENV: (...), ARGS: (...)> = {f: F, env: ENV};`
The callable held is a special one generated that will be named as `\Closure_module_line_i`
where `i` is a counter indicating the index of the closure on that line, this will obviously
be `0` in most cases, but it will be `1` for the second one defined in a line. 

The type of C like function pointers is `@Function<R, T...>` the `Function` type is an impl type with
a function private to std which is only implemented for normal functions 
`type Function<R, T...> = pub impl !pub function<R, T...>`

### Impl types:
`type T = impl f`
where f is some function or a tuple containing multiple functions.

### Type Elision
`?`
The `?` is not a concrete type. It is not instantiable and unknown. Creating an instance of a type
containing a `?` in any position, type parameter or actual type, is not instantiable

### Never
`!`
The never type indicates that something can not exist, it is allowed to be present in a type and 
doesn't hinder the complete type from being instantiable, just itself isn't. 
Returning `!` from a function means that the function never returns. 
It is also useful for instance as `Fallible<!, MyError>`, meaning: 
If this function returns, it has an error, `Fallible<!, MyError>::Ok` will never be created.

### Type container:
`type Foo = ?;`
Type containers are only used to be overwritten exactly once. They are used together with 
the `where Self: ` construct and are used to set one implementation of certain global types at 
compile time. This could for instance be used for platform specific features/defaults.
