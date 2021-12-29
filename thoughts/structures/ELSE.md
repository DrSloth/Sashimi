The `else` keyword is a binary operator which is used on one expression with any type implementing
the `left?` function, this includes `Either`, `Result`, `Option` and `T|V`, and another expression.
It is used to retrieve the `left!` value of the first expression or the second expression.

Examples:
```
let a = Left(10) else 2; # x is 10 and of type I32
let b = Right(10) else 2; # x is 2 and of type I32
let c = Left(10) else true; # z is 10 and has the type `10 | Bool`

let x = Some(10) else 2; # x is 10 and of type I32
let y = None else 2; # x is 2 and of type I32
let z = Some(10) else true; # z is 10 and has the type `10 | Bool` 
```

This is commonly used together with the `lrswap` function to get the `right!` value of something.

It is also common to use this together with `if` as either expression. 
This results in the usal if/else if/else chaining that are also common in other languages.

```
let x = 10;
let y = if x < 10 {
    10
} else if x > 10 {
    100
} else {
    0
};
io::writeln(y); # y is 0
```

Note that a `match` is likely the better option instead of if else chains. 
The previous example as match would be:
```
let x = 10;
let y = match x {
    _ if x < 10 => 10;
    _ if x > 10 => 100;
    _ => 0;
};
io::writeln(y); # y is 0
```
