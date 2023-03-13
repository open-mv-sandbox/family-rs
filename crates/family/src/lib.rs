#![no_std]

//! I can't believe it's not HKT!
//!
//! A small family pattern implementation, implementing "associated type constructors".
//! Additionally provides some dynamic casting utilities that make use of families and members.
//!
//! See this post for more information:
//! <http://smallcultfollowing.com/babysteps/blog/2016/11/03/associated-type-constructors-part-2-family-traits/>

extern crate alloc;

use core::any::Any;

pub mod any;
pub mod utils;

pub use family_derive::Member;

/// Family pattern family interface.
pub trait Family: Any + Sized {
    type Member<'a>: Member<Self>;
}

/// Family pattern member interface.
pub trait Member<F>
where
    F: Family,
{
}
