//! Dynamic type casting support for families.
//!
//! Rust std's `Any` type is limited to only work on `'static` types.
//! The limitations added by the family pattern allow it to do downcasting with borrowed types.
//!
//! > ⚠️ This module uses unsafe code that has not been properly audited for soundness.

use std::any::TypeId;

use crate::Family;

/// Marker newtype for a specific family's member.
///
/// Since members can be part of multiple families, this marker restricts to just one for dynamic
/// type casting.
pub struct FamilyMember<'a, F>(pub F::Member<'a>)
where
    F: Family;

/// Dynamic upcast type identification for `FamilyMember`.
///
/// # Safety
/// While calling this is safe, implementing this isn't, as `family_id` is used to check
/// downcasting.
///
/// ```compile_fail
/// # use family::{*, any::*};
/// # use std::{cell::Cell, rc::Rc};
/// struct Foo;
/// impl Family for Foo { type Member<'a> = Rc<Cell<&'a str>>; }
/// impl Member<Foo> for Rc<Cell<&str>> {}
///
/// let s = Rc::new(Cell::new("Hello, world!"));
/// let member: Box<dyn AnyMember> = Box::new(FamilyMember::<Foo>(Rc::clone(&s)));
/// let s2 = member.downcast::<Foo>().unwrap().0;
///
/// // Borrow is set with something only briefly available
/// s2.set(&"Hello, Rust!".to_string());
///
/// // Borrow is used, extending lifetime requirement, and correctly failing to compile
/// println!("{}", s.get());
/// ```
pub unsafe trait AnyMember<'a> {
    /// Get the `TypeId` of the family this is a member of.
    fn family_id(&self) -> TypeId;
}

unsafe impl<'a, F> AnyMember<'a> for FamilyMember<'a, F>
where
    F: Family,
{
    fn family_id(&self) -> TypeId {
        TypeId::of::<F>()
    }
}

impl<'a> dyn AnyMember<'a> + 'a {
    /// Downcast a box containing a `FamilyMember`, to an instance with compatible lifetime.
    pub fn downcast<F>(self: Box<Self>) -> Option<Box<FamilyMember<'a, F>>>
    where
        F: Family,
    {
        // Check that the family ID matches
        if TypeId::of::<F>() != self.family_id() {
            return None;
        }

        // The ID matches, so this *should* be sound
        // Lifetimes are enforced by 'a
        let raw = Box::into_raw(self) as *mut FamilyMember<'a, F>;
        let casted = unsafe { Box::from_raw(raw) };

        Some(casted)
    }
}

/// Dynamic upcast type identification for `Option`s containing `FamilyMember`.
///
/// # Safety
/// While calling this is safe, implementing this isn't, as `family_id` is used to check
/// downcasting.
///
/// ```compile_fail
/// # use family::{*, any::*};
/// # use std::cell::Cell;
/// struct Foo;
/// impl Family for Foo { type Member<'a> = Cell<&'a str>; }
/// impl Member<Foo> for Cell<&str> {}
///
/// let s = Cell::new("Hello, world!");
/// let mut o = Some(FamilyMember::<Foo>(s));
/// let any: &mut dyn AnyOption = &mut o;
/// let o2 = any.downcast::<Foo>().unwrap();
///
/// // Borrow is set with something only briefly available
/// o2.map(|v| v.0.set(&"Hello, Rust!".to_string()));
///
/// // Borrow is used, extending lifetime requirement, and correctly failing to compile
/// println!("{}", o.unwrap().0.get());
/// ```
pub unsafe trait AnyOption<'a> {
    /// Get the `TypeId` of the family this is a member of.
    fn family_id(&self) -> TypeId;
}

unsafe impl<'a, F> AnyOption<'a> for Option<FamilyMember<'a, F>>
where
    F: Family,
{
    fn family_id(&self) -> TypeId {
        TypeId::of::<F>()
    }
}

impl<'a: 'b, 'b> dyn AnyOption<'a> + 'b {
    /// Downcast an option containing a `FamilyMember`, to an instance with compatible lifetime.
    pub fn downcast<'c, F>(&'c mut self) -> Option<&'c mut Option<FamilyMember<'a, F>>>
    where
        F: Family,
    {
        // Check that the family ID matches
        if TypeId::of::<F>() != self.family_id() {
            return None;
        }

        // The ID matches, so this *should* be sound
        // Lifetimes are enforced by 'a
        let raw = self as *mut Self as *mut Option<FamilyMember<'a, F>>;
        let casted = unsafe { &mut *raw };

        Some(casted)
    }
}
