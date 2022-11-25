use playfair_rs::{Playfair, Cipher};

#[test]
fn playfair_wiki_test() {
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

    let plaintext = "JudgeJoeBrownIsCool";
    let encrypt = pf.encrypt(plaintext);

    assert_eq!(encrypt, "eprmnengcbqvuexksvvr");

    let decrypt = pf.decrypt(encrypt.as_str());
    assert_eq!(decrypt, "iudgeioebrowniscoxol");
}
