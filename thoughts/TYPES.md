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

### Uninstantiable types:
?: Type elision 
!: The never type indicating that something can not exist

### Direct type alias: 
`type T = U;`

### References:
`type T = &U`

### Mutable References:
`type T = &mut U;`

### Map:
`type T = {a: U, b: V, c: W};`

### Tuple:
`type T = (U, V, W);`

### Fixed size Array:
`type T = [U;N];`
where N is an integer number.

### Vector:
`type T = [U];`
The only compound type with special syntax is defined as:
`type Vec<T> = {ptr: &[T;?], len: USize}`

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

### Function types:
`type T = fn(U, V) -> W;`

### Impl types:
`type T = impl f`
where f is some function or a tuple containing multiple functions.

### Type container:
`type Foo = ?;`

### Type template:
`type Foo<T:> = T;`
Type parameters end in `:` in the `<>`.
If a type parameter and a concrete type overlap the type parameter is preffered on the right hand.

They can be restricted in two ways:
1. `type Foo<T: I> = T`
2. `type Foo<T> = T where T: I`
   - The `:` can be omitted in the `<>` because it is restricted with where

Only type parameters and `Self` can be restricted.


Any given type has two important boolean attributes which define the usability of a type.
That is concreteness and instantiability. All normal types are instantiable and concrete.
The `!` type is concrete but not instantiable. 
Any type containing the `?` type is not instantiable and not concrete.

For type templates logic is similiar. Type templates can be instantiated but are never concrete.
That means a type template `TE<T:>` is not concrete but an instantiated form like `TE<T>` is.
One exception would be `TE<?>` which would also never be concrete.
