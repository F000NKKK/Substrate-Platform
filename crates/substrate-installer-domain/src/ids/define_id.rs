//! The macro behind every id newtype.

/// Defines an id newtype checked under the `IdKind::$kind`.
macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $kind:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Makes the id from `text`.
            ///
            /// # Errors
            ///
            /// [`IdError`](crate::ids::IdError) when `text` is not a valid id.
            pub fn new(text: impl Into<String>) -> Result<Self, $crate::ids::IdError> {
                let text = text.into();
                $crate::ids::validate::check($crate::ids::IdKind::$kind, &text)?;
                Ok(Self(text))
            }

            /// The id as text.
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
            type Err = $crate::ids::IdError;

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

pub(crate) use define_id;
