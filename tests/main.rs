use enum_all_variants::AllVariants;

#[derive(AllVariants, Debug, PartialEq, Eq)]
enum Zero {}

#[derive(AllVariants, Debug, PartialEq, Eq)]
enum One {
    Five = 5,
}

#[derive(AllVariants, Debug, PartialEq, Eq)]
enum Many {
    First = 1000,
    Second,
    Third = 100,
}

#[test]
fn test_zero() {
    assert_eq!(Zero::all_variants(), &[]);
}

#[test]
fn test_one() {
    assert_eq!(One::all_variants(), &[One::Five]);
}

#[test]
fn test_many() {
    use Many::*;
    assert_eq!(Many::all_variants(), &[First, Second, Third]);
}
