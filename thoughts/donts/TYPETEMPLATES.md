Idea:
Type Template:
`type T<U:> = V;`
This works as any type template that doesn't contain `?` is instantiable as that type.
This however is an unbound type template there will also be the type requirement declaration:
`type T<U:>: V;` 
which means that T with any type parameter U has to extend V. They also "cascade"

Bad idea because:
Here it is really unclear that it is possible to specialize `T`. `T` being specializable and
"unrestricted" of course means that `T` is not really fixed and could be anything, thus writing
generic functions with `T` is pretty much impossible. 

## Better ideas:
### impl type
There is a strict difference between type declarations, requirements and specializations.
A type requirement is of the form: 
`type T<U:>: V;`
This means that type `T` with any type parameter `U` has to extend V.
In extension: 
`type T<U: W> = V;`
Means that type `T` may only accept type parameters `U`, which extend `W`, and has to extend `V`.
Using `type!` prevents further overloading of type requirements from happening.

Type requirements may be overloaded in the form of:
`impl type T<U: W+A> = V+B;`
Which means that any type `T` with any type parameter `U` which ALSO extends `A` has to extend `B`.


A type declaration takes the form of: 
`type T<U:> = V;`
This could be extended to:
```
type T<U:>: V;
impl type T<U:> = V;
```

A strict type declaration like:
`type! T<U:> = V;`
could be extended to:
```
type! T<U:>: V;
impl! type T<U:> = V;
```

The `impl` keyword is used to create types from type templates.

