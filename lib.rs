#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait DynftUpgrade {
    /// Initialize the contract. This function is called once when the contract is deployed.
    #[init]
    fn init(&self) {}

    /// Endpoint to upgrade a Citizen NFT to a Soldier NFT by consuming 5 GOLD and 5 ORE.
    #[payable("*")]
    #[endpoint(upgradeCitizenToSoldier)]
    fn upgrade_citizen_to_soldier(&self, nft_token_id: TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let gold_token_id: TokenIdentifier = TokenIdentifier::from("GOLD");
        let ore_token_id: TokenIdentifier = TokenIdentifier::from("ORE");

        // Check if the correct amounts of GOLD and ORE tokens are sent
        let payments = self.call_value().all_esdt_transfers();
        let mut gold_amount = BigUint::zero();
        let mut ore_amount = BigUint::zero();

        for payment in payments.iter() {
            if payment.token_identifier == gold_token_id {
                gold_amount += &payment.amount;
            } else if payment.token_identifier == ore_token_id {
                ore_amount += &payment.amount;
            }
        }

        require!(gold_amount == 5u64.into(), "Require 5 GOLD tokens");
        require!(ore_amount == 5u64.into(), "Require 5 ORE tokens");

        // Burn the GOLD and ORE tokens by sending them to the zero address
        self.send().direct(&ManagedAddress::zero(), &gold_token_id, 0, &gold_amount, &[]);
        self.send().direct(&ManagedAddress::zero(), &ore_token_id, 0, &ore_amount, &[]);

        // Verify the caller owns the specified Citizen NFT and perform the upgrade
        self.verify_and_upgrade_nft(&caller, &nft_token_id, nft_nonce)?;

        Ok(())
    }

    /// Helper function to verify ownership of a Citizen NFT and upgrade it to a Soldier NFT.
    fn verify_and_upgrade_nft(&self, owner: &ManagedAddress, nft_token_id: &TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
        // Verify ownership of the NFT
        let nft_balance = self.blockchain().get_esdt_balance(owner, nft_token_id, nft_nonce);
        require!(nft_balance > 0u64.into(), "Caller does not own the specified NFT");

        // Logic to upgrade the Citizen NFT to a Soldier NFT using dyNFT feature
        // This involves changing the metadata or attributes of the existing NFT
        self.upgrade_nft_metadata(owner, nft_token_id, nft_nonce)?;

        Ok(())
    }

    /// Function to change the metadata of the NFT to reflect the upgraded rank.
    fn upgrade_nft_metadata(&self, owner: &ManagedAddress, nft_token_id: &TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
        // Assume a function or interface exists to modify dyNFT metadata
        // Modify the metadata to change the rank from Citizen to Soldier
        // Placeholder for actual dyNFT operations
        Ok(())
    }
}