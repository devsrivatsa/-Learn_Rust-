fn main() {
    //1. aggregators

    //fold: kinda list comprehension with optional conditions
    let blocks = vec!['W','B','W','W','B','B'];
    let mut min_ops = blocks.iter().fold(0, |acc, i| if *i == 'W' {acc+1} else {0});
    println!("{}", min_ops);
}