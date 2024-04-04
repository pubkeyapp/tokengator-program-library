use anchor_lang::solana_program::pubkey::Pubkey;

pub const ROYALTY_BASIS_POINTS_FIELD: &str = "royalty_basis_points";

pub const MANAGER_SEED: &[u8] = b"manager";
pub const GROUP_ACCOUNT_SEED: &[u8] = b"group";
pub const MEMBER_ACCOUNT_SEED: &[u8] = b"member";
pub const META_LIST_ACCOUNT_SEED: &[u8] = b"extra-account-metas";
pub const APPROVE_ACCOUNT_SEED: &[u8] = b"approve-account";

pub const TOKEN22: Pubkey = anchor_spl::token_2022::ID;

pub mod approve;
pub mod group;
pub mod manager;
pub mod member;

pub use approve::*;
pub use group::*;
pub use manager::*;
pub use member::*;
