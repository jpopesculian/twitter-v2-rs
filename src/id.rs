use std::borrow::Cow;
use std::fmt::Display;

pub trait ToId: Display {}

impl ToId for u64 {}

impl<'a> ToId for &'a u64 {}

impl ToId for String {}

impl<'a> ToId for &'a str {}

impl<'a> ToId for Cow<'a, str> {}
