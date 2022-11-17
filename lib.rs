#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[cfg(not(feature = "ink-as-dependency"))]

#[ink::contract]
pub mod tic_tac_toe {

    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::{
        contracts::{
            traits::psp22::PSP22Ref
        },
    };

    use ink_env::{CallFlags, balance};

    #[ink(storage)]
    #[derive(SpreadAllocate)]

    pub struct TicTacToe {
        board: Vec<u64>,
        turn: AccountId,
        symbols: ink_storage::Mapping<AccountId, u8>,
        player_one:AccountId,
        player_two:AccountId,
        staking_token: AccountId,
        stake_amount: Balance,
        stakes: ink_storage::Mapping<AccountId, Balance>,
        last_winner: AccountId
    }

    impl TicTacToe {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new(player_one:AccountId, player_two:AccountId, player_one_symbol:u8, player_two_symbol:u8, staking_token: AccountId, stake_amount: Balance) -> Self {
            
            let me = ink_lang::utils::initialize_contract(|contract: &mut Self| {
                
                let mut board = Vec::new(); //empty array

                //Set cells to 0
                for item in 0..=8 {
                    board[item] = 0;
                }

                contract.board = board; //set board to empty state

                contract.staking_token = staking_token; //set staking token

                contract.stake_amount = stake_amount; //set stake amount

                assert!(player_one != player_two); //addresses must not be the same

                assert!(player_one_symbol != player_two_symbol); //symbols must be distinct

                assert!((player_one_symbol == 1 || player_one_symbol == 2) && (player_two_symbol == 1 || player_two_symbol == 2)); //symbols must be either 1 or 2

                contract.player_one = player_one; //set player one address

                contract.player_two = player_two; //set player two address

                contract.symbols.insert(player_one, &player_one_symbol); //set player one symbol

                contract.symbols.insert(player_two, &player_two_symbol); //set player two symbol

                contract.turn = player_one; //initialize turn to player one
                
            });
            
            me
           
        }

        #[ink(message)]
        pub fn get_stake_amount(&self) -> Balance {
            self.stake_amount //amount to be staked in game
        }

        #[ink(message)]
        pub fn get_last_winner(&self) -> AccountId {
            self.last_winner //address of most recent winner
        }

        #[ink(message)]
        pub fn get_current_turn(&self) -> AccountId {
            self.turn //who is meant to play?
        }

        #[ink(message)]
        pub fn get_staking_token(&self) -> AccountId {
            self.staking_token //get address of staking token smart contract
        }

        #[ink(message)]
        pub fn get_player_two_stake(&self) -> Balance {
            self.stakes.get(self.player_two).unwrap_or(0) //get total amount of tokens staked by player two
        }

        #[ink(message)]
        pub fn get_player_one_stake(&self) -> Balance {
            self.stakes.get(self.player_one).unwrap_or(0) //get total amount of tokens staked by player one
        }

        #[ink(message)]
        pub fn get_board(&self) -> Vec<u64> {
            //read and return board as array
            let mut board = Vec::new();
            for item in 0..=8 {
                board[item] = self.board[item];
            }
            board
        }

        #[ink(message)]
        pub fn stake_tokens(&mut self) {

            let player = self.env().caller(); //get caller address
            let stakes = self.stakes.get(player).unwrap_or(0); //get stake if existent

            assert!(player == self.player_one || player == self.player_two); //Caller must be player one or two

            if stakes > 0 {
                !panic!(
                    "Already staked for this round"
                )
            } //Make sure player hasn't already staked

            let balance = PSP22Ref::balance_of(&self.staking_token, player); //get user balance of token

            let allowance = PSP22Ref::allowance(&self.staking_token, player, Self::env().account_id()); //get spending allowance contract has to player

            assert!(balance > self.stake_amount); //balance must be greater than stake amount

            assert!(allowance > self.stake_amount); //allowance must be greater than stake amount

            //Transfer stake amount from caller (player) to contract
            PSP22Ref::transfer_from_builder(&self.staking_token, self.env().caller(), Self::env().account_id(), panx_to_lock, ink_prelude::vec![]).call_flags(CallFlags::default().set_allow_reentry(true)).fire().expect("Transfer failed").expect("Transfer failed");
            
            self.stakes.insert(self.player_one, &self.stake_amount); //Add stake amount to user stake

        }

        #[ink(message)]
        pub fn play(&mut self) {

            assert!(self.get_player_one_stake() > 0 && self.get_player_two_stake() > 0);

        }
 
    }
}