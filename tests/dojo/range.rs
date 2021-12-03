use rust_guide::dojo::range_demo::range::Range;

#[test]
fn should_create_e2_6() {
    let range = Range::init("[2,6)");
    assert_eq!(range.show(), "[2, 6)");
}

#[test]
fn should_create_e2_6_2() {
    let range = Range::init(" (2,6]");
    assert_eq!(range.show(), "(2, 6]");
}

#[test]
fn should_create_2_6_3() {
    let range = Range::init(" (2, 6 )");
    assert_eq!(range.show(), "(2, 6)");
}

#[test]
fn should_e2_6_and_3_7_return_e2_7() {
    let mut range = Range::init(" [2, 6 )");
    range.and_default("(3,7)");
    assert_eq!(range.show(), "[2, 6) ∪ (3, 7)");
}

#[test]
fn should_e2_5_not_overlap_e7_10() {
    let range = Range::init("[2,5)");
    let range_2 = Range::init("[7,10)");
    assert_eq!(range.overlaps_range(&range_2), false);
}