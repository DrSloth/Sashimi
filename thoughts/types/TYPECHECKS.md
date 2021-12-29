There are multiple questions the type checker has to solve.
There 

1. equal?:      type T = U, type U = V => T == U && U == V && T == V
    - Equal types have the same size
    - Equal types are field wise safely transmutable to each other
2. same?:       type T = U => only T is the same as T
3. compatible?  type T = pub U => T <- U => !(T <- V where V <- U) 
    - compatible types are assignable to each other
    - compatible types are always equal to each other
So: `type T = pub U;` `type V = U`: T == U, V == U, T == V, T <- U, !(V <- U), !(T <- V)
and `type T = pub U;` `type V = pub U`: T == U, V == U, T == V, T <- U, V <- U, !(T <- V)
