
#[test]
fn vecset_test() {
    use spudbox::vecset::*;

    let mut set = VecSet::new();

    let values = [1,2,3,4,5,6,7,8,9,0,2,1,2,3,4,5,6,7,8,9,0,2,3,2,2,2,2,0,2,2,2,2,0,3,3,4,45,6,6,4,3,2,2,4,0,5,6,0,3,0,2,0,];

    set.extend(values);

    set.insert(2, 33);
    set.remove_index(5);
    set.remove_value(&7);
    set.extend(values);
    set.pop();
    set.push(7);

    assert_eq!(format!("{}", set), "{1, 2, 33, 3, 4, 6, 8, 9, 0, 45, 5, 7}")
}