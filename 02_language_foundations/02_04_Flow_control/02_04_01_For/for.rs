

fn main(){
    
// Even if container variable remains within local scope, the lifetime has ended. So container cannot be reused    
for item in container {
    // ...
}

// Using a reference we can use a reference
for item in &container {
    // ...
}

// If need to modify some item, we can use a mutable reference
for item in &mut container {
    // ...
}

}