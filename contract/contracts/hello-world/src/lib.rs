#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Symbol, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantStatus {
    pub address: Address,
    pub paid: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitBill {
    pub total_amount: u64,
    pub participants: Vec<ParticipantStatus>,
}

#[contracttype]
pub enum DataKey {
    Split(String),
}

#[contract]
pub struct SplitBillRegistry;

#[contractimpl]
impl SplitBillRegistry {
    pub fn create_split(env: Env, bill_id: String, total_amount: u64, participants: Vec<Address>) {
        let key = DataKey::Split(bill_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("Split bill already exists");
        }

        let mut participant_statuses = Vec::new(&env);
        for p in participants.iter() {
            participant_statuses.push_back(ParticipantStatus {
                address: p,
                paid: false,
            });
        }

        let bill = SplitBill {
            total_amount,
            participants: participant_statuses,
        };

        env.storage().persistent().set(&key, &bill);

        // Emit split_created event
        env.events().publish(
            (Symbol::new(&env, "split_created"), bill_id),
            total_amount,
        );
    }

    pub fn mark_paid(env: Env, bill_id: String, participant_address: Address) {
        let key = DataKey::Split(bill_id.clone());
        let mut bill: SplitBill = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Split bill does not exist"));

        let mut found = false;
        let mut updated_participants = Vec::new(&env);

        for p in bill.participants.iter() {
            let mut p_status = p.clone();
            if p_status.address == participant_address {
                p_status.paid = true;
                found = true;
            }
            updated_participants.push_back(p_status);
        }

        if !found {
            panic!("Participant address not found in split bill");
        }

        bill.participants = updated_participants;
        env.storage().persistent().set(&key, &bill);

        // Emit payment_marked event
        env.events().publish(
            (Symbol::new(&env, "payment_marked"), bill_id),
            participant_address.clone(),
        );
    }

    pub fn get_split_status(env: Env, bill_id: String) -> SplitBill {
        let key = DataKey::Split(bill_id);
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Split bill does not exist"))
    }
}

mod test;
