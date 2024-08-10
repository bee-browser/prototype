#[macro_export]
macro_rules! delegate_all {
    ($type:ty => $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($type:ty => $target:ty, $lifetime:lifetime) => {
        impl<$lifetime> std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$lifetime> std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($lhs:expr, $rhs:expr) => {
        std::assert_eq!($lhs, $rhs, "{} == {}", std::stringify!($lhs), std::stringify!($rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::assert_eq!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($lhs:expr, $rhs:expr) => {
        std::assert_ne!($lhs, $rhs, "{} != {}", std::stringify!($lhs), std::stringify!($rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::assert_ne!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! debug_assert_eq {
    ($lhs:expr, $rhs:expr) => {
        std::debug_assert_eq!($lhs, $rhs, "{} == {}", std::stringify!($lhs), std::stringify!($rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::debug_assert_eq!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! debug_assert_ne {
    ($lhs:expr, $rhs:expr) => {
        std::debug_assert_ne!($lhs, $rhs, "{} != {}", std::stringify!($lhs), std::stringify!($rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::debug_assert_ne!($lhs, $rhs, $($t)*);
    };
}

pub use delegate_all;
pub use assert_eq;
pub use assert_ne;
pub use debug_assert_eq;
pub use debug_assert_ne;
