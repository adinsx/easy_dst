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
// todo: Improve initialization. Probably via macro. impl all the stuff that can be. fix derives if need be

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

#[macro_export]
macro_rules! named {
    (@tmp_type $sized_type:ty, $name:literal) => {
        &($sized_type, [u8; $name.len()])
    };

    (const $var: ident = <[$sized_type: ty; _]>($sized_data: tt, $name: literal)) =>{
        const $var: Named<[$sized_type; $sized_data.len()]> = {
            const TMP: named!(@tmp_type [$sized_type; $sized_data.len()], $name) = &($sized_data, *$name);
            Named::new(TMP)
        };
    };

    (const $var: ident = <$sized_type: ty>($sized_data: expr, $name: literal)) => {
        const $var: Named<$sized_type> = {
    const TMP: named!(@tmp_type $sized_type, $name) = &($sized_data, *$name);
            Named::new(TMP)
        };
    };

    (const $var: ident = ($sized_type: tt $sized_data: tt, $name: literal)) => {
        const $var: Named<$sized_type> = {
    const TMP: named!(@tmp_type $sized_type, $name) = &($sized_type $sized_data, *$name);
            Named::new(TMP)
        };
    };
}

#[cfg(test)]
mod tests;
