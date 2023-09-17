# iron-macros
Macros that add more features to rust.

## About
This is meant to add more functionality to rust. Adding macros for:
    - `define`, the c keyword that makes a variable into a macro and makes it faster
    - `c_loop`, a macro that allows you to easily code c loops.
    - more on the way

### define
The `define` macro is taken from c/c++.
```rust
define!(width, 500);

assert_eq!(width!(), 500); 
``` 

### C_loop 
Allows you to easily code c loops.
```rust
c_loop!(i = 0; i > 5; i += 1; {
    println!("{}", i); // prints 0-5
})
```