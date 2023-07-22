A strategy quite common the deal with ownership constraints is to use wrapper types, that allow more flexibilty but with  costs for  rust deal with safety guarantees.

It a way theat rust programmers opt ofr having a garbage collector.

RC(Reference counter) provides shared ownership of a T. Shared Ownership prevents T from being removed from memory until every owner is removed.

RC<T> does not allow muttation. For that we need to use RC<RefCell<T>>. That is a type that has interior mutability.




