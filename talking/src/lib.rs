fn is_cap(t: &str) -> bool {
    let has_letters = t.chars().any(|c| c.is_alphabetic());
    has_letters && t.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}


pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    if is_cap(trimmed) && trimmed.ends_with('?') {
        return "Quiet, I am thinking!";
    }

    if trimmed.ends_with('?') {
        return "Sure.";
    }

    if is_cap(trimmed) {
        return "There is no need to yell, calm down!";
    }

    "Interesting"
}