impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut most_wealth = 0;
        for account in accounts {
            let mut account_wealth = 0;
            for amount in account {
                account_wealth+=amount;
            }
            if account_wealth > most_wealth { most_wealth = account_wealth; }
        }
        most_wealth
    }
}