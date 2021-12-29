The `for` and `loop` loops can be labeled with a label. Using labels you can `continue ` 
or `break` an outer loop from an inner one without the introduction of additional variables.
Labels use the same syntax as lifetimes but aren't inherently the same.

```
'outer: for (0..10).filter(num::even?) => i {
    for (0..10).filter(num::even? -> !) => j {
        if i * j > 20 {
            break 'outer i*j;
        }
        io::writef("{i} * {j} = {i * j}"); # NOTE writef is a macro fn but this syntax doesn't work in fn mode.
    }
}
```
