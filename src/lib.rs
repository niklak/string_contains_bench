


pub fn css_class_contains_split(class_string: &str, target_class: &str) -> bool {
    let classes: Vec<&str> = class_string.split_whitespace().collect();
    classes.contains(&target_class)
}


pub fn css_class_contains_any(classes: &str, target_class: &str) -> bool {
    classes.split_whitespace().any(|c| c == target_class)
}


pub fn css_class_contains_substring(classes: &str, target_class: &str) -> bool {
    let class_str = format!(" {} ", classes);
    let target = format!(" {} ", target_class);
    class_str.contains(&target)
}
