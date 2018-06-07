type MyResult = Result<Vec<u32>, &'static str>;

fn add_one(val: &Option<u32>) -> Option<u32> {
    val.map(|i| i + 1)
}

fn less_than_five(val: Option<u32>) -> Result<u32, &'static str> {
    match val {
        Some(i) if i < 5 => Ok(i),
        _ => Err("Value does not exist, or value is >= 5!"),
    }
}

fn main() {
    let input: &[Option<u32>] = &[Some(1), Some(2), None, None, Some(5)];

    let res = input
        .iter()
        .map(add_one)
        .map(less_than_five)
        .collect::<MyResult>();

    let print_elems = |vals: Vec<u32>| vals.iter().for_each(|i| println!("{:?}", i));

    match res {
        Ok(vals) => print_elems(vals),
        Err(e) => println!("Something went wrong!"),
    }
}
