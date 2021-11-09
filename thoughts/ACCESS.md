By default everything is private. You can use `pub` to make something public.
Only the module which has direct access to the definition may use private parts of it.
Anonymous types act like completely public types because usage and definition is the same for them.
Every item can be made public.

This means for functions there is one spot to potentialy put a `pub`:

```
# Private function
fn fun() {

}

# Public function
pub fn fun() {

}
```

For types there are multiple spots:
```
# T is a public type but its definition is private, only public functions can be used on it.
pub type T = U;

# T is public and its definition is public it acts transparently the same as U.
pub type V = pub U;

# pub can be used on individual parts of a type
# here a can be accessed by other modules but b can't
pub type W = {pub a: X, b: Y};

# This works exactly the same for sum types, unions and all other compound types
```

One important thing about pub sum types has to be noted:

```
# This is a transparent type
pub type T = pub (U | V | W);

# Here only B is pub
pub type A = pub B | C | D;
```
