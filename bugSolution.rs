fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, use the vector's indexing.
    v[0] = 10;
    println!("v: {:?}", v);

    // Or alternatively, if you absolutely need to work with a mutable slice
    // Ensure the borrow of v is within the scope
    {
      let slice = &mut v[..];
      slice[0] = 100;
    }
    println!("v: {:?}", v);
} 