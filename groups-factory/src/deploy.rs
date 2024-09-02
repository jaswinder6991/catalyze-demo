use near_sdk::serde::Serialize;
use near_sdk::{env, log, near, AccountId, NearToken, Promise, PromiseError, PublicKey};

use crate::{Contract, ContractExt, NEAR_PER_STORAGE, NO_DEPOSIT, TGAS};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct GroupInitArgs {
    name: String, 
    description: String, 
    website: String
}

#[near]
impl Contract {
    #[payable]
    pub fn create_factory_subaccount_and_deploy(
        &mut self,
        name: String,
        description: String, 
        website: String,
        sub_account: String,
        public_key: Option<PublicKey>,
    ) -> Promise {
        // Assert the sub-account is valid
        let current_account = env::current_account_id().to_string();
        let subaccount: AccountId = format!("{sub_account}.{current_account}").parse().unwrap();
        assert!(
            env::is_valid_account_id(subaccount.as_bytes()),
            "Invalid subaccount"
        );

        // Assert enough tokens are attached to create the account and deploy the contract
        let attached = env::attached_deposit();

        let code = self.code.clone().unwrap();
        let contract_bytes = code.len() as u128;
        let minimum_needed = NEAR_PER_STORAGE.saturating_mul(contract_bytes);
        assert!(
            attached >= minimum_needed,
            "Attach at least {minimum_needed} yⓃ"
        );

        let init_args = near_sdk::serde_json::to_vec(&GroupInitArgs { name, description, website }).unwrap();

        let mut promise = Promise::new(subaccount.clone())
            .create_account()
            .transfer(attached)
            .deploy_contract(code)
            .function_call(
                "new".to_owned(),
                init_args,
                NO_DEPOSIT,
                TGAS.saturating_mul(5),
            );

        // Add full access key if the user passes one
        if let Some(pk) = public_key {
            promise = promise.add_full_access_key(pk);
        }

        // Add callback
        promise.then(
            Self::ext(env::current_account_id()).create_factory_subaccount_and_deploy_callback(
                subaccount,
                env::predecessor_account_id(),
                attached,
            ),
        )
    }

    #[private]
    pub fn create_factory_subaccount_and_deploy_callback(
        &mut self,
        account: AccountId,
        user: AccountId,
        attached: NearToken,
        #[callback_result] create_deploy_result: Result<(), PromiseError>,
    ) -> bool {
        if let Ok(_result) = create_deploy_result {
            log!("{}",format!("Correctly created and deployed to {account}"));
            return true;
        };

        log!("{}",format!(
            "Error creating {account}, returning {attached}yⓃ to {user}"
        ));
        Promise::new(user).transfer(attached);
        false
    }
}