use test_study::add;

#[test]
fn example_2_3(){
    let sum = add(2, 3);
    assert_eq!(5,sum);
}
fn main() {
    let a = 5;
    let b = 6;
    let c = add(a, b);
    println!("The sum of {} and {} is {}", a, b, c);
    assert_eq!(c,11);
}