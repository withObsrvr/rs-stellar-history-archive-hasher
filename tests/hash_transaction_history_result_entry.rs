extern crate core;

use stellar_xdr::Error;

#[test]
fn test(){
    let xdr = "AAh/CAAAAAEGJh/ut6Pw5WiDtPWF5h94fONDaUn+YwXn7Wdt5pFAogAAAAAAAABkAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
    let bytes = base64::decode(xdr).unwrap();
    let hash = stellar_history_archive_hasher::internal::hash_transaction_history_result_entry(bytes).unwrap();
    assert_eq!(hash, [197, 223, 135, 233, 220, 177, 122, 123, 25, 178, 71, 248, 30, 27, 251, 57, 15, 1, 7, 161, 220, 118, 242, 129, 67, 187, 32, 207, 36, 163, 233, 59]);
}

#[test]
fn test_xdr_not_transaction_history_result_entry(){
    let xdr = "AAh/Gg9MQXSBI87Fg64T0aAFEbpbYmnPC399aiRzRbYzCvD1AAAAAgAAAADbHA6xiKB1+G79mVqpsHMOleOqKa5mxDpP5KEp/Xdz9wAAAGQACG4fAAAABgAAAAAAAAAD9Q3Uczv1nI3SEJaSQ4d/3PGK0dIAAAAAAAAAAAAAAAAAAAABAAAAAQAAAAD9anuOI/ouLiE/mq2w+EA0AbfK8hHiXe2tI7JEN58A3gAAAAAAAAAAAOQlmS/nmcmj3qCrFPr9+25a8+yUrWMTsDdWyYzXqHsAAAAN6fk41gAAAAAAAAAB/Xdz9wAAAEDjOfwPZifNg1eVREMcI4YOtJL5gxiW1x76iCbSI3+Bt4ZjCOrZjbOJaq3CCHucqhquCEVM587WCtN1KOqzz10IAAAAAIEi4R7juq15ymL00DNlAddunyFT4FyUD4muC4t3bobdAAAAZAAIbjoAAAAEAAAAAAAAAAN3OvqJtxRHK4+6lTz3pC7EK0LKogAAAAAAAAAAAAAAAAAAAAEAAAABAAAAAP1qe44j+i4uIT+arbD4QDQBt8ryEeJd7a0jskQ3nwDeAAAAAAAAAAB8/joC+DFmyt1S/N2Y8VIBpQBOUJyEUmYM18dMl2DWWAAAAAAML3VAAAAAAAAAAAF3bobdAAAAQBBSQWYbAEIdY1ePxHzRw86KrQyhpOmUod+JjpcVTRvkhvPXIAEiL+IcwUqa4fVvecg+H8iEx96xq8A0+VW5Dw8AAAAA";
    let bytes = base64::decode(xdr).unwrap();
    let result = stellar_history_archive_hasher::internal::hash_transaction_history_result_entry(bytes);
    assert_eq!(Err(Error::Invalid),result)
}