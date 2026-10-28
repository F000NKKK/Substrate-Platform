//! The macro behind every validated text newtype of this crate.

/// Defines a text newtype whose constructor runs the rule `$rule` of
/// `text::rule` under the `TextKind::$kind`.
macro_rules! define_text {
    ($(#[$meta:meta])* $name:ident, $kind:ident, $rule:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Makes the value from `text`.
            ///
            /// # Errors
            ///
            /// [`TextError`](crate::texts::TextError) when the text breaks the rule of its kind.
            pub fn new(text: impl Into<String>) -> Result<Self, $crate::texts::TextError> {
                let text = text.into();
                $crate::texts::rule::$rule($crate::texts::TextKind::$kind, &text)?;
                Ok(Self(text))
            }

            /// The text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::texts::TextError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::new(text)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

pub(crate) use define_text;
