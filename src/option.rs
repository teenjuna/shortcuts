use core::fmt::Debug;

/// Shortcut methods for [Option].
///
/// [Option]: core::option::Option
pub trait OptionShortcuts<T> {
    /// Shortcut for [Option::unwrap].
    fn un(self) -> T;
    /// Shortcut for yet-unstable [Option::unwrap_none](https://github.com/rust-lang/rust/issues/62633).
    fn un_none(self)
    where
        T: Debug;
    /// Shortcut for [Option::expect].
    fn ex(self, msg: &str) -> T;
    /// Shortcut for yet-unstable [Option::expect_none](https://github.com/rust-lang/rust/issues/62633).
    fn ex_none(self, msg: &str)
    where
        T: Debug;
}

impl<T> OptionShortcuts<T> for Option<T> {
    fn un(self) -> T {
        self.unwrap()
    }
    fn un_none(self)
    where
        T: Debug,
    {
        // https://github.com/rust-lang/rust/issues/62633
        if let Some(v) = self {
            panic!("called `Option::unwrap_none()` on a `Some` value: {:?}", v);
        }
    }
    fn ex(self, msg: &str) -> T {
        self.expect(msg)
    }
    fn ex_none(self, msg: &str)
    where
        T: Debug,
    {
        // https://github.com/rust-lang/rust/issues/62633
        if let Some(v) = self {
            panic!("{}: {:?}", msg, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OptionShortcuts;

    const NONE: Option<()> = core::option::Option::None;
    const SOME: Option<()> = core::option::Option::Some(());

    #[test]
    fn un_with_some() {
        SOME.un();
    }

    #[test]
    #[should_panic]
    fn un_with_none() {
        NONE.un();
    }

    #[test]
    fn un_none_with_none() {
        NONE.un_none();
    }

    #[test]
    #[should_panic]
    fn un_none_with_some() {
        SOME.un_none();
    }

    #[test]
    fn ex_with_some() {
        SOME.ex("msg");
    }

    #[test]
    #[should_panic]
    fn ex_with_none() {
        NONE.ex("msg");
    }

    #[test]
    fn ex_none_with_none() {
        NONE.ex_none("msg");
    }

    #[test]
    #[should_panic]
    fn ex_none_with_some() {
        SOME.ex_none("msg");
    }
}
