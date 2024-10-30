// Find all our documentation at https://docs.near.org
//use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near, AccountId};
//use near_sdk::serde::Serialize;

//#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize )]
//#[borsh(crate = "near_sdk::borsh")]
//#[serde(crate = "near_sdk::serde")]
#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct Post {
    pub post_id: u16,
    pub content: String,
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Clone )]
// #[borsh(crate = "near_sdk::borsh")]
// #[serde(crate = "near_sdk::serde")]
#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub website: String,
    pub owner: AccountId,
    pub members: Vec<AccountId>, //change to Store::Vector
    pub created_on: u64,
    pub posts: Vec<Post>, ////change to Store::Vector
}

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    group: Group,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            group: Group {
                name: "Default Group".to_string(),
                description: "A default group description".to_string(),
                website: "https://example.com".to_string(),
                owner: env::signer_account_id(),
                members: vec![env::signer_account_id()],
                created_on: env::block_timestamp(),
                posts: Vec::new(),
            },
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    #[init]
    pub fn new(name: String, description: String, website: String) -> Self {
        Self {
            group: Group {
                name,
                description,
                website,
                owner: env::signer_account_id(),
                members: vec![env::signer_account_id()],
                created_on: env::block_timestamp(),
                posts: Vec::new(),
            },
        }
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn get_group_info(&self) -> Group {
        self.group.clone()
    }

    pub fn add_member(&mut self, member: AccountId) {
        assert!(
            env::signer_account_id() == self.group.owner,
            "Only the owner can add members"
        );
        self.group.members.push(member);
    }

    pub fn add_post(&mut self, content: String) {
        assert!(
            self.group.members.contains(&env::signer_account_id()),
            "Only members can add posts"
        );
        let post_id = self.group.posts.len() as u16;
        let post = Post { post_id, content };
        self.group.posts.push(post);
    }
}
