The `match` keyword is used to match the pattern of a variable, `match` has to be exhaustive.

The syntax of match is:
```
match <Expression> {
    (<FromPattern> [if <Condition>] => <ToExpression>;)+
}
```

As always the semicolon can be elided if the `ToExpression` is a block.

Match can be used on any type but unions. Match arms are matched from top to bottom.

### Skalars
Skalar types can be easily matched 

```
let b = true;
match b {
    true => 0;
    false => 10;
}
let x = 10;
match x {
    0 => -1;
    # With @ you can match a pattern and store the value in a variable.
    x @ 1..30 => x * 3; # Ranges are a patterns which match if a value is inside its bounds.
    # Match must be exhaustive so a every possible value has to be matched.
    z => z * 10; # Just a variable name is a pattern which always match.
}
```

### Sum types
Sum types can be matched on type.
```
let x: I32 | Bool = 10;
match x {
    I32 => io::writeln("Number");
    b @ Bool => io::writef("Boolean with value {b}\n");
}
```

### Maps and Tupels
Maps and Tupels can be structurely matched
```
let t = (1, true, "Hello");
match t {
    (a @ 0..10, true, c) if c.len() < 100 => io::writeln("Good tuple");
    t => io::writeln("Other tuple");
}
let m = {n: 10, b: false, s: "Bye"};
match m {
    {n: m @ 1..20, b, s: string} => io::writef("n: {m} is >= 1 < 20, b is {b}. {s}\n");
    d => io::writef("{?:d}\n");
} 
```

### Lists
List match syntax can be used on anything representable as list literal.
```
let x = [1,2,3,4,5]
match x {
    [5,4,3,2,1] => io::writeln("Wow. Numbers");
    [1, mid @ ..., x, 5] => io::writef("Starts with 1, ends with {x} and 5, mid is {?:mid}\n");
}
```

### Enums
Enums can be matched on variant.
```
let e: I64{0 = First, 1 = Second: I64} = First;
match e {
    First => io::writeln("First variant");
    Second(x) => io::writef("Second variant with value {x}\n");
}
```
