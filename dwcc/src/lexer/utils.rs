fn is_numeric(v: &str) -> bool {
    if v == "" {
        return false;
    }
    for c in v.chars().into_iter() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn is_alphanumeric(v: &str) -> bool {
    if v == "" {
        return false;
    }
    for c in v.chars().into_iter() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    true
}

fn is_special_char(v: &str) -> bool {
    if v == "" {
        return false;
    }
    let specialchar = "{}();";
    for c in v.chars().into_iter() {
        if !specialchar.contains(c) {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emptystring_not_numeric() {
        assert!(!is_numeric(""))
    }

    #[test]
    fn test_space_not_numeric() {
        assert!(!is_numeric(" "))
    }

    #[test]
    fn test_alpha_not_numeric() {
        assert!(!is_numeric("a"))
    }

    #[test]
    fn test_emptystring_not_alphanumeric() {
        assert!(!is_alphanumeric(""))
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("1"))
    }

    #[test]
    fn test_not_alphanumeric() {
        assert!(!is_numeric(" "))
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_numeric("1"))
    }
}
