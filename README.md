Implementation of the `calc` animal in Andrej Bauer's [Progamming Language Zoo](http://andrej.com/plzoo/).

Parsing is done with recursive descent using operator precedence climbing. Based on the
C++ implementation from the [LLVM tutorial](http://llvm.org/docs/tutorial/LangImpl2.html) for an example language.
Inspiration also drawn from this [python implementation](http://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing).

Does simple integer calculations with correct operator precedence, and supports negation and parentheses. Hit Ctrl-C to end.

```
$ cargo run
     Running `target/debug/rust-zoo-calc`
Welcome to calc!
     > 5 + 6 * 7
47
     > (5+6)*7
77
     > -8 * -9
72
     > (-8)
-8
     > 4/3
1
     > ^Cfish: Job 1, 'cargo run' terminated by signal SIGINT (Quit request from job control (^C))
$
```
