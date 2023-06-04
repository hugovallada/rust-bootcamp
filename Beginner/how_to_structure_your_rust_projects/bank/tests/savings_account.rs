use bank::SavingsAccount;

#[test]
fn should_have_a_starting_balance_of_0() {
    let acc = SavingsAccount::new();
    assert_eq!(acc.get_balance(), 0);
}
