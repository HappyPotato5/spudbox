use spudbox::vecset;

fn main() {
    let vecset = vecset!["my", "name", "name", "name", "is", "Potato", "Potato", "I", "Am", "Am", "Am", "A", "Robot"];

    assert_eq!(format!("{}", vecset), r#"{my, name, is, Potato, I, Am, A, Robot}"#);
}