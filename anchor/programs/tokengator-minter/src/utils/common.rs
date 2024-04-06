use anchor_lang::{prelude::*, system_program};
use wen_new_standard::{
    id as wns_program_id, GROUP_ACCOUNT_SEED, MANAGER_SEED, MEMBER_ACCOUNT_SEED,
};

use crate::constants::*;
use crate::errors::*;
use crate::id;

pub fn fetch_community_id(community: &str) -> Pubkey {
    let (community_id, _) =
        Pubkey::find_program_address(&[PREFIX, community.as_bytes()], &crate::id());
    community_id
}

pub fn check_for_wns_accounts(
    mint_key: &Pubkey,
    group_key: &Pubkey,
    manager_key: &Pubkey,
    member_mint_key: &Option<Pubkey>,
    member_key: &Option<Pubkey>,
) -> Result<()> {
    let wns_program = wns_program_id();
    let (expected_group_key, _) =
        Pubkey::find_program_address(&[GROUP_ACCOUNT_SEED, mint_key.as_ref()], &wns_program.key());
    let (expected_manager_key, _) =
        Pubkey::find_program_address(&[MANAGER_SEED], &wns_program.key());

    require_eq!(
        group_key,
        &expected_group_key,
        TokenGatorMinterError::InvalidWNSGroup
    );

    require_eq!(
        manager_key,
        &expected_manager_key,
        TokenGatorMinterError::InvalidWNSManager
    );

    if let Some(member_key) = member_key {
        let member_mint_key = member_mint_key.unwrap();
        let (expected_member_key, _) = Pubkey::find_program_address(
            &[MEMBER_ACCOUNT_SEED, member_mint_key.as_ref()],
            &wns_program.key(),
        );

        require_eq!(
            member_key,
            &expected_member_key,
            TokenGatorMinterError::InvalidWNSManager
        );
    }

    Ok(())
}

pub fn is_valid_username(username: &str) -> bool {
    if username.len() < 3 || username.len() > MAX_NAME_SIZE {
        return false;
    }

    true
}

pub fn is_valid_url(url: &str) -> bool {
    let starts_with_http = url.starts_with("http://") || url.starts_with("https://");

    let has_valid_protocol_format = url.matches("://").count() == 1 && !url.contains(":///");

    let after_protocol = url.split("://").nth(1).unwrap_or("");
    let has_content_after_protocol = !after_protocol.is_empty() && after_protocol != "/";

    let dot_parts = url.split('.').collect::<Vec<&str>>();
    let is_ipv6 = url.contains("[::1]") || url.contains('[') && url.contains(']');

    // Simplified check for IPv6
    let has_valid_domain = dot_parts.len() >= 2
        && !dot_parts.last().unwrap_or(&"").is_empty()
        && dot_parts.iter().all(|&part| !part.is_empty());
    let has_valid_domain_or_localhost =
        (has_valid_domain && !dot_parts.contains(&"")) || url.contains("localhost") || is_ipv6;

    let no_empty_segments =
        !url.contains("..") && !after_protocol.starts_with('.') && !after_protocol.ends_with('.');
    let no_start_or_end_hyphen_in_domain =
        !url.contains("://-") && !url.contains(".-") && !url.contains("-.");

    let valid_scheme = starts_with_http && has_valid_protocol_format;
    let valid_path = has_content_after_protocol && no_empty_segments;
    let valid_domain = has_valid_domain_or_localhost && no_start_or_end_hyphen_in_domain;

    valid_scheme && valid_path && valid_domain
}

pub fn realloc_account<'a>(
    account: AccountInfo<'a>,
    new_account_size: usize,
    rent_payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
) -> Result<()> {
    require_keys_eq!(
        *account.owner,
        id(),
        TokenGatorMinterError::InvalidAccountOwner
    );

    let current_account_size = account.data.borrow().len();
    if current_account_size >= new_account_size {
        return Ok(());
    }

    let current_lamports = account.lamports();
    let rent_exempt_lamports = Rent::get()?.minimum_balance(new_account_size);

    let lmaports_diff = rent_exempt_lamports.saturating_sub(current_lamports);
    if lmaports_diff.gt(&0) {
        system_program::transfer(
            CpiContext::new(
                system_program,
                system_program::Transfer {
                    from: rent_payer,
                    to: account.clone(),
                },
            ),
            lmaports_diff,
        )?;
    }

    AccountInfo::realloc(&account, new_account_size, false)?;
    Ok(())
}
