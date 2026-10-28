//! An executable's file name here.

/// `name` with the operating system's executable suffix (`.exe` on Windows).
pub(crate) fn executable(name: &str) -> String {
    format!("{name}{}", std::env::consts::EXE_SUFFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_the_suffix_of_this_system() {
        assert_eq!(
            executable("cargo"),
            format!("cargo{}", std::env::consts::EXE_SUFFIX)
        );
    }
}
