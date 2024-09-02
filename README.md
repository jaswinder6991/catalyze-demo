# NEAR Group Contracts

This project implements a decentralized group management system on the NEAR blockchain. It consists of two main components: a Group Contract and a Groups Factory Contract.

## Overview

1. **Group Contract**: Allows users to create and manage on-chain groups.
2. **Groups Factory Contract**: Enables users to deploy Group Contracts dynamically.

## Group Contract Features

- Create a group with a name, description, and website
- Add members to the group
- Create posts within the group
- Retrieve group information

## Groups Factory Contract Features

- Deploy new Group Contracts on-chain
- Create subaccounts for new groups (happens as a part fo above)
- Update and manage the stored Group Contract code

## Smart Contract Structure

### Group Contract (`groups/lib.rs`)

- Defines the `Group` and `Post` structures
- Implements group management functions:
  - Initialize a new group
  - Add members
  - Create posts
  - Retrieve group information

### Groups Factory Contract

1. **Deployment (`groups-factory/deploy.rs`)**
   - Creates subaccounts and deploys Group Contracts
   - Handles callbacks for deployment results

2. **Manager (`groups-factory/manager.rs`)**
   - Updates and retrieves the stored Group Contract code

3. **Main Contract (`groups-factory/lib.rs`)**
   - Defines the factory contract structure
   - Sets up constants and default implementation

## Usage

To use this system:

1. Deploy the Groups Factory Contract
2. Use the factory to create new Group Contracts
3. Interact with individual Group Contracts to manage groups

## Development

This project is developed using the NEAR SDK in Rust. Ensure you have the NEAR development environment set up to compile and deploy these contracts.

## Note

This is a proof of concept (POC) implementation. Review and adapt the code for production use, considering security and scalability aspects.