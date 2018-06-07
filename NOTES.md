### Options and null

```
"I call it my billion-dollar mistake. It was the invention of the null
reference in 1965. At that time, I was designing the first comprehensive type
system for references in an object oriented language (ALGOL W). My goal was to
ensure that all use of references should be absolutely safe, with checking
performed automatically by the compiler. But I couldn't resist the temptation
to put in a null reference, simply because it was so easy to implement. This
has led to innumerable errors, vulnerabilities, and system crashes, which have
probably caused a billion dollars of pain and damage in the last forty years."

  -- Tony Hoare
```

### Exceptions and Errors

```
Almost anything in software can be implemented, sold, and even used given enough determination. There is nothing a mere scientist can say that will stand against the flood of a hundred million dollars. But there is one quality that cannot be purchased in this way - and that is reliability. The price of reliability is the pursuit of the utmost simplicity.

[...]

Gradually these objectives have been sacrificed in favor of power,
supposedly achieved by a plethora of features and notational conventions, many of them unnecessary
and some of them, like exception handling, even dangerous.

  -- Tony Hoare
```

### Option Map Example

```rust
fn add_one(val: &Option<u32>) -> Option<u32> {
    val.map(|i| i + 1)
}

fn main() {
    [Some(1), Some(2), None, None, Some(5)]
        .iter()
        .map(add_one)
        .for_each(|val| println!("{:?}", val));
}
```

### Collecting Results Example


