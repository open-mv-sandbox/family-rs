use family::{
    any::{AnyMember, AnyOption, FamilyMember},
    utils::{FamilyT, MemberT},
};

struct MemberA;
type FamilyA = FamilyT<MemberA>;

struct MemberB;
type FamilyB = FamilyT<MemberB>;

#[test]
fn box_can_downcast_valid() {
    let member = MemberT(MemberA);
    let value: Box<dyn AnyMember> = Box::new(FamilyMember::<FamilyA>(member));

    let result = value.downcast::<FamilyA>();

    assert!(result.is_some());
}

#[test]
fn box_cant_downcast_invalid() {
    let member = MemberT(MemberB);
    let value: Box<dyn AnyMember> = Box::new(FamilyMember::<FamilyB>(member));

    let result = value.downcast::<FamilyA>();

    assert!(result.is_none());
}

#[test]
fn option_can_downcast_valid() {
    let member = MemberT(MemberA);
    let mut option = Some(FamilyMember::<FamilyA>(member));
    let option: &mut dyn AnyOption = &mut option;

    let result = option.downcast::<FamilyA>();

    assert!(result.is_some());
}

#[test]
fn option_cant_downcast_invalid() {
    let member = MemberT(MemberB);
    let mut option = Some(FamilyMember::<FamilyB>(member));
    let option: &mut dyn AnyOption = &mut option;

    let result = option.downcast::<FamilyA>();

    assert!(result.is_none());
}
