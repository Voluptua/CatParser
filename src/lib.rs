mod catparser;

#[cfg(test)]
mod tests {
    use crate::catparser::parse;

    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(parse("test"), "test");
    }
}
