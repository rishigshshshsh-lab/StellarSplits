#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String, Address};
use soroban_sdk::testutils::Address as _;

#[test]
fn test_split_bill() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitBillRegistry, ());
    let client = SplitBillRegistryClient::new(&env, &contract_id);

    let bill_id = String::from_str(&env, "bill-123");
    let total_amount = 100000000; // 10 XLM (in stroops or standard scale)

    let addr1 = Address::generate(&env);
    let addr2 = Address::generate(&env);
    let participants = vec![&env, addr1.clone(), addr2.clone()];

    // Create split
    client.create_split(&bill_id, &total_amount, &participants);

    // Get status
    let status = client.get_split_status(&bill_id);
    assert_eq!(status.total_amount, total_amount);
    assert_eq!(status.participants.len(), 2);
    assert_eq!(status.participants.get(0).unwrap().address, addr1);
    assert_eq!(status.participants.get(0).unwrap().paid, false);
    assert_eq!(status.participants.get(1).unwrap().address, addr2);
    assert_eq!(status.participants.get(1).unwrap().paid, false);

    // Mark paid
    client.mark_paid(&bill_id, &addr1);

    // Verify status updated
    let status2 = client.get_split_status(&bill_id);
    assert_eq!(status2.participants.get(0).unwrap().paid, true);
    assert_eq!(status2.participants.get(1).unwrap().paid, false);
}
