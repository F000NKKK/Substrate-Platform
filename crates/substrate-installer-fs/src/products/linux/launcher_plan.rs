//! The launcher entry of a program, decided and rendered without touching the disk.

use std::path::{Path, PathBuf};

use substrate_installer_domain::ids::ProductId;

use super::launcher_entry::LauncherEntry;

/// The file extension of a launcher entry.
const ENTRY_EXTENSION: &str = "desktop";

/// What to write to list a program among the desktop's applications: the
/// entry's path and its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherPlan {
    /// Where the entry goes (`<entries>/<product id>.desktop`).
    pub path: PathBuf,
    /// The entry's text, in the Desktop Entry format.
    pub text: String,
}

/// Plans the launcher entry of `entry` in the `entries` folder.
#[must_use]
pub fn plan_launcher(entries: &Path, entry: &LauncherEntry<'_>) -> LauncherPlan {
    let mut lines = vec![
        "[Desktop Entry]".to_owned(),
        "Type=Application".to_owned(),
        format!("Name={}", one_line(entry.name.as_str())),
    ];
    if let Some(comment) = entry.comment {
        lines.push(format!("Comment={}", one_line(comment.as_str())));
    }
    lines.push(format!("Exec={} %U", exec_path(entry.program)));
    if let Some(icon) = entry.icon {
        lines.push(format!("Icon={}", one_line(&icon.to_string_lossy())));
    }
    lines.push("Terminal=false".to_owned());
    lines.push("Categories=Development;".to_owned());
    lines.push(String::new());
    LauncherPlan {
        path: entry_path(entries, entry.id),
        text: lines.join("\n"),
    }
}

/// Where the entry of the product `id` goes in the `entries` folder.
pub(super) fn entry_path(entries: &Path, id: &ProductId) -> PathBuf {
    entries.join(format!("{id}.{ENTRY_EXTENSION}"))
}

/// A value on one line: a line break would start another key.
fn one_line(text: &str) -> String {
    text.replace(['\n', '\r'], " ")
}

/// The program quoted for an `Exec` key: inside quotes `"`, `` ` ``, `$` and
/// `\` are escaped by a backslash, and `%` is doubled so it is not a field code.
fn exec_path(program: &Path) -> String {
    let mut quoted = String::from("\"");
    for c in program.to_string_lossy().chars() {
        match c {
            '"' | '`' | '$' | '\\' => {
                quoted.push('\\');
                quoted.push(c);
            }
            '%' => quoted.push_str("%%"),
            '\n' | '\r' => quoted.push(' '),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

#[cfg(test)]
mod tests {
    use substrate_installer_domain::texts::{Description, Label};

    use super::*;

    fn plan(program: &str, icon: Option<&Path>, comment: Option<&str>) -> LauncherPlan {
        let id = ProductId::new("yog-idle").unwrap();
        let name = Label::new("Yog IDLE").unwrap();
        let comment = comment.map(|text| Description::new(text).unwrap());
        let entry = LauncherEntry {
            id: &id,
            name: &name,
            comment: comment.as_ref(),
            program: Path::new(program),
            icon,
        };
        plan_launcher(Path::new("/apps"), &entry)
    }

    #[test]
    fn the_entry_is_named_after_the_product_and_lists_the_program() {
        let plan = plan("/home/u/.yog/p/yog-idle.AppImage", None, Some("An IDE"));
        assert_eq!(plan.path, Path::new("/apps/yog-idle.desktop"));
        assert_eq!(
            plan.text,
            "[Desktop Entry]\nType=Application\nName=Yog IDLE\nComment=An IDE\n\
             Exec=\"/home/u/.yog/p/yog-idle.AppImage\" %U\nTerminal=false\n\
             Categories=Development;\n"
        );
    }

    #[test]
    fn an_icon_is_listed_and_a_comment_is_optional() {
        let plan = plan("/p/x", Some(Path::new("/p/icon.png")), None);
        assert!(plan.text.contains("\nIcon=/p/icon.png\n"));
        assert!(!plan.text.contains("Comment="));
    }

    #[test]
    fn a_path_with_special_characters_is_quoted_for_exec() {
        let plan = plan("/p/a \"b\" $c `d` 50%\\e", None, None);
        assert!(
            plan.text
                .contains("Exec=\"/p/a \\\"b\\\" \\$c \\`d\\` 50%%\\\\e\" %U")
        );
    }
}
