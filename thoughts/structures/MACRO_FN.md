A `macro fn` can be called just like any other function. Either it is called as a normal 
function or as a macro which first generates optimized code. 
Macro fns may not take untyped meta types. Macro fns have a function fallback which will be
used if no macro matches or the function is used as a function pointer.

