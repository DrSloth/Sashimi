There are two keywords used for looping, `loop` and `for`.

The `loop` keyword is used to loop infinitely. The syntax is `loop <Expression>`.
Example
```
use std::io;
loop {
    # This prints Hello, World until the program is killed.
    io::writeln("Hello, World!");
}
```
As loop accepts any kind of expression this could be rewritten as
```
loop io::writeln("Hello, World!);
```

There are two ways to restrict the iterations of `loop` the first possibility is to use `break`
```
let mut cnt = 0;
loop {
    if cnt == 10 {
        break;
    }
    io::writeln(cnt);
    cnt += 1;
}
```
or returning a value of `std::flow::ControlFlow`:
```
let mut cnt = 0;
loop {
    if cnt == 10 {
        ControlFlow::Break
    } else {
        io::writeln(cnt);
        cnt += 1;
        ControlFlow::Skip
    }
}
```
Anything that implements `into<ControlFlow<T,T>>` can be used, it should be carefully implemented.
For instance `Option<T>` implements `into<ControlFlow<T,T>>` where `None => Break(Void)`,
`Some(T) => Skip` so do `Either<ControlFlow, ControlFlow>` and `ControlFlow | ControlFlow`.

This means you can directly use the if construct with loop
```
let mut cnt = 0;
loop if cnt < 10 {
    io::writeln(cnt);
    cnt += 1;
}
```
This results in something similiar to a while loop in other languages.

You can `break` a `loop` with a value.
```
let x = loop {
    break 10;
};
# x is 10
```
If loop is not broken with a value it yields the value of the last iteration.
The `loop` stops as soon as a `break` is hit or the if does not succeed.

It is also possible to make loop return an `iterator` when using `continue` with a value.
Values given to `break` and `continue` have to be of the same type.

```
let mut cnt = 0;
let iter = loop if cnt < 2 {
    cnt += 1;
    continue cnt;
};
$assert_eq(iter.next(), Some(1));
$assert_eq(iter.next(), Some(2));
$assert_eq(iter.next(), None);
```

This also works with `flow::ControlFlow` by returning `Break(T)` or `Continue(T)`,
`loop` thus only accepts `ControlFlow<T,T>` and not `ControlFlow<B, C>`.

You can also add an else block to the `loop if`.
```
let x = loop if false {break 0;} else 10; # x is 10.
let y = loop if true {break 0;} else 10; # y is 0.
```
If you use `else if` and an `else if` block succeeds the loop will not continue. This works because
`else` is a seperate expression which operates on the `Option` type and has lower priority.
You can also use `loop` with `match` and the same rules apply.

IMPORTANT `loop` has a higher precedence than `else`.

The other available keyword is `for`, it also supports `continue` and `break`.
The syntax of `for` is `for <impl iter> [=> <Ident>] <Block>`.
Example:
```
for 0..10 => i {
    io::writeln(i); # Prints all numbers from 0 to (excluding) 10
}
```

There are four very common use cases `for` loops are used in.
They are abbreviated as `forn`, `fori`, `foriter` and `forin`.
The `forn` loop is used to loop `n` times. It has the form:
```
let n = 10;
for 0..n {
    # This will be executed `n` times
    io::writeln("Hello, World!");
}
```
The `fori` loop is used to loop for a certain number of times and know the number of passed
iterations, it is the closest to the classical C like numerical for loop.
```
for 0..10 => i {
    io::writeln(i); # Prints all numbers from 0 to (excluding) 10
}
```
The `foriter` loop is used to run something until an iterator is exhausted.
```
for [1,2,3,4,5].iter() {
    # Executes once for every element of the array
    io::writeln("Hello, World!")
}
```
It is the more general version of the `forn` loop.
The more general version of the `fori` loop is called `forin` and is used to iterate over every 
element inside an iterator while also saving it into a temporary variable.
```
for [1,2,3,4,5].iter() => e {
    io::writeln(e); # Prints 1,2,3,4,5 each on a single line.
}
```
These constructs are pronounced as if there was a space after `for`:
`forn`: "for n"
`fori`: "for i"
`foriter`: "for iter"
`forin`: "for in"