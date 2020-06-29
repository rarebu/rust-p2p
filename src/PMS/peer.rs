#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let peer =  Peer::new().unwrap();

        assert_eq!(add(1, 2), 3);
    }

}