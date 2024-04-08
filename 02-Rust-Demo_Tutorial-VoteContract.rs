#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, log, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub yes: u64,
    pub no: u64,
    pub total: u64,
}

const POLL: Symbol = symbol_short!("POLL");

#[contracttype]
pub enum Registry {
    Record(Symbol)
}

#[contracttype]
#[derive(Clone)]
pub struct Record {
    pub name: Symbol,
    pub selected: Symbol,
    pub votes: u64,
    pub time: u64,
}

const YES: Symbol = symbol_short!("yes");

#[contract]
pub struct VoteContract;

#[contractimpl]
impl VoteContract {

    pub fn record_votes(env: Env, user: Symbol, votes: u64, selected: Symbol) {
        let mut records = Self::view_voter(env.clone(), user.clone());
        let time = env.ledger().timestamp();
        if votes == 0 || records.time != 0 || votes > 5 {
            panic!("Cannot Vote");
        }
        else {
            let mut poll = Self::view_poll(env.clone());
            records.name = user.clone();
            records.selected = selected;
            records.votes = votes;
            records.time = time;
            if records.selected == YES {
                poll.yes = poll.yes + votes;
            }
            else {
                poll.no = poll.no + votes;
            }
            poll.total = poll.total + votes;
            env.storage().persistent().set(&Registry::Record(user), &records);
            env.storage().persistent().set(&POLL, &poll);
            log!(&env, "Votes Registered!");
            return;
        }
        
    }

    pub fn view_poll(env: Env) -> Poll {
        env.storage().instance().get(&POLL).unwrap_or(Poll {
            yes: 0,
            no: 0,
            total: 0,
        })
    }
    
    pub fn view_voter(env: Env, voter: Symbol) -> Record {
        let key = Registry::Record(voter.clone());
        env.storage().instance().get(&key).unwrap_or(Record {
            name: symbol_short!("not_found"),
            selected: symbol_short!("none"),
            votes: 0,
            time: 0,
        })
    }
}
