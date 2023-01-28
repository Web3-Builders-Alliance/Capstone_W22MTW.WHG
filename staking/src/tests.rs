#[cfg(test)]
mod tests {
    use crate::test_functions;

    #[test]
    fn proper_initalization(){
        test_functions::set_contract();
    }

    #[test]
    fn test_bonding(){
        test_functions::set_contract();
        test_functions::bonding_tokens();
    }

    #[test]
    fn test_redelegate(){
        test_functions::set_contract();
        test_functions::redelegate();
    }
}