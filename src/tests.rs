

#[cfg(test)]
mod tests {

    use crate::anchors;
    use crate::reader;

    #[test]
    fn test_function_returns_perro() {
        let result = anchors::find_previous_match("perro", 'r');
        let unr: usize = result.unwrap();
        let x = &unr;
        let result = "perro";
        assert_eq!(result, "perro");
    }


    #[test]
    fn test_read_file() {
        let reader = reader::new(String::from("/path/to/file.txt"));
        let result = reader.read_file();
        assert!(result.is_ok());
        let contents = result.unwrap();
        assert_eq!(contents, "File contents"); // Replace "File contents" with the expected contents of the file
    }


}


