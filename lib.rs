#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "ink-as-dependency"))]

#[ink::contract]
pub mod tic_tac_toe {

    use ink::prelude::vec;
    use ink::prelude::vec::Vec;
    
    use openbrush::contracts::traits::psp22::PSP22Ref;

    use ink::env::CallFlags;

    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct TicTacToe {
        board: Vec<u64>, //0 to 8 cells
        turn: AccountId,
        symbols: ink::storage::Mapping<AccountId, u64>,
        player_one: AccountId,
        player_two: AccountId,
        staking_token: AccountId,
        stake_amount: Balance,
        stakes: ink::storage::Mapping<AccountId, Balance>,
        last_winner: AccountId,
    }

    impl TicTacToe {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new(
            player_one: AccountId,
            player_two: AccountId,
            player_one_symbol: u64,
            player_two_symbol: u64,
            staking_token: AccountId,
            stake_amount: Balance,
        ) -> Self {

            let mut contract = Self::default();

            let board = vec![0; 9]; //empty array

            contract.board = board; //set board to empty state

            contract.staking_token = staking_token; //set staking token

            contract.stake_amount = stake_amount; //set stake amount

            assert!(player_one != player_two); //addresses must not be the same

            assert!(player_one_symbol != player_two_symbol); //symbols must be distinct

            assert!(
                (player_one_symbol == 1 || player_one_symbol == 2)
                    && (player_two_symbol == 1 || player_two_symbol == 2)
            ); //symbols must be either 1 or 2

            contract.player_one = player_one; //set player one address

            contract.player_two = player_two; //set player two address

            contract.symbols.insert(player_one, &player_one_symbol); //set player one symbol

            contract.symbols.insert(player_two, &player_two_symbol); //set player two symbol

            contract.turn = player_one; //initialize turn to player one

            contract

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
        pub fn get_player_one(&self) -> AccountId {
            self.player_one //get player one address
        }

        #[ink(message)]
        pub fn get_player_two(&self) -> AccountId {
            self.player_two //get player two address
        }

        #[ink(message)]
        pub fn get_player_two_symbol(&self) -> u64 {
            self.symbols.get(self.player_two).unwrap_or(0) //get player two symbol
        }

        #[ink(message)]
        pub fn get_player_one_symbol(&self) -> u64 {
            self.symbols.get(self.player_one).unwrap_or(0) //get player one symbol
        }

        #[ink(message)]
        pub fn get_board(&self) -> Vec<u64> {
            //read and return board as array
            let board = &self.board;
            board.to_vec()
        }

        #[ink(message)]
        pub fn stake_tokens(&mut self) {
            let player = self.env().caller(); //get caller address
            let stakes = self.stakes.get(player).unwrap_or(0); //get stake if existent

            assert!(player == self.player_one || player == self.player_two); //Caller must be player one or two

            if stakes > 0 {
                panic!("Already staked for this round")
            } //Make sure player hasn't already staked

            let balance = PSP22Ref::balance_of(&self.staking_token, player); //get user balance of token

            let allowance =
                PSP22Ref::allowance(&self.staking_token, player, Self::env().account_id()); //get spending allowance contract has to player

            assert!(balance > self.stake_amount); //balance must be greater than stake amount

            assert!(allowance > self.stake_amount); //allowance must be greater than stake amount

            //Transfer stake amount from caller (player) to contract
            PSP22Ref::transfer_from_builder(
                &self.staking_token,
                self.env().caller(),
                Self::env().account_id(),
                self.stake_amount,
                ink::prelude::vec![],
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .expect("Transfer failed")
            .expect("Transfer failed");

            self.stakes.insert(player, &self.stake_amount); //Add stake amount to user stake
        }

        #[inline]
        pub fn _has_won(&self, symbol: u64) -> bool {
            let vertical = [[0, 3, 6], [1, 4, 7], [2, 5, 8]];
            let horizontal = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
            let diagonal = [[0, 4, 8], [2, 4, 6]];

            //check vertical
            let mut v_win = false;
            for i in 0..=2 {
                let mut count = 0;
                for j in 0..=2 {
                    if self.board[vertical[i][j]] == symbol {
                        count += 1;
                    }
                }
                if count == 3 {
                    v_win = true;
                    break;
                }
            }

            //check horizontal
            let mut h_win = false;
            for i in 0..=2 {
                let mut count = 0;
                for j in 0..=2 {
                    if self.board[horizontal[i][j]] == symbol {
                        count += 1;
                    }
                }
                if count == 3 {
                    h_win = true;
                    break;
                }
            }

            //check diagonal
            let mut d_win = false;
            for i in 0..=1 {
                let mut count = 0;
                for j in 0..=2 {
                    if self.board[diagonal[i][j]] == symbol {
                        count += 1;
                    }
                }
                if count == 3 {
                    d_win = true;
                    break;
                }
            }

            if v_win == true || h_win == true || d_win == true {
                true
            } else {
                false
            }
        }

        #[inline]
        pub fn _clear_board(&mut self) {
            let board = vec![0; 9];
            self.board = board;
        }

        #[inline]
        pub fn _is_cell_empty(&self, cell: u64) -> bool {
            if self.board[usize::try_from(cell).unwrap()] == 0 {
                true
            } else {
                false
            }
        }

        #[inline]
        pub fn _is_board_filled(&self) -> bool {
            let mut filled_cells = 0;
            let board = &self.board;
            for cell in 0..=8 {
                if board[usize::try_from(cell).unwrap()] != 0 {
                    filled_cells += 1;
                }
            }
            if filled_cells == 9 {
                true
            } else {
                false
            }
        }

        #[inline]
        pub fn _reward_winner(&mut self, account: AccountId) {
            let total_stakes = PSP22Ref::balance_of(&self.staking_token, Self::env().account_id()); //get total stakes

            PSP22Ref::transfer(
                &self.staking_token,
                account,
                total_stakes,
                ink::prelude::vec![],
            ); //transfer everything to the winner

            self.stakes.insert(self.player_one, &0);
            
            self.stakes.insert(self.player_two, &0);
        }

        #[inline]
        pub fn _refund_tokens(&mut self) {
            let total_stakes = PSP22Ref::balance_of(&self.staking_token, Self::env().account_id()); //get total stakes
            let per_player = total_stakes / 2;

            PSP22Ref::transfer(
                &self.staking_token,
                self.player_one,
                per_player,
                ink::prelude::vec![],
            ); //transfer half to player one
            PSP22Ref::transfer(
                &self.staking_token,
                self.player_two,
                per_player,
                ink::prelude::vec![],
            ); //transfer half to player two

            self.stakes.insert(self.player_one, &0);

            self.stakes.insert(self.player_two, &0);

        }

        #[ink(message)]
        pub fn play(&mut self, cell: u64) {
            assert!(cell <= 8);

            let player = self.env().caller(); //get caller address

            assert!(player == self.player_one || player == self.player_two); //caller must be player one or two

            assert!(self.get_player_one_stake() > 0 && self.get_player_two_stake() > 0); //both players must have staked

            let is_empty = self._is_cell_empty(cell); //check if cell is empty

            assert!(is_empty == true); //cell must be empty

            assert!(self.turn == player); //must be player's turn

            let mut board = self.get_board();

            let player_one_symbol = self.get_player_one_symbol();
            let player_two_symbol = self.get_player_two_symbol();

            let cell_index = usize::try_from(cell).unwrap(); //convert index to usize

            board[cell_index] = self.symbols.get(player).unwrap_or(0);

            self.board = board;

            let player_one_won = self._has_won(player_one_symbol);

            let player_two_won = self._has_won(player_two_symbol);

            let mut game_over = false;

            if player_one_won == true {
                //player one won
                self.turn = self.player_one; //set player to start next round
                self._reward_winner(self.player_one);
                self._clear_board(); //clear game board
                self.last_winner = self.player_one; //set to last winner
                game_over = true; //game is over
            } else if player_two_won == true {
                //player two won
                self.turn = self.player_two; //set player to start next round
                self._reward_winner(self.player_two);
                self._clear_board(); //clear game board
                self.last_winner = self.player_one; //set to last winner
                game_over = true;
            } else {
                if self._is_board_filled() == true {
                    //It's a draw
                    self.turn = self.player_one;
                    self._refund_tokens(); //refund tokens because no one won
                    self._clear_board();
                    game_over = true;
                }
            }

            if game_over == false {
                if self.turn == self.player_one {
                    self.turn = self.player_two;
                } else {
                    self.turn = self.player_one;
                }
            }
        }
    }
}
