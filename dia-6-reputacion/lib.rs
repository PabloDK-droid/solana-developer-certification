use anchor_lang::prelude::*;

declare_id!("5ktee6vdNAh8RFmtN6A5kV1GshvUEGtiFrtG4ZyJfAyc");

#[program]
pub mod reputacion {
    use super::*;

    pub fn registrar_usuario(ctx: Context<RegistrarUsuario>, nombre: String) -> Result<()> {
        require!(nombre.len() <= 32, ErrorReputacion::NombreMuyLargo);
        let perfil = &mut ctx.accounts.perfil_usuario;
        perfil.autoridad = ctx.accounts.usuario.key();
        perfil.nombre = nombre.clone();
        perfil.puntos = 0;
        perfil.bump = ctx.bumps.perfil_usuario;
        msg!("Usuario registrado: {}", nombre);
        Ok(())
    }

    pub fn otorgar_puntos(ctx: Context<GestionarPuntos>, cantidad: u64, motivo: String) -> Result<()> {
        require!(cantidad > 0, ErrorReputacion::CantidadInvalida);
        require!(motivo.len() <= 64, ErrorReputacion::MotivoMuyLargo);
        let perfil = &mut ctx.accounts.perfil_usuario;
        perfil.puntos = perfil.puntos.checked_add(cantidad)
            .ok_or(ErrorReputacion::Overflow)?;
        msg!("Puntos otorgados: {} | Motivo: {} | Total: {}", cantidad, motivo, perfil.puntos);
        Ok(())
    }

    pub fn quitar_puntos(ctx: Context<GestionarPuntos>, cantidad: u64, motivo: String) -> Result<()> {
        require!(cantidad > 0, ErrorReputacion::CantidadInvalida);
        require!(motivo.len() <= 64, ErrorReputacion::MotivoMuyLargo);
        let perfil = &mut ctx.accounts.perfil_usuario;
        require!(perfil.puntos >= cantidad, ErrorReputacion::PuntosInsuficientes);
        perfil.puntos -= cantidad;
        msg!("Puntos quitados: {} | Motivo: {} | Total: {}", cantidad, motivo, perfil.puntos);
        Ok(())
    }

    pub fn consultar_reputacion(ctx: Context<ConsultarReputacion>) -> Result<()> {
        let perfil = &ctx.accounts.perfil_usuario;
        msg!("Usuario: {} | Puntos: {}", perfil.nombre, perfil.puntos);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarUsuario<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + 32 + 4 + 32 + 8 + 4 + 64 + 1,
        seeds = [b"perfil", usuario.key().as_ref()],
        bump
    )]
    pub perfil_usuario: Account<'info, PerfilUsuario>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarPuntos<'info> {
    #[account(
        mut,
        seeds = [b"perfil", perfil_usuario.autoridad.as_ref()],
        bump = perfil_usuario.bump
    )]
    pub perfil_usuario: Account<'info, PerfilUsuario>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct ConsultarReputacion<'info> {
    #[account(
        seeds = [b"perfil", perfil_usuario.autoridad.as_ref()],
        bump = perfil_usuario.bump
    )]
    pub perfil_usuario: Account<'info, PerfilUsuario>,
}

#[account]
pub struct PerfilUsuario {
    pub autoridad: Pubkey,
    pub nombre: String,
    pub puntos: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorReputacion {
    #[msg("El nombre no puede superar 32 caracteres")]
    NombreMuyLargo,
    #[msg("La cantidad debe ser mayor a 0")]
    CantidadInvalida,
    #[msg("El motivo no puede superar 64 caracteres")]
    MotivoMuyLargo,
    #[msg("No hay suficientes puntos")]
    PuntosInsuficientes,
    #[msg("Overflow en puntos")]
    Overflow,
}
