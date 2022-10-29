#![no_std]

elrond_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint] 
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let caller = self.blockchain().get_caller();
        self.staking_position(&caller).update(|cur| *cur += payment_amount);
        self.staked_addresses().insert(caller);
    }

    #[endpoint]
    fn unstake(&self, unstake_amount_opt: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        let stake_mapper = self.staking_position(&caller);
        let unstake_amount = match unstake_amount_opt {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => stake_mapper.get()
        };

        let remaining_stake = stake_mapper.update(|amount| {
            require!(
            unstake_amount > 0 && unstake_amount <= *amount,
                "Invalid unstake amount"
            );

            *amount -= &unstake_amount;
            amount.clone()
        });

        if remaining_stake == 0 {
            self.staked_addresses().swap_remove(&caller);
        }

        self.send().direct_egld(&caller, &unstake_amount);
    }

    // STORAGE:
    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
