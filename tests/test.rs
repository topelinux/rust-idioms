pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::io::Error;
    use std::mem;
    use std::rc::Rc;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }

    #[test]
    fn test_option_iter() {
        let turing = Some("Turing");
        let mut logicians = vec!["Curry", "Kleene"];

        logicians.extend(turing);
        assert_eq!(vec!["Curry", "Kleene", "Turing"], logicians);
    }

    #[derive(Default)]
    struct SomeOptions {
        foo: i32,
        bar: f32,
    }

    #[test]
    fn test_default_trait() {
        let options: SomeOptions = Default::default();
        let option1 = SomeOptions {
            foo: 42,
            ..Default::default()
        };
        assert_eq!(option1.foo, 42);
    }

    #[test]
    fn test_variable_closure() {
        //Rc::new will use Box to contain the data
        let num1 = Rc::new(1);
        let num2 = Rc::new(2);
        let num3 = Rc::new(3);

        let closure = {
            let num2 = num2.clone();
            let num3 = num3.as_ref();

            move || {
                let ret = *num1 + *num2 + *num3;
                assert_eq!(ret, 6);
            }
        };

        closure();
    }

    enum MyEnum {
        A { name: String, x: u8 },
        B { name: String },
    }

    fn a_to_b(e: &mut MyEnum) {
        *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
            MyEnum::B {
                name: mem::replace(name, String::new()),
            }
        } else {
            return;
        }
    }

    #[test]
    fn test_mem_replace() {
        let mut a = MyEnum::A {
            name: String::from("tope"),
            x: 0,
        };

        a_to_b(&mut a);

        match a {
            MyEnum::A { name: _, x: _ } => assert!(false),
            MyEnum::B { name } => assert_eq!(name, String::from("tope")),
        }
    }
    trait FileReader {
        fn read_all(&self) -> Result<String, Error>;
    }

    impl FileReader for str {
        fn read_all(&self) -> Result<String, Error> {
            use std::fs::File;
            use std::io::prelude::*;

            let mut contents = String::new();
            let mut f = File::open(self)?;
            f.read_to_string(&mut contents)?;
            Ok(contents)
        }
    }

    impl FileReader for Option<String> {
        fn read_all(&self) -> Result<String, Error> {
            use std::io::ErrorKind;
            match *self {
                Option::None => Err(Error::new(ErrorKind::NotFound, "No such file or directory")),
                Option::Some(ref s) => s.read_all(),
            }
        }
    }

    trait FileReaderIter: Iterator<Item = String> {
        fn read_nth(&mut self, pos: usize) -> Result<String, Error> {
            self.nth(pos).read_all()
        }
    }

    struct MyTest {
        inner: Vec<String>,
    }
    impl Iterator for MyTest {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.pop()
        }
    }

    //impl FileReaderIter for Args {}
    impl FileReaderIter for MyTest {}

    fn assert_check_content<T: AsRef<str>, T1: AsRef<str>>(e: T, r: T1) {
        assert_eq!(e.as_ref(), r.as_ref());
    }
    #[test]
    fn test_trait_obj() {
        let mut my_test = MyTest {
            inner: vec!["./tests/test_trait.txt".to_string()],
        };
        let trait_obj: &mut dyn FileReaderIter = &mut my_test;

        if let Err(err) = trait_obj.read_nth(0).and_then(|content| {
            assert_check_content("hello rust\n", content);
            Ok(())
        }) {
            println!("err is {:?}", err);
            assert!(false);
        }
    }
}
