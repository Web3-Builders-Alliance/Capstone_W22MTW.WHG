#[cfg(test)]
mod test{
    use crate::contract;
    use crate::functions;
    use crate::msg;
    use cosmwasm_std::testing;
    use cosmwasm_std;

    const SENDER: &str = "sender_address";
    const AMOUNT: u128 = 1_000;
    const DENOM: &str = "utest";
}