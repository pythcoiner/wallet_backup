# Wallet backup

## Abstract

This document specifies a format for exporting wallet backup data, including 
accounts, descriptors, associated keys, labels, transactions, and partially 
signed Bitcoin transactions (PSBTs). The format aims to standardize wallet 
backup and restore operations across different Bitcoin wallet implementations.

## Motivation

Bitcoin wallets store various forms of metadata beyond just private keys, 
including account structures, descriptors, labels, and transaction history. 
While BIP39 and BIP32 provide standardized mechanisms for key recovery, they 
do not preserve additional wallet-specific metadata necessary for seamless 
migration between wallets.

A standardized wallet backup format ensures that users can migrate wallets 
without losing critical metadata, reducing vendor lock-in and enabling more 
robust recovery options.

## Rationale

Several wallet implementations store backup data in proprietary formats, 
making migration difficult. This proposal aims to introduce a structured, 
human-readable format, leveraging JSON to store all essential wallet metadata 
in a portable way. The JSON Lines format is chosen for compatibility with 
existing tools and for incremental parsing capabilities.

## Specification

A wallet backup is a UTF-8 encoded text file containing a single valid 
JSON object representing the backup structure. This object includes wallet-level 
metadata, multiple accounts, and associated key data.

### Wallet Backup Structure
- `created_at`: Optional timestamp indicating the backup creation time
- `version`: Optionnal version of the backup format
- `name`: Optional wallet name
- `description`: Optional wallet description
- `accounts`: Array of account objects
- `network`: The network Bitcoin/Testnet/Signet/Regtest
- `proprietary`: Optional JSON object storing application-specific metadata

### Account Object Structure

- `name`: Optional account name
- `description`: Optional account description
- `active`: Optional field to notify if the account is active
- `descriptor`: Output descriptor defining the account structure

  TBD: It may not strictly contain a descriptor but should allow room for:
    - output descriptor (BIP-0380)
    - wallet policy (BIP-0388)
    - silent payments
    - lightning?
    - ark?
    - any arbitrary representation of metadata needed to find & spend coins for an `account`

- `receive_index`: Optional maximum receive index for generated receive addresses
- `change_index`: Optional maximum change index for generated change addresses
- `timestamp`: Optional timestamp indicating account creation time
- `keys`: Optional dictionary of descriptor keys mapped to metadata
- `labels`: Optional labels for transactions, addresses, and keys following BIP-0329
- `transactions`: Optional list of fully confirmed transactions

  TBD: Which transactions should be backed up?
    - Only the transactions spending coins controlled by the account?
    - Also, transactions funding controlled coins? Or only the corresponding outpoints?

- `psbts`: Optional list of unspent but (partially) signed transactions
- `bip39_mnemonic`: Optional mnemonic words following bip39

  Note: the purpose of such backup is to potentially to be stored online, so storing
  mainnet mnemonics is strongly discouraged.

- `proprietary`: Optional JSON object storing account-specific metadata

### Key Object Structure

- `key`: Public key/fingerprint
- `alias`: (optional) User-defined alias
- `role`: (optional) Defines the role of the key in wallet operations
- `key_type`: (optional) Defines the ownership type of the key
- `key_status`: (optional) Status of the key (active/inactive/revoked)
- `bip85_derivation_path`: (optionnal) the bip85 path used to derived this key from the master key

### Key Roles (enum)

- `main`: Key used for normal spending conditions
- `recovery`: Key designated for recovery scenarios
- `inheritance`: Key to inherit funds if the primary user disappears
- `cosigning`: Key designated for policy-enforcing cosigning

### Key Types (enum)

- `internal`: Main user-owned key
- `external`: Heirs or trusted individuals
- `third_party`: Keys held by a service provider

### Key Status (enum)

- `active`: The key is actively used
- `inactive`: The key is not (yet) actively used
- `revoked`: The key have been revoked and MUST not be used anymore

## Importing

- Importing wallets should preserve all metadata they support and 
discard unsupported fields.
- Wallets should warn users if essential data cannot be restored.
- Wallets should ensure that key roles and types are properly mapped 
to their internal structures.

## Security Considerations

- The backup format should not include private keys to avoid unintended 
key exposure.
- Backup should be encrypted to prevent unauthorized access.
- Care should be taken to ensure that proprietary metadata does not 
contain sensitive information.

## Backwards Compatibility

This format is extensible and allows future additions without breaking 
compatibility. Wallets may choose to ignore fields they do not recognize 
while maintaining the structural integrity of the backup.
