use near_workspaces::types::{AccountId, NearToken};
use serde_json::json;

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    // let alice = sandbox
    //     .create_tla(
    //         "alice.test.near".parse().unwrap(),
    //         SecretKey::from_random(KeyType::ED25519),
    //     )
    //     .await?
    //     .unwrap();

    let bob = sandbox.dev_create_account().await?;
    
    let res = bob
        .call(contract.id(),"create_factory_subaccount_and_deploy")
        .args_json(json!({"name": "rustaceans", "description": "A group for Rust Devs", "website":"rust.org", "sub_account":"rustaceans"}))
        .max_gas()
        .deposit(NearToken::from_near(5))
        .transact()
        .await?;

    assert!(res.is_success());

    //your group smart contract and bob is the owner
    let sub_accountid: AccountId = format!("rustaceans.{}", contract.id())
        .parse()
        .unwrap();

    let add_member_outcome = bob
    .call(&sub_accountid, "add_member")
    .args_json(json!({"member":"jaswinder.testnet"}))
    .transact()
    .await?;
    assert!(add_member_outcome.is_success());

    let user_message_outcome = bob
    .view(&sub_accountid, "get_group_info")
    .args_json(json!({}))
    .await?;
    println!("{}", user_message_outcome.json::<serde_json::Value>()?);


    Ok(())
}