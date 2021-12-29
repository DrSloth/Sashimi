It is possible to define custom literals. There are multiple kinds of custom literals
1. String literals
2. Numeric literals
3. List literals
4. Map literals
5. Tuple literals

Custom literals are defined with macros whicha re annotated with either the `str_lit` or
`num_lit` macros. For instance a string literal which parses a user from `firstname;lastname`

```
$[str_lit(usr)]
macro username_lit(usrname: $Text) {
    $({
        let [firstname, lastname] = split_once(usrname, ';');
        User {
            firstname, 
            lastname
        }
    })
}

fn main() {
    let user = usr"Mark;Fischbach";
    std::io::write(user);
}
```

Numeric macros for meters and centimeters:
```
$[num_lit(m)]
macro meters_lit(meters: $Number) {
    $(Meters(meters.into<F64>()))
}

$[num_lit(cm)]
macro centimeters_lit(centimeters: $Integer) {
    $(Meters(centimeters.into<I64>() as F64 / 100))
}

use std::io;

fn main() {
    let height = 1m + 80cm;
    io::write(height); # 1.8m
}
```

Literals are available as soon as the containing module is in scope. 
One of the most important literals is the `f` from the `fmt` module it can be sysed to 
format strings, as a short form of the `format` macro function


```
use std::{fmt, io};

fn main() {
    let x = 10;
    let y: I64 = io::readline().parse();
    io::write(f"x: {x}, y: {y}");
}
```


--------------------

```
# Definition of write
fn write<W, C>(writer: W, content: C);

provide fn write<W, C: impl into<@[u8]>>(writer: W, content: C) {
    # code
}

provide fn write<C>(content: C) where C: impl write<Stdout, C> {
    let stdout = stdout();
    stdout.write(content);
}
```

