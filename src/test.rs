#[cfg(test)]
mod tests {
    use crate::prim_tester;

    #[test]
    fn test_prim_tester() {
        assert_eq!(true,prim_tester(2));
        assert_eq!(true,prim_tester(3));
        assert_eq!(true,prim_tester(5));
        assert_eq!(true,prim_tester(7));
        assert_eq!(true,prim_tester(11));
        assert_eq!(true,prim_tester(13));
        assert_eq!(true,prim_tester(17));
        assert_eq!(true,prim_tester(19));
        assert_eq!(true,prim_tester(23));
        assert_eq!(true,prim_tester(29));
        assert_eq!(true,prim_tester(31));
        assert_eq!(true,prim_tester(37));
        assert_eq!(true,prim_tester(41));
        assert_eq!(true,prim_tester(43));

    }
}