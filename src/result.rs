use core::fmt::Debug;

/// Shortcut methods for [Result].
///
/// [Result]: core::result::Result
pub trait ResultShortcuts<T, E> {
    /// Shortcut for [Result::unwrap].
    fn un(self) -> T
    where
        E: Debug;
    /// Shortcut for [Result::unwrap_err].
    fn un_err(self) -> E
    where
        T: Debug;
    /// Shortcut for [Result::expect].
    fn ex(self, msg: &str) -> T
    where
        E: Debug;
    /// Shortcut for [Result::expect_err].
    fn ex_err(self, msg: &str) -> E
    where
        T: Debug;
}

impl<T, E> ResultShortcuts<T, E> for Result<T, E> {
    fn un(self) -> T
    where
        E: Debug,
    {
        self.unwrap()
    }
    fn un_err(self) -> E
    where
        T: Debug,
    {
        self.unwrap_err()
    }
    fn ex(self, msg: &str) -> T
    where
        E: Debug,
    {
        self.expect(msg)
    }
    fn ex_err(self, msg: &str) -> E
    where
        T: Debug,
    {
        self.expect_err(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::ResultShortcuts;

    const OK: Result<(), ()> = core::result::Result::Ok(());
    const ERR: Result<(), ()> = core::result::Result::Err(());

    #[test]
    fn un_with_ok() {
        OK.un();
    }

    #[test]
    #[should_panic]
    fn un_with_err() {
        ERR.un();
    }

    #[test]
    fn un_err_with_err() {
        ERR.un_err();
    }

    #[test]
    #[should_panic]
    fn un_err_with_ok() {
        OK.un_err();
    }

    #[test]
    fn ex_with_ok() {
        OK.ex("msg");
    }

    #[test]
    #[should_panic]
    fn ex_with_err() {
        ERR.ex("msg");
    }

    #[test]
    fn ex_err_with_err() {
        ERR.ex_err("msg");
    }

    #[test]
    #[should_panic]
    fn ex_err_with_ok() {
        OK.ex_err("msg");
    }
}
