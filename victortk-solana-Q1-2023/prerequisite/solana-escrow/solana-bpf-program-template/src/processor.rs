use solana_program:: {
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
};

use crate::instruction:EscrowInstruction ;

pub struct Processor;

impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])-> ProgramResult {
      let instruction = EscrowInstruction::unpck(instruction_data)?;

      match instruction {
        EscrowInstruction::InitEscrow {amount} => {
          msg!("Instruction: InitEscrow");
          self::process_init_escrow(accounts, amount, program_id)
        }
      }
  }
}