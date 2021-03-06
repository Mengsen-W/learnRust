#[cfg(test)]
mod test {
    use csv_read::{
        replace_column, {load_csv, write_csv},
    };
    use std::path::PathBuf;

    #[test]
    fn test_csv_challenge() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
        let csv_data = csv_data.unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing");
        assert!(modified_data.is_ok());
        let modified_data = modified_data.unwrap();
        let output_file = write_csv(&modified_data, "output/test.csv");
        assert!(output_file.is_ok());
    }
}
