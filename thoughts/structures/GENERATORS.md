Generators are functions which can be used to create iterators. They are functions which return 
a value and are continuable with the iterator `next` function.

Generators return opaque types of the form `\generator_<FunctionPath>` but you should just return 
the opaque `Generator<T>` type which implements `Iterator` and is usable in for loops.

```
fn range_until_n(n: I64) -> Generator<I64> {
    let i = 0;
    loop if i < n {
        return continue i;
        i += 1; 
    }
}
```

The combination of `return` and `continue` can be used to return a value and continue afterwards.
This automatically turns the return type of a function into a generator of the returned values.
A generator may be stopped by using `return break value` after which the `Generator` becomes 
finished, it will only return `None` on calls to next afterwards. Generators are fused.
Any function using `return break/continue` become a generator function. 
Using a normal return is not allowed while also using `return continue` or `return break`.
When not using `return continue/break` you have to return a Generator you got from somewhere else,
tail returns also work for this normally, a tail return in a generator function works just like
a normal `return break value` does.
