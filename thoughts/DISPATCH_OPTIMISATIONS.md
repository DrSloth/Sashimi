There are many places where you would have to change a lot of code because it is really hard to 
know which implementation strategy is the fastest. One of this things is static vs dynamic 
dispatch of generics. Generally static dispatch is the most performant speedwise in most cases.
However this isn't true for all cases, and static dispatch leads to bloated binaries.
It takes a lot of effort to change static dispatch to dynamic dispatch and optimising accordingly.
Getting it right is hard and very repetitive, because it is a rather repetitive task it is trivial
to change for the compiler. Thus making it controllable through compiler options is the best way
of giving control to the user. You should never have to choose between flexibility/readability 
and code performance, all software should be as performant as possible.
