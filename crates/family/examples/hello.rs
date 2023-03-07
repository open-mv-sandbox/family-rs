use family::any::{AnyMember, FamilyMember};
use family_derive::Member;

#[derive(Member)]
struct Borrowed<'a>(&'a mut String);

#[derive(Member)]
struct Owned(String);

fn main() {
    let mut string = "Family".to_string();

    let value = Borrowed(&mut string);
    let boxed = Box::new(FamilyMember::<BorrowedF>(value));
    say_hello_borrowed(boxed);

    let value = Owned(string);
    let boxed = Box::new(FamilyMember::<Owned>(value));
    say_hello_owned(boxed);
}

fn say_hello_borrowed<'a>(value: Box<dyn AnyMember<'a> + 'a>) {
    let value: Borrowed = value.downcast::<BorrowedF>().expect("downcast failed").0;
    println!("Hello, {}!", value.0);

    // Since it's a mutable borrow, we can change it too
    *value.0 = "Changed".to_string();
}

fn say_hello_owned<'a>(value: Box<dyn AnyMember<'a> + 'a>) {
    let value: Owned = value.downcast::<Owned>().expect("downcast failed").0;
    println!("Hello, {}!", value.0);
}
