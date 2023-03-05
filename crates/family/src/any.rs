//! Dynamic type casting support for families.
//!
//! Rust std's `Any` type is limited to only work on `'static` types.
//! The limitations added by the family pattern allow it to do downcasting with borrowed types.

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
pub unsafe trait AnyMember {
    /// Get the `TypeId` of the family this is a member of.
    fn family_id(&self) -> TypeId;
}

unsafe impl<'a, F> AnyMember for FamilyMember<'a, F>
where
    F: Family,
{
    fn family_id(&self) -> TypeId {
        TypeId::of::<F>()
    }
}

impl<'a> dyn 'a + AnyMember {
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

/// Dynamic upcast type identification for `FamilyMember`.
///
/// # Safety
/// While calling this is safe, implementing this isn't, as `family_id` is used to check
/// downcasting.
pub unsafe trait AnyOption {
    fn family_id(&self) -> TypeId;
}

unsafe impl<'a, F> AnyOption for Option<FamilyMember<'a, F>>
where
    F: Family,
{
    fn family_id(&self) -> TypeId {
        TypeId::of::<F>()
    }
}

impl<'a> dyn 'a + AnyOption {
    /// Downcast an option containing a `FamilyMember`, to an instance with compatible lifetime.
    pub fn downcast<'b, F>(&'b mut self) -> Option<&'b mut Option<FamilyMember<'a, F>>>
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
