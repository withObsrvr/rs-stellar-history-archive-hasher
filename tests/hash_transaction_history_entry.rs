extern crate core;

use std::io;
use stellar_xdr::Error;

#[test]
fn test_happy() {
    let xdr = "AAh/CKBEC6fy2kGDeNx+90PWPbgu2RYwDs0AKqLMJqVtrlrLAAAAAQAAAACaNpLL5YMfjOTdXVEqrAh99LM12sN6He6pHgCRAa1f1QAAAGQACG5PAAAAAwAAAAAAAAADvia3ZzwUSZZxb5ZOnGVRlscuJckAAAAAAAAAAAAAAAAAAAABAAAAAQAAAAD9anuOI/ouLiE/mq2w+EA0AbfK8hHiXe2tI7JEN58A3gAAAAAAAAAAH8I0q455HrnoWzB2A3/sEMGV82k+R/nl19aSArfm6fcAAAp87inUggAAAAAAAAABAa1f1QAAAEAzoLFNOGlepvZMN/HEDs/fp8KwM2w5OaawhwTXvIRzcQBJZts/auvsyHBoVWYzpXNBWujhzGxTj7Z6H/5HBEoKAAAAAA==";
    let bytes = base64::decode(xdr).unwrap();
    let hash = stellar_history_archive_hasher::internal::hash_transaction_history_entry(bytes).unwrap();
    let hash_base64 = base64::encode(hash);
    assert_eq!(hash_base64, "rx/vbqe2TibD5lhk3swAgQzeMud7rPtqZgKdangjWIA=");
}

#[test]
fn test_generalized_transaction_set(){
    let xdr = "AAAB/wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABZD1tW5cNq6NRoiyJvF1DqAl9/ino+WtRe4U4hWdkOU8AAAACAAAAAAAAAAEAAAAAAAAAAQAAAAAAAABkAAAAAwAAAAIAAAAAatBWGhMJ1jPq8YuCAn2dTiQyAYTrC9XOKCigcKfVc4kAACcQAAAB8QAAAA4AAAABAAAAAAAAAAAAAAAAZaAuXwAAAAAAAAABAAAAAQAAAABq0FYaEwnWM+rxi4ICfZ1OJDIBhOsL1c4oKKBwp9VziQAAAAYAAAACQVRQTE4AAAAAAAAAAAAAAGfK1mN4mg51jbX6by6TWghGynQ463doEDgzriqZo9bzf/////////8AAAAAAAAAAafVc4kAAABAbuTksdW+vLS0Y16ysIsQxBGuAxMvf+7b8IW542WfOKDpCHnMrCr0608Xm/1OZYv6AJ7q/kshVNkt4T55roScAwAAAAIAAAAAhjGie08Ys7khHSnNbV+NqH5NwAGVgT4WCHFKecUNLj8AACcQAAAB/AAAAAMAAAABAAAAAAAAAAAAAAAAZaAuXgAAAAAAAAABAAAAAQAAAACGMaJ7TxizuSEdKc1tX42ofk3AAZWBPhYIcUp5xQ0uPwAAAAYAAAACQVRVQUgAAAAAAAAAAAAAAGfK1mN4mg51jbX6by6TWghGynQ463doEDgzriqZo9bzf/////////8AAAAAAAAAAcUNLj8AAABAhdlRHEefM8uit0y4cgOUaBxfQKghrScaUdS45V4CrGqEh2Y+4MWu+u1t4lwa313N/FqmBPK3pzUaS3fUrUprBQAAAAIAAAAAQFBgCKbHnBpAdoCm0qDLZr8gAcxJ73oRO3nCIw5KkpcAHoSAAAAB8AAAAAgAAAABAAAAAAAAAAAAAAAAZaAtawAAAAEAAAAJcHNwYjoyMDM1AAAAAAAAAgAAAAEAAAAAipbrVDeRk91DqBNgVJDbSRMKIqRjopwRbELV4KSGTMUAAAABAAAAAON96Psq3pllN7fPHCqX6R0fiTBUeOCR85gRp4uoaghGAAAAAkFUVVNEAAAAAAAAAAAAAABnytZjeJoOdY21+m8uk1oIRsp0OOt3aBA4M64qmaPW8wAAAAAAOHUgAAAAAQAAAACKlutUN5GT3UOoE2BUkNtJEwoipGOinBFsQtXgpIZMxQAAAAEAAAAA433o+yremWU3t88cKpfpHR+JMFR44JHzmBGni6hqCEYAAAACQVRVQUgAAAAAAAAAAAAAAGfK1mN4mg51jbX6by6TWghGynQ463doEDgzriqZo9bzAAAAAAaOd4AAAAAAAAAAAg5KkpcAAABARNE8Ccc3tUKA+1jPOdedJnTQCiZdy+LoJVqaImZ4YbJDi/7HQMbICxLwgY+DlWIktO9RSFKrkE3U1SRVxS9pAqSGTMUAAABAklPqeWTOIS5oFVUpi6wKfaEC56w5CIDOJCeiJA2uhKXL2PZICUB3E6RUpFlFJbACPPtjdq0qLeYoXYwPSzqEBQAAAAAAAAAA";
    let bytes = base64::decode(xdr).unwrap();
    let hash = stellar_history_archive_hasher::internal::hash_transaction_history_entry(bytes).unwrap();
    let hash_base64 = base64::encode(hash);
    assert_eq!(hash_base64, "FKACDbEPvEJoPWbUkhJwkPSWugfv9uj6PnYl1FfbA6E=");
}

#[test]
fn test_xdr_not_transaction_history_entry() {
    let xdr = "AAh/CAAAAAEGJh/ut6Pw5WiDtPWF5h94fONDaUn+YwXn7Wdt5pFAogAAAAAAAABkAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
    let bytes = base64::decode(xdr).unwrap();
    let result = stellar_history_archive_hasher::internal::hash_transaction_history_entry(bytes);
    assert_eq!(Err(Error::Io(io::ErrorKind::UnexpectedEof.into())), result)
}