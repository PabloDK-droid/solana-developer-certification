use anchor_lang::prelude::*;

declare_id!("5ktee6vdNAh8RFmtN6A5kV1GshvUEGtiFrtG4ZyJfAyc");

#[program]
pub mod contador {
    use super::*;

    pub fn inicializar(ctx: Context<Inicializar>) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_contador;
        cuenta.contador = 0;
        cuenta.autoridad = ctx.accounts.usuario.key();
        cuenta.bump = ctx.bumps.cuenta_contador;
        msg!("Contador PDA inicializado");
        Ok(())
    }

    pub fn incrementar(ctx: Context<Incrementar>) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_contador;
        require!(
            cuenta.autoridad == ctx.accounts.usuario.key(),
            ErrorContador::NoAutorizado
        );
        cuenta.contador += 1;
        msg!("Contador: {}", cuenta.contador);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Inicializar<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + 8 + 32 + 1,
        seeds = [b"contador", usuario.key().as_ref()],
        bump
    )]
    pub cuenta_contador: Account<'info, CuentaContador>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Incrementar<'info> {
    #[account(
        mut,
        seeds = [b"contador", usuario.key().as_ref()],
        bump = cuenta_contador.bump
    )]
    pub cuenta_contador: Account<'info, CuentaContador>,
    pub usuario: Signer<'info>,
}

#[account]
pub struct CuentaContador {
    pub contador: u64,
    pub autoridad: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum ErrorContador {
    #[msg("No tienes autoridad para incrementar este contador")]
    NoAutorizado,
}
