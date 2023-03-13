//! Utilities for adding family functionality to types that don't implement `Family` and `Member`.
//!
//! These are primarily workarounds for a lack of HKT support in Rust.

use core::marker::PhantomData;

use crate::{Family, Member};

/// Generic family type for families of static members, where every member is the same.
pub struct FamilyT<T> {
    _t: PhantomData<T>,
}

impl<T: 'static> Family for FamilyT<T> {
    type Member<'a> = MemberT<T>;
}

/// Newtype wrapper for static members that don't themselves implement `Member`.
pub struct MemberT<T>(pub T);

impl<T: 'static> Member<FamilyT<T>> for MemberT<T> {}

impl<T> From<T> for MemberT<T> {
    fn from(value: T) -> Self {
        MemberT(value)
    }
}
