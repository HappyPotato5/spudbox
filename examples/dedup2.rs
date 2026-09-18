use spudbox::vecset::*;

fn main() {
    let mut vecset = vecset!["My", "name", "name", "name", "is", "Potato", "Potato", "I", "am", "am", "am", "a", "Robot"];

    vecset.insert(7, "super");
    vecset.push("Robot");
    vecset.push("NOOOO");
    vecset.pop();

    assert_eq!(format!("{}", vecset), r#"{My, name, is, Potato, I, am, a, super, Robot}"#);
}