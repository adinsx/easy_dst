#![feature(const_str_from_utf8)]

use core::{
    mem::transmute,
    str::{from_utf8, from_utf8_unchecked},
};
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::Deref,
};
/*
named!(const | static $ident = ($literal as $ty | $ty($tt)), $literal)
or
named!($literal as $ty | $ty($tt)), $literal)
*/

//fn stuff(input: TokenStream) -> TokenStream {}

#[derive(Clone, Hash)]
pub(crate) struct SizedUnsized<S, U: ?Sized> {
    sized_data: S,
    unsized_data: U,
}
#[derive(Clone, Hash)]
pub struct Named<'a, T>(&'a SizedUnsized<T, [u8]>);

impl<'a, T> Deref for Named<'_, T> {
    type Target = T;

    fn deref(&self) -> &'_ Self::Target {
        &self.0.sized_data
    }
}

impl<'a, T: Debug> Debug for Named<'_, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(type_name::<Self>())
            .field(&**self)
            .field(&self.name())
            .finish()
    }
}

impl<'a, T> Named<'_, T> {
    pub const fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0.unsized_data) }
    }

    pub const fn new<const N: usize>(init: &(T, [u8; N])) -> Named<'_, T> {
        assert!(from_utf8(&init.1).is_ok());
        Named(unsafe { transmute::<_, &SizedUnsized<_, [_; N]>>(init) })
    }
}

// todo: Improve initialization. Probably via macro. impl all the stuff that can be. fix derives if need be

#[macro_export]
macro_rules! named {
    (@strip_as $lhs:tt $(as)? $rhs:tt) => {$lhs $rhs};

    (@type_of _:literal as $datatype:ty) => {$datatype};

    (@type_of $datatype:ty) => {$datatype};

    (@const_type $datatype:ty, $name:literal) => {&($datatype, [u8; $name.len()])};

    (@named_expr $data: tt as $datatype:ty, $name:literal) => {
        {
            #[allow(clippy::unnecessary_cast)]
            const TMP: named!(@const_type $datatype, $name) = &($data as $datatype, *$name);
            Named::new(TMP)
        }
    };

    (@named_expr $datatype:tt $data:tt, $name:literal) => {
        {
            const TMP: named!(@const_type $datatype, $name) = &($datatype $data, *$name);
            Named::new(TMP)
        }
    };
    //($data: tt as $datatype:ty, $name:literal)
    /*(const $var: ident = $data: expr, $name: literal) => {
            const $var: Named<named!(@type_of $data)> = named!(@named_expr $data, $name);
        };*/

    (const $var: ident = ($datatype:tt $data:tt, $name:literal)) => {
        const $var: Named<$datatype> = named!(@named_expr $data as $datatype, $name);
    };
}

#[cfg(test)]
mod tests;
