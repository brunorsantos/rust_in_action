Four general strategies can help with ownership issues

- Use reference where full ownership is not required
- Duplicate the value
- Refactor code to reduce the number of long-lived objects
- Wrap data in a type designed to assist with movement issues


## Summary

- A value's owner is reponsible for cleaning up after that valeu when its lifetime ends.
- A value's lifetime is the period when accessing that valeu is valid behavior. Attempting to access a value after its lifetime had expired leads to code that won't compile
- To borrow a value means to access that value
- Of you find that the borrow checker wont allow your program to compile, several tactics are avaible to you. This often means that you need the rethink the design of your program
- Use shorted-lived values rather than values that stick aroud for a long time.
- Borrows can be read-only or read-write. Only one read-write borrow an exist at any one time.
- Duplicating a value can be a pragmatic  way to break an impasse with te borrow checker. To duplicate a value, implement Clone or Copy.
- It, possible to opt in to reference counting semantics thorough RC<T>
- Rust supports a feature known as interior mutability, which enables type to present themselves as immutable even though their values can change over time.
- 
