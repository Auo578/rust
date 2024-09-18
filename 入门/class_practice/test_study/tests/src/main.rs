use test_study::add;

#[test]

fn add_two_and_two() {
    assert_eq!(4, add(2, 2));
}

#[test]
fn add_three_and_two() {
    assert_eq!(5, add(2, 3));
}


