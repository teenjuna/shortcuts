use core::fmt::Debug;

/// Shortcut methods for [TryFrom].
///
/// [TryFrom]: core::convert::TryFrom
/// [Result]: core::result::Result
pub trait TryFromShortcuts<T> {
    /// Shortcut for [TryFrom::try_from] + [Result::unwrap].
    fn force_from(t: T) -> Self;
}

impl<T, E, I> TryFromShortcuts<T> for I
where
    I: TryFrom<T, Error = E>,
    E: Debug,
{
    fn force_from(t: T) -> Self {
        I::try_from(t).unwrap()
    }
}

/// Shortcut methods for [TryInto].
///
/// [TryInto]: core::convert::TryInto
/// [Result]: core::result::Result
pub trait TryIntoShortcuts<T> {
    /// Shortcut for [TryInto::try_into] + [Result::unwrap].
    fn force_into(self) -> T;
}

impl<T, E, I> TryIntoShortcuts<T> for I
where
    I: TryInto<T, Error = E>,
    E: Debug,
{
    fn force_into(self) -> T {
        self.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{TryFromShortcuts, TryIntoShortcuts};

    #[test]
    fn force_from_valid() {
        u32::force_from(1i32);
    }

    #[test]
    #[should_panic]
    fn force_from_invalid() {
        u32::force_from(-1i32);
    }

    #[test]
    fn force_into_valid() {
        let _: u32 = 1i32.force_into();
    }

    #[test]
    #[should_panic]
    fn force_into_invalid() {
        let _: u32 = (-1i32).force_into();
    }
}
