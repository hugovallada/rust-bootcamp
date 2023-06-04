/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0, "Balance should be 0");
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Can not deposit a negative amount")
        }
        self.balance += amount;
    }

    pub fn transfer(self, account_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to ${account_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();
        let balance = account.get_balance();
        assert_eq!(balance, 0, "Balance should be 0");
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "Balance should be 100");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut acc = SavingsAccount::new();
        acc.deposit(-2);
    }

    #[test]
    fn should_be_able_to_transfer() -> Result<(), String> {
        let mut acc = SavingsAccount::new();
        acc.deposit(100);
        acc.transfer(123, 100)?;
        Ok(())
    }
}
