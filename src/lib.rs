use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct OCTToken {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

#[near_bindgen]
impl OCTToken {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token
            .internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }
}

near_contract_standards::impl_fungible_token_core!(OCTToken, token);
near_contract_standards::impl_fungible_token_storage!(OCTToken, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for OCTToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{env, testing_env, MockedBlockchain};

    use super::*;

    const DECIMALS_BASE: Balance = 1000_000_000_000_000_000_000_000;
    const TOTAL_SUPPLY: Balance = 100 * 1_000_000 * DECIMALS_BASE;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn new_token() -> OCTToken {
        OCTToken::new(
            accounts(0).into(),
            TOTAL_SUPPLY.into(),
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "OCTToken".to_string(),
                symbol: "OCT".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let token_contract = new_token();
        testing_env!(context.is_view(true).build());
        assert_eq!(token_contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(token_contract.ft_balance_of(accounts(0)).0, TOTAL_SUPPLY);
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut token_contract = new_token();

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(token_contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        token_contract.storage_deposit(None, None);

        let transfer_amount = TOTAL_SUPPLY / 5;
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        token_contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(
            token_contract.ft_balance_of(accounts(0)).0,
            (TOTAL_SUPPLY - transfer_amount)
        );
        assert_eq!(token_contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
