fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;
    use std::fs::File;
    use std::io::ErrorKind;

    fn user_name_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        let mut f = File::open("hello.wd")?.read_to_string(&mut s)?;
        Ok(s)
    }

    #[test]
    fn match_different_errors() {
        let f = File::open("hello.wd");
        {
            let f = match f {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.wd") {
                        Ok(fc) => fc,
                        Err(_) => panic!("Tenso!")
                    },
                    other_problem => panic!("Vish!")
                }
            };
        }

        let user_name = user_name_from_file().unwrap();
        assert_eq!("allan.silva", user_name.trim());
    }
}