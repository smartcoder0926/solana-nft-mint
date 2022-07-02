use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("2oSArBTstkKKkhrPPTJC4PXSou3zUPPmTTemQJqYnjAc");
pub mod constants {
    pub const MINTING_PDA_SEED: &[u8] = b"wallet_nft_minting";
}

#[program]
pub mod wallet_nft_mint {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _nonce_minting: u8,
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
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_list: Vec<Pubkey>,
    ) -> ProgramResult {
        for new_og in new_og_list.iter() {
            if ctx
                .accounts
                .minting_account
                .og_list
                .iter()
                .find(|&og| og == new_og)
                == None
            {
                ctx.accounts.minting_account.og_list.push(*new_og);
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn remove_og_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        old_og_list: Vec<Pubkey>,
    ) -> ProgramResult {
        for old_og in old_og_list.iter() {
            match ctx
                .accounts
                .minting_account
                .og_list
                .iter()
                .position(|og| og == old_og)
            {
                Some(index) => {
                    ctx.accounts.minting_account.og_list.remove(index);
                }
                None => {}
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_wl_list(
        ctx: Context<CreateWhiteList>,
        user: Pubkey,
    ) -> ProgramResult {
        
        let whitelist = &mut ctx.accounts.whitelist;

        whitelist.user = user;
        whitelist.minting_account = ctx.accounts.minting_account.key();
        whitelist.initializer = ctx.accounts.admin.key();
        whitelist.count = 1;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.initializer))]
    pub fn remove_wl_list(
        ctx: Context<RemoveWhiteList>,
    ) -> ProgramResult {

        ctx.accounts
                .whitelist
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
            // WL
            match ctx
                .accounts
                .minting_account
                .og_list
                .iter()
                .position(|og| *og == ctx.accounts.payer.key())
            {
                Some(_index) => {
                    _max_num = ctx.accounts.minting_account.og_max;
                    _price = ctx.accounts.minting_account.og_price;
                    _state = 1; // WL
                }
                None => {}
            }

            if ctx.accounts.whitelist.count == 1 {
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
#[instruction(_nonce_minting: u8)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = initializer,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump = _nonce_minting,
        space = 32 * 10 + 32 * 3 * 50
        // space = 308000
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
    pub freeze_program: bool,
    pub aury_vault: Pubkey,
    pub authorized_creator: Pubkey,
    pub max_supply: u64,
    pub og_max: u64,
    pub wl_max: u64,
    pub public_max: u64,
    pub og_price: u64,
    pub wl_price: u64,
    pub public_price: u64,
    pub og_list: Vec<Pubkey>,
    pub wl_list: Vec<Pubkey>,
    pub public_list: Vec<Pubkey>,
    pub cur_num: u64,
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
    seeds = [
        "nftminting".as_bytes(),
        "whitelist".as_bytes(),
        minting_account.key().as_ref(),
        user.as_ref(),
    ],
    bump,
    payer = admin,
    space = 112,
    )]
    whitelist: Account<'info, WhiteList>,

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

    #[account(mut, has_one = initializer, constraint = minting_account.key() == whitelist.minting_account)]
    whitelist: Account<'info, WhiteList>,

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

    #[account(mut, constraint = minting_account.key() == whitelist.minting_account)]
    whitelist: Account<'info, WhiteList>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        init_if_needed,
        payer = payer,
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
