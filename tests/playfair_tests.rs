use playfair::{Playfair, Cipher};

#[test]
fn test_playfair_wiki() {
    let kw = "playfair example";
    let pf = Playfair::new(kw);

    let enc = pf.encrypt("hide the gold in the tree stump");
    assert_eq!(enc, "bmodzbxdnabekudmuixmmouvif");

    let dec = pf.decrypt(enc.as_str());
    assert_eq!(dec, "hidethegoldinthetrexestump");
}

#[test]
fn test_is_and_js_included() {
    let kw = "playfair";
    let pf = Playfair::new(kw);

    let plaintext = "JaneIsAName";
    let encrypt = pf.encrypt(plaintext);

    assert_eq!(encrypt, "bpuncnpqfhku");

    let decrypt = pf.decrypt(encrypt.as_str());
    assert_eq!(decrypt, "ianeisanamex");
}

#[test]
fn test_updating_keyword() {
    let kw = "nonsense";
    let mut pf = Playfair::new(kw);

    let enc_1 = pf.encrypt("hide the gold in the tree stump");

    // Now change the keyword
    pf.update_keyword("playfair example");

    let enc_2 = pf.encrypt("hide the gold in the tree stump");

    // First, we are pretty sure that the two cipher texts should differ
    assert_ne!(enc_1, enc_2);

    // We also know what the output should be for the second one from the first test
    assert_eq!(enc_2, "bmodzbxdnabekudmuixmmouvif");
}
