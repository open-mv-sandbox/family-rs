use family::any::{AnyMember, FamilyMember};
use family_derive::Member;

#[derive(Member)]
struct Borrowed<'a>(&'a str);

fn main() {
    let value = "Family".to_string();
    let value = Borrowed(value.as_str());
    let boxed = Box::new(FamilyMember::<BorrowedF>(value));
    say_hello(boxed);
}

fn say_hello<'a>(value: Box<dyn AnyMember<'a> + 'a>) {
    let value: Borrowed = value.downcast::<BorrowedF>().expect("downcast failed").0;
    println!("Hello, {}!", value.0);
}
