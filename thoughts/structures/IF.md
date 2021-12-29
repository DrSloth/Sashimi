The `if` expression is used to conditionally execute code.
The syntax of `if` is `if <ConditionExpression> <Block>`. If conditions are expressions
which always return values of type `Option`, `Some` is returned when the if succeeds, None
is returned if the condition evaluates to false. Examples:
```
let x = if true {10}; # x is Some(10)
let y = if true {}; # y is Some(Void)
ley z = if false {10}; # z is None
```  
