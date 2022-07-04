use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("4qLzEGU2KaBggBkVgELQAU8wayPMTGa9EwxGWwzKRmKT");
pub mod constants {
    pub const MINTING_PDA_SEED: &[u8] = b"wallet_nft_minting";
}

#[program]
pub mod wallet_nft_mint {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        authorized_creator: Pubkey,
        max_supply: u64,
        og_max: u64,
        wl_max: u64,
        public_max: u64,
        og_price: u64,
        wl_price: u64,
        public_price: u64,
    ) -> ProgramResult {
        ctx.accounts.minting_account.admin_key = *ctx.accounts.initializer.key;
        ctx.accounts.minting_account.authorized_creator = authorized_creator;
        ctx.accounts.minting_account.max_supply = max_supply;
        ctx.accounts.minting_account.og_max = og_max;
        ctx.accounts.minting_account.wl_max = wl_max;
        ctx.accounts.minting_account.public_max = public_max;
        ctx.accounts.minting_account.og_price = og_price;
        ctx.accounts.minting_account.wl_price = wl_price;
        ctx.accounts.minting_account.public_price = public_price;
        ctx.accounts.minting_account.cur_num = 0;
        ctx.accounts.minting_account.cur_stage = 0; // disabled

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_og_list(
        ctx: Context<CreateOriginalList>,
        user: Pubkey,
    ) -> ProgramResult {
        
        let og_list = &mut ctx.accounts.og_list;

        og_list.user = user;
        og_list.minting_account = ctx.accounts.minting_account.key();
        og_list.initializer = ctx.accounts.admin.key();
        og_list.count = 1;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.initializer))]
    pub fn remove_og_list(
        ctx: Context<RemoveOriginalList>,
    ) -> ProgramResult {

        ctx.accounts
                .og_list
                .close(ctx.accounts.initializer.to_account_info())?;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_wl_list(
        ctx: Context<CreateWhiteList>,
        user: Pubkey,
    ) -> ProgramResult {
        
        let wl_list = &mut ctx.accounts.wl_list;

        wl_list.user = user;
        wl_list.minting_account = ctx.accounts.minting_account.key();
        wl_list.initializer = ctx.accounts.admin.key();
        wl_list.count = 1;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.initializer))]
    pub fn remove_wl_list(
        ctx: Context<RemoveWhiteList>,
    ) -> ProgramResult {

        ctx.accounts
                .wl_list
                .close(ctx.accounts.initializer.to_account_info())?;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_price(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_price: u64,
        new_wl_price: u64,
        new_public_price: u64,
    ) -> ProgramResult {
        if new_og_price > 0 {
            ctx.accounts.minting_account.og_price = new_og_price;
        }
        if new_wl_price > 0 {
            ctx.accounts.minting_account.wl_price = new_wl_price;
        }
        if new_public_price > 0 {
            ctx.accounts.minting_account.public_price = new_public_price;
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_amount(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_amount: u64,
        new_wl_amount: u64,
        new_public_amount: u64,
    ) -> ProgramResult {
        if new_og_amount > 0 {
            ctx.accounts.minting_account.og_max = new_og_amount;
        }
        if new_wl_amount > 0 {
            ctx.accounts.minting_account.wl_max = new_wl_amount;
        }
        if new_public_amount > 0 {
            ctx.accounts.minting_account.public_max = new_public_amount;
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_stage(ctx: Context<CommonSt>, _nonce_minting: u8, new_stage: i8) -> ProgramResult {
        if new_stage > -1 && new_stage < 3 {
            ctx.accounts.minting_account.cur_stage = new_stage;
        }
        // 0 => disabled;  1 => OG/WL; 2 => Public;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_uri(ctx: Context<CommonSt>, _nonce_minting: u8, new_uri: String) -> ProgramResult {
        ctx.accounts.minting_account.base_uri = new_uri;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>, creator_key: Pubkey, title: String) -> ProgramResult {
        if ctx.accounts.minting_account.cur_stage < 0 || ctx.accounts.minting_account.cur_stage > 2
        {
            return Err(MintError::InvalidStage.into());
        }

        if ctx.accounts.minting_account.cur_stage == 0 {
            // disabled
            return Err(MintError::NotActive.into());
        }

        // set user minting info
        let mut _max_num = ctx.accounts.minting_account.public_max;
        let mut _price = ctx.accounts.minting_account.public_price;
        let mut _state = 2; // public

        if ctx.accounts.minting_account.cur_stage == 1 {

            if ctx.accounts.og_list.count == 1 {
                _max_num = ctx.accounts.minting_account.og_max;
                _price = ctx.accounts.minting_account.og_price;
                _state = 1; // OG
            }

            if ctx.accounts.wl_list.count == 1 {
                _max_num = ctx.accounts.minting_account.wl_max;
                _price = ctx.accounts.minting_account.wl_price;
                _state = 1; // WL
            }

        }

        if ctx.accounts.minting_account.max_supply <= ctx.accounts.minting_account.cur_num
            || ctx.accounts.minting_account.cur_stage != _state
            || ctx.accounts.user_minting_counter_account.cur_num >= _max_num
        {
            return Err(MintError::NotAllowed.into());
        }

        if ctx.accounts.minting_account.admin_key != *ctx.accounts.owner.key {
            return Err(MintError::NotAllowed.into());
        }

        if ctx.accounts.payer.lamports() < _price {
            return Err(MintError::InsufficientFunds.into());
        }

        let transfer_sol_to_seller =
            system_instruction::transfer(ctx.accounts.payer.key, ctx.accounts.owner.key, _price);

        invoke(
            &transfer_sol_to_seller,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
        )?;

        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];

        let new_uri = format!(
            "{}{}{}",
            ctx.accounts.minting_account.base_uri, ctx.accounts.minting_account.cur_num, ".json"
        );

        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                new_uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        ctx.accounts.user_minting_counter_account.cur_num += 1;
        ctx.accounts.minting_account.cur_num += 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init_if_needed,
        payer = initializer,
        space = 8 + 32 * 3 + 8 * 8 + 1 + 8 + 50,
        seeds = [
            "wallet_nft_minting".as_bytes(),
        ],
        bump,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct MintingAccount {
    pub admin_key: Pubkey,
    pub aury_vault: Pubkey,
    pub authorized_creator: Pubkey,
    pub max_supply: u64,
    pub og_max: u64,
    pub wl_max: u64,
    pub public_max: u64,
    pub og_price: u64,
    pub wl_price: u64,
    pub public_price: u64,
    pub cur_num: u64,
    pub freeze_program: bool,
    pub cur_stage: i8,
    pub base_uri: String,
}
#[derive(Accounts)]
#[instruction(_nonce_minting: u8)]
pub struct CommonSt<'info> {
    #[account(
        mut,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump = _nonce_minting,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct CreateWhiteList<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    
    #[account(mut)]
    minting_account: Box<Account<'info, MintingAccount>>,

    #[account(
    init,
    payer = admin,
    space = 8 + 32 * 3 + 8,
    seeds = [
        "nftminting".as_bytes(),
        "whitelist".as_bytes(),
        minting_account.key().as_ref(),
        user.as_ref(),
    ],
    bump,
    )]
    wl_list: Account<'info, WhiteList>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RemoveWhiteList<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    
    #[account(mut)]
    minting_account: Box<Account<'info, MintingAccount>>,

    #[account(mut, has_one = initializer, constraint = minting_account.key() == wl_list.minting_account)]
    wl_list: Account<'info, WhiteList>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct CreateOriginalList<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    
    #[account(mut)]
    minting_account: Box<Account<'info, MintingAccount>>,

    #[account(
    init,
    payer = admin,
    space = 8 + 32 * 3 + 8,
    seeds = [
        "nftminting".as_bytes(),
        "originallist".as_bytes(),
        minting_account.key().as_ref(),
        user.as_ref(),
    ],
    bump,
    )]
    og_list: Account<'info, OriginalList>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RemoveOriginalList<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    
    #[account(mut)]
    minting_account: Box<Account<'info, MintingAccount>>,

    #[account(mut, has_one = initializer, constraint = minting_account.key() == og_list.minting_account)]
    og_list: Account<'info, OriginalList>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct UserMintingAccount {
    pub cur_num: u64,
}

#[account]
pub struct WhiteList {
    user: Pubkey,
    minting_account: Pubkey,
    initializer: Pubkey,
    count: u64
}

#[account]
pub struct OriginalList {
    user: Pubkey,
    minting_account: Pubkey,
    initializer: Pubkey,
    count: u64
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub owner: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump,
        constraint = !minting_account.freeze_program,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    #[account(mut, constraint = minting_account.key() == wl_list.minting_account)]
    wl_list: Account<'info, WhiteList>,

    #[account(mut, constraint = minting_account.key() == og_list.minting_account)]
    og_list: Account<'info, OriginalList>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 8,
        seeds = [ payer.key().as_ref() ],
        bump,
    )]
    pub user_minting_counter_account: Box<Account<'info, UserMintingAccount>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
#[error]
pub enum MintError {
    #[msg("Not allowed.")]
    NotAllowed,
    #[msg("Mint not active")]
    NotActive,
    #[msg("Invalid stage")]
    InvalidStage,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
}
fn is_admin<'info>(
    minting_account: &Account<'info, MintingAccount>,
    signer: &Signer<'info>,
) -> ProgramResult {
    if minting_account.admin_key != *signer.key {
        return Err(MintError::NotAllowed.into());
    }

    Ok(())
}
