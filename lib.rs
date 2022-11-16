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

    use ink_env::CallFlags;

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
        stakes: ink_storage::Mapping<AccountId, Balance>
    }

    impl TicTacToe {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new(player_one:AccountId, player_two:AccountId, player_one_symbol:u8, player_two_symbol:u8, staking_token: AccountId, stake_amount: Balance) -> Self {
            
            let me = ink_lang::utils::initialize_contract(|contract: &mut Self| {
                
                let mut board = Vec::new();

                for item in 0..=8 {
                    board[item] = 0;
                }

                contract.board = board;

                contract.staking_token = staking_token;

                contract.stake_amount = stake_amount;

                assert!(player_one != player_two);

                assert!(player_one_symbol != player_two_symbol);

                assert!((player_one_symbol == 1 || player_one_symbol == 2) && (player_two_symbol == 1 || player_two_symbol == 2));

                contract.player_one = player_one;

                contract.player_two = player_two;

                contract.symbols.insert(player_one, &player_one_symbol);

                contract.symbols.insert(player_two, &player_two_symbol);

                contract.turn = player_one;
                
            });
            
            me
           
        }

        #[ink(message)]
        pub fn get_stake_amount(&self) -> Balance {
            self.stake_amount
        }

        #[ink(message)]
        pub fn get_current_turn(&self) -> AccountId {
            self.turn
        }

        #[ink(message)]
        pub fn get_staking_token(&self) -> AccountId {
            self.staking_token
        }

        #[ink(message)]
        pub fn get_player_two_stake(&self) -> Balance {
            self.stakes.get(self.player_two).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_player_one_stake(&self) -> Balance {
            self.stakes.get(self.player_one).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_board(&self) -> Vec<u64> {
            let mut board = Vec::new();
            for item in 0..=8 {
                board[item] = self.board[item];
            }
            board
        }
 
    }
}