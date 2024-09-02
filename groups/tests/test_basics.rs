use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let outcome = user_account
        .call(contract.id(), "new")
        .args_json(json!({
            "name":"Test Name", 
            "description": "This is a test description", 
            "website": "google.com"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let add_member_outcome = user_account
    .call(contract.id(), "add_member")
    .args_json(json!({"member":"jaswinder.testnet"}))
    .transact()
    .await?;
    assert!(add_member_outcome.is_success());

    let add_post_outcome = user_account
    .call(contract.id(), "add_post")
    .args_json(json!({"content":"This is a post"}))
    .transact()
    .await?;
    assert!(add_post_outcome.is_success());

    let user_message_outcome = contract
    .view("get_group_info")
    .args_json(json!({}))
    .await?;
println!("{}", user_message_outcome.json::<serde_json::Value>()?);

    Ok(())
}
