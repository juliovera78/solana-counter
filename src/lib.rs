use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
    //pub name: String,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("primero tengo que acceder a lo que me pasas por parametro, osea que queres hacer, para eso accedo al instructions_data ");
    msg!("eso que queres hacer se divide en 2 , incerementar y cuanto, o restar y cuanto o actualizar a cuanto,y creo una estructura con eso");
    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;
    msg!("desde el main accedo a otra clase llamada Instruction y accedo a CounterInstruction.unpack y le paso instructions_data ");
    msg!("Basicamente convierte el input que le mando en una estrucuta que defino en otra clase para ser mas elegante,crea un struc con un enumeativo asi despues comparo con enumerativos");

    let accounts_iter = &mut accounts.iter();
    msg!("luego accedo a accounts y de ahi tomo el iterador");

    let account = next_account_info(accounts_iter)?;
    msg!("luego del iterador tomo la cuenta primera");

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    msg!("los programas solana no guardan los datos,tomo de otra cuenta los datos, CounterAccount desde la account.data.borrow :{} " , counter_account.counter);

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter = counter_account.counter + args.value;
        }
        CounterInstructions::Decrement(args) => {
            if counter_account.counter >= args.value {
                counter_account.counter = counter_account.counter - args.value;
            }
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    msg!(
        "luego de la operacion queda asi el contador :{} ",
        counter_account.counter
    );

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        let inc_value: u32 = 5u32;
        increment_instruction_data.extend_from_slice(&inc_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            5
        );

        let dec_value = 2u32;
        decrement_instruction_data.extend_from_slice(&dec_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            3
        );

        let update_value = 33u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
