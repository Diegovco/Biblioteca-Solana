use anchor_lang::prelude::*;

declare_id!("9PSzcynaaxbm3aZQRBbvmRgfr6jKFNhWoaY4tNX6sDgz");

#[program]
pub mod clases_universidad {
    use super::*;

//Boleta de inscricipcion que guarda la prime materia 
    pub fn inscribir(ctx: Context<Inscribir>,materia: u16) -> Result<()> {
        ctx.accounts.mi_boleta.materia = materia;
        Ok(())
    }
//modificar la materia que ya habiamos guardado 
pub fn cambiar(ctx: Context <Cambiar>, nueva_materia : u16) -> Result<()> {
    ctx.accounts.mi_boleta.materia = nueva_materia;
    Ok(())
}
//Borrar la boleta 
pub fn borrar(_ctx: Context<Borrar>) -> Result<()> {
    Ok(())
}
}


#[derive(Accounts)]
pub struct Inscribir<'info> {
// inti es igual a creae una cuenta poir primera vez 
// player = alumno significa: Cerrar la cuenta y rembolso del dinero 
#[account(init, payer = alumno, space = 8 + 4, seeds = [b"boleta", alumno.key().as_ref()], bump)]
pub mi_boleta: Account<'info, Boleta>,
#[account(mut)]
pub alumno: Signer<'info>,// firma de la transaccion
pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct Cambiar<'info> {
    //mut significa modificar los datos 
    #[account(mut, seeds =[b"boleta",alumno.key().as_ref()],bump)]
    pub mi_boleta: Account<'info, Boleta>,
    pub alumno : Signer<'info>,
}
#[derive(Accounts)]
pub struct Borrar<'info>{
    //close= alumno significa cierra la cuenta y rembolsar el dinero 
    #[account(mut,close=alumno,seeds = [b"boleta", alumno.key().as_ref()],bump )]
    pub mi_boleta: Account<'info,Boleta>,
    #[account(mut)]
    pub alumno : Signer<'info>,
}

// muestra de como se miraria la boleta 
#[account]
pub struct Boleta {
    pub materia: u16
}
