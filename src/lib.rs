let mut result = Transfers {
    transfers: Vec::new(),
};

result.transfers.push(Transfer {
    from: sender,
    to: recipient,
    amount,
});

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const SYSTEM_PROGRAM: &str =
    "11111111111111111111111111111111";

const TOKEN_PROGRAM: &str =
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

const TOKEN_2022_PROGRAM: &str =
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<Transfers, Error> {

    let mut transfers: Vec<Transfer> = Vec::new();

    // Process every successful transaction.
    for transaction in block.transactions() {

        if !transaction.is_successful() {
            continue;
        }

        // walk_instructions includes outer instructions
        // and their inner CPI instructions.
        for instruction in transaction.walk_instructions() {

            let program_id = instruction.program_id().to_string();

            let accounts = instruction.accounts();

            let account_addresses: Vec<String> = accounts
                .iter()
                .map(|account| account.to_string())
                .collect();

            let data = instruction.data();

            // --------------------------------
            // Native SOL transfers
            // --------------------------------
            if program_id == SYSTEM_PROGRAM {

                // System Program Transfer
                // Discriminator: 2
                // Amount: bytes 4..12
                if data.len() >= 12
                    && data[0..4] == [2, 0, 0, 0]
                    && account_addresses.len() >= 2
                {
                    let amount = u64::from_le_bytes(
                        data[4..12].try_into().unwrap()
                    );

                    transfers.push(Transfer {
                        from: account_addresses[0].clone(),
                        to: account_addresses[1].clone(),
                        amount,
                    });
                }

                // System Program TransferWithSeed
                // Discriminator: 11
                if data.len() >= 12
                    && data[0..4] == [11, 0, 0, 0]
                    && account_addresses.len() >= 3
                {
                    let amount = u64::from_le_bytes(
                        data[4..12].try_into().unwrap()
                    );

                    transfers.push(Transfer {
                        from: account_addresses[0].clone(),
                        to: account_addresses[2].clone(),
                        amount,
                    });
                }
            }

            // --------------------------------
            // SPL Token and Token-2022
            // --------------------------------
            else if program_id == TOKEN_PROGRAM
                || program_id == TOKEN_2022_PROGRAM
            {

                // SPL Transfer
                // Discriminator: 3
                // Accounts: source, destination, authority
                // Amount: bytes 1..9
                if data.len() >= 9
                    && data[0] == 3
                    && account_addresses.len() >= 2
                {
                    let amount = u64::from_le_bytes(
                        data[1..9].try_into().unwrap()
                    );

                    transfers.push(Transfer {
                        from: account_addresses[0].clone(),
                        to: account_addresses[1].clone(),
                        amount,
                    });
                }

                // SPL TransferChecked
                // Discriminator: 12
                // Accounts: source, mint, destination, authority
                // Amount: bytes 1..9
                if data.len() >= 10
                    && data[0] == 12
                    && account_addresses.len() >= 3
                {
                    let amount = u64::from_le_bytes(
                        data[1..9].try_into().unwrap()
                    );

                    transfers.push(Transfer {
                        from: account_addresses[0].clone(),
                        to: account_addresses[2].clone(),
                        amount,
                    });
                }
            }
        }
    }

    Ok(Transfers {
        transfers,
    })
}
