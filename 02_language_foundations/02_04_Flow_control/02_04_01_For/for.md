

    
Even if container variable remains within local scope, the lifetime has ended. So container cannot be reused    
```rust
for item in container {
    // ...
}
```


Using a reference we can use a reference
```rust
for item in &container {
    // ...
}
```


If need to modify some item, we can use a mutable reference
```rust
for item in &mut container {
    // ...
}
```


