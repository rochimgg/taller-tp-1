

#[cfg(test)]
mod tests {

    use crate::reader::Reader;

    #[test]
    fn test_read_file() {
        let reader = Reader::new(String::from("/Users/mac/repos/taller-tp-1/src/test.txt"));
        let result = reader.read_file();
        assert!(result.is_ok());
        let contents = result.unwrap();
        assert_eq!(contents, "test text");
    }

}


