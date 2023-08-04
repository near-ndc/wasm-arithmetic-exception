use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn test_hash(&mut self, data: String) {}
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    use crate::*;
    use near_sdk::base64::{decode, encode};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};

    fn acc_u1() -> AccountId {
        "user2.near".parse().unwrap()
    }

    #[allow(dead_code)]
    fn setup() -> (VMContext, Contract) {
        let ctx = VMContextBuilder::new()
            .predecessor_account_id(acc_u1())
            .is_view(false)
            .build();
        let ctr = Contract {};

        testing_env!(ctx.clone());
        return (ctx, ctr);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: InvalidLastSymbol(5, 113)"
    )]
    fn base64_check() {
        /* using unix base64 tool the two encodings give the same value:
        d1="QzNWwq=="
        d2="QzNWwr=="

        echo -n $d1 | base64 -d | xxd
        echo -n $d2 | base64 -d | xxd
         */

        let s1 = "QzNWwq==";
        let s2 = "QzNWwr==";

        let b1 = decode(s1).unwrap();
        let b2 = decode(s2).unwrap();

        assert_eq!(b1, b2);
        assert_eq!(s1, encode(b1));
        assert_eq!(s2, encode(b2));
    }
}
