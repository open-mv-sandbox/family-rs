use family::Member;

#[derive(Member)]
struct SimpleOwned {}

#[derive(Member)]
struct SimpleBorrowed<'a> {
    _a: &'a u32,
}

//#[derive(Member)]
//struct GenericOwned<A> {
//    _a: A,
//}

fn main() {}
