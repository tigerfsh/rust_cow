
use std::borrow::Cow;

// 1) A function rarely modifying the data
fn remove_whitespaces(s: &str) -> Cow<str> {
    // s.to_string().replace(' ', "")
    if s.contains(' ') {
        Cow::Owned(s.to_string().replace(' ', ""))
    } else {
        Cow::Borrowed(s)
    }
}

// 2) A struct optionally owning the data
struct User<'a> {
    first_name: Cow<'a, str>,
    last_name: Cow<'a, str>,
}

// we can construct both owned and borrowed version of the User struct
impl<'a> User<'a> {
    pub fn new_owned(first_name: String, last_name: String) -> User<'a> {
        User {
            first_name: Cow::Owned(first_name),
            last_name: Cow::Owned(last_name),
        }
    }

    pub fn new_borrowed(first_name:&'a str, last_name: &'a str) -> Self {
        Self { first_name: Cow::Borrowed(first_name), last_name: Cow::Borrowed(last_name) }
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}

// 3) A clone on write struct

// It allows you to implement an interface based on the structures, lazily storing the references to the data and cloning it only if (and for the first time) the mutation is required.

struct LazyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> LazyBuffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data: Cow::Borrowed(data) }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn append(&mut self, data: &[u8]) {
        self.data.to_mut().extend(data)
    }
}
fn main() {
    let value = remove_whitespaces("Hello world");
    let c: &str = &value;
    println!("c: {c}");

    let b = value.to_owned();
    println!("{b}");

    let a = value.into_owned();
    println!("{a}");


    let user = User::new_owned("first_name".to_owned(), "last_name".to_owned());
    println!("Name: {} {}", user.first_name, user.last_name);

    let user = User::new_borrowed("first_name", "last_name");
    println!("Name: {} {}", user.first_name, user.last_name);


    let first_name = "Eve".to_owned();
    let last_name = "Monepenny".to_owned();
    let user = User::new_borrowed(&first_name, &last_name);
    println!("Name: {} {}", user.first_name, user.last_name);

    // 3)
    let data = vec![0u8; 10];
    let mut buffer = LazyBuffer::new(&data);
    println!("{:?}", buffer.data());

    buffer.append(&[1, 2, 3]);
    println!("{:?}", buffer.data());

    buffer.append(&[4, 5, 6]);
    println!("{:?}", buffer.data());


}
