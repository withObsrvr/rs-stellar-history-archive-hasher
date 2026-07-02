use js_sys::{Uint8Array};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub unsafe fn hash_transaction_history_result_entry(bytes: &[u8]) -> Uint8Array {
    return Uint8Array::view(&internal::hash_transaction_history_result_entry(bytes).unwrap());
}

#[wasm_bindgen]
pub unsafe fn hash_transaction_history_entry(bytes: &[u8]) -> Uint8Array {
    return Uint8Array::view(&internal::hash_transaction_history_entry(bytes).unwrap());
}

pub mod internal {
    use stellar_xdr::{Error, Limits, ReadXdr, TransactionEnvelope, TransactionHistoryEntry, TransactionHistoryEntryExt, TransactionHistoryResultEntry, WriteXdr};
    use sha2::{Digest, Sha256};

    pub fn hash_transaction_history_result_entry(bytes: impl AsRef<[u8]>) -> Result<[u8; 32], Error> {
        let transaction_history_result_entry = TransactionHistoryResultEntry::from_xdr(bytes, Limits::none())?;
        let tx_result_set_xdr = transaction_history_result_entry.tx_result_set.to_xdr(Limits::none())?;

        return Ok(Sha256::digest(&tx_result_set_xdr).into());
    }

    struct TxEnvelope {
        hash: [u8; 32],
        tx: Vec<u8>,
    }

    pub fn hash_transaction_history_entry(bytes: impl AsRef<[u8]>) -> Result<[u8; 32], Error> {
        let transaction_history_entry = TransactionHistoryEntry::from_xdr(bytes, Limits::none())?;

        return match transaction_history_entry.ext {
            TransactionHistoryEntryExt::V1(generalized) => {
                let mut hasher = Sha256::new();
                sha2::Digest::update(&mut hasher, &generalized.to_xdr(Limits::none())?);
                Ok(hasher.finalize().into())
            }
            _ => {
                let mut transaction_envelopes: Vec<TxEnvelope> = Vec::new();
                let txs_vec: Vec<TransactionEnvelope> = transaction_history_entry.tx_set.txs.into_vec();
                for tx in txs_vec {
                    let tx_envelope_xdr = tx.to_xdr(Limits::none())?;
                    transaction_envelopes.push(TxEnvelope {
                        hash: Sha256::digest(&tx_envelope_xdr).into(),
                        tx: tx_envelope_xdr,
                    });
                }
                let mut hasher = Sha256::new();
                sha2::Digest::update(&mut hasher, &transaction_history_entry.tx_set.previous_ledger_hash.to_xdr(Limits::none())?);

                transaction_envelopes.sort_unstable_by(|a, b| { a.hash.cmp(&b.hash) });
                for sorted_tx in transaction_envelopes {
                    sha2::Digest::update(&mut hasher, &sorted_tx.tx);
                }

                Ok(hasher.finalize().into())
            }
        }
    }
}