#[cfg(test)]
mod tests {

    use php_5x::BUILD_ID;

    #[test]
    fn test_build_id() {
        assert_eq!(BUILD_ID.to_str().unwrap(), "API20131226,NTS");
    }


}