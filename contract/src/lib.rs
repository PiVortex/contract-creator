use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise, PromiseError, Gas};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};


const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; // 10e18yⓃ
const TGAS: Gas = Gas(10u64.pow(12)); // 10e12yⓃ
const NO_DEPOSIT: Balance = 0; // 0yⓃ

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Contract {
    contract_tracker: UnorderedMap<AccountId, Vec<ContractTracker>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ContractTracker {
    contract_account: AccountId,
    contract_type: ContractType
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ContractType {
    Donation,
    Greeting
}

impl Default for Contract {
    fn default() -> Self {
      Self{contract_tracker: UnorderedMap::new(b"c")}
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_factory_subaccount_and_deploy(
        &mut self,
        name: String,
        contract_type: ContractType,
    ) -> Promise {
        // Assert the sub-account is valid
        let current_account: String = env::current_account_id().to_string();
        let subaccount: AccountId = format!("{name}.{current_account}").parse().unwrap();
        assert!(
            env::is_valid_account_id(subaccount.as_bytes()),
            "Invalid subaccount"
        );

        // Assert enough money is attached to create the account and deploy the contract
        let attached: u128 = env::attached_deposit();

        let mut code: Vec<u8> = Vec::new();
        match contract_type {
            ContractType::Donation => {
                code = include_bytes!("./contracts-wasm/donation.wasm").to_vec();
            }
            ContractType::Greeting => {
                code = include_bytes!("./contracts-wasm/greeting.wasm").to_vec();
            }
        }

        let contract_bytes: u128 = code.len() as u128;
        let minimum_needed: u128 = NEAR_PER_STORAGE * contract_bytes;
        assert!(
            attached >= minimum_needed,
            "Attach at least {minimum_needed} yⓃ"
        );


        let promise: Promise = Promise::new(subaccount.clone())
            .create_account()
            .transfer(attached)
            .deploy_contract(code);

        // Add callback
        promise.then(
            Self::ext(env::current_account_id()).create_factory_subaccount_and_deploy_callback(
                subaccount,
                env::predecessor_account_id(),
                attached,
                contract_type
            ),
        )
    }

    #[private]
    pub fn create_factory_subaccount_and_deploy_callback(
        &mut self,
        account: AccountId,
        user: AccountId,
        attached: Balance,
        contract_type: ContractType,
        #[callback_result] create_deploy_result: Result<(), PromiseError>,
    ) -> bool {
        if let Ok(_result) = create_deploy_result {
            log!(format!("Correctly created and deployed to {account}"));

            let element: ContractTracker = ContractTracker {
                contract_account: account,
                contract_type: contract_type
            };

            match self.contract_tracker.get(&user) {
                Some(mut vec) => {
                    vec.push(element);
                    self.contract_tracker.insert(&user, &vec);
                }

                None => {
                    let mut vec: Vec<ContractTracker> = Vec::new();
                    vec.push(element);
                    self.contract_tracker.insert(&user, &vec);
                }
            }

            return true;
        };

        log!(format!(
            "Error creating {account}, returning {attached}yⓃ to {user}"
        ));
        Promise::new(user).transfer(attached);
        false
    }
}

