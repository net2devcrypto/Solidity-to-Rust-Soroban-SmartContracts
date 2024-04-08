#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct VoteContract;

#[contractimpl]
impl VoteContract {

    pub fn registerVotes(env: Env, user: String, votes: u64, choice: String) {
        let time = env.ledger().timestamp();
        if votes == 0 {
            log!(&env, "Invalid Vote Count", votes);
            return;
        }
        else {
            log!(&env, "Name: {}", user);
            log!(&env, "Selected: {}", choice);
            log!(&env, "Votes: {}", votes);
            log!(&env, "Timestamp: {}", time);
        }
    }
}
