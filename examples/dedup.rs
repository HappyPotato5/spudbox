use spudbox::ordered_set::*;

fn main() {
    let mut ordset = ordset!["My", "name", "name", "name", "is", "Potato", "Potato", "I", "am", "am", "am", "a", "Robot"];

    ordset.insert(7, "super");
    ordset.push("Robot");
    ordset.push("NOOOO");
    ordset.pop();

    assert_eq!(format!("{}", ordset), r#"{My, name, is, Potato, I, am, a, super, Robot}"#);
}