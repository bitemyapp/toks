use toks::{DecodeFlags as D, EncodeFlags as E, ScratchOptions, Tier, Tokenizer};

const BYTE: &[u8] = include_bytes!("../../tests/data/compile/nosplit.json");
const SPM: &[u8] = include_bytes!("../../tests/data/spm/llamalike.json");
const UNI: &[u8] = include_bytes!("../../tests/data/unigram/bound_bf_meta.json");
const WP: &[u8] = br###"{
    "normalizer":{"type":"BertNormalizer","clean_text":true,"handle_chinese_chars":true,"strip_accents":null,"lowercase":true},
    "pre_tokenizer":{"type":"BertPreTokenizer"},
    "decoder":{"type":"WordPiece","prefix":"##","cleanup":true},
    "model":{"type":"WordPiece","unk_token":"[UNK]","continuing_subword_prefix":"##","max_input_chars_per_word":100,"vocab":{"[UNK]":0,"hello":1,"world":2,"a":3,"b":4,"cafe":5}}
}"###;

#[test]
fn exact_prefix_reuse_and_raw_bytes() {
    let tokenizer = Tokenizer::from_bytes(BYTE, Tier::Auto).unwrap();
    let mut encoder = tokenizer.encoder(ScratchOptions::default()).unwrap();
    let text = b"hello\x00world \xf0\x93\x80\x80\xff\xc0\x80";
    let ids = encoder.encode(text, E::ALL).unwrap();
    assert_eq!(tokenizer.decode(&ids, D::RAW).unwrap(), text);
    for n in 0..=ids.len() + 1 {
        let mut out = vec![u32::MAX; n];
        assert_eq!(
            encoder.encode_into(text, E::ALL, &mut out).unwrap(),
            ids.len()
        );
        assert_eq!(&out[..n.min(ids.len())], &ids[..n.min(ids.len())]);
    }
    let mut reused = Vec::with_capacity(4096);
    let before = reused.as_ptr();
    encoder.encode_to(text, E::ALL, &mut reused).unwrap();
    assert_eq!(reused, ids);
    assert_eq!(before, reused.as_ptr());
    assert!(encoder
        .encode_to(text, E::from_bits(u32::MAX), &mut reused)
        .is_err());
    assert!(reused.is_empty());
    encoder.clear_cache();
    assert_eq!(encoder.encode(text, E::ALL).unwrap(), ids);
    let mut bytes = Vec::new();
    for id in &ids {
        let token = tokenizer.token(*id).unwrap();
        bytes.extend_from_slice(token);
        assert_eq!(tokenizer.token_to_id(token), Some(*id));
    }
    assert_eq!(bytes, text);
    assert!(tokenizer.token(u32::MAX).is_none());
    assert_eq!(tokenizer.id_flags(u32::MAX).unwrap_err().code, -7);
}

#[test]
fn load_diagnostics_and_options() {
    let error = Tokenizer::from_bytes(b"{", Tier::Auto).err().unwrap();
    assert_eq!(error.code, -2);
    assert!(!error.message.is_empty());
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/compile/nosplit.json");
    let file = Tokenizer::from_file(path, Tier::Scalar).unwrap();
    assert_eq!(file.info().tier, Tier::Scalar as u32);
    assert_eq!(
        file.info().source_sha256,
        Tokenizer::from_bytes(BYTE, Tier::Auto)
            .unwrap()
            .info()
            .source_sha256
    );
    for options in [
        ScratchOptions {
            memo_mib: Some(4096),
            cache_mib: 0,
        },
        ScratchOptions {
            memo_mib: None,
            cache_mib: 3,
        },
    ] {
        assert_eq!(file.encoder(options).err().unwrap().code, -10);
    }
    assert_eq!(file.decoder(D::RAW).err().unwrap().code, -10);
    assert_eq!(file.decode(&[u32::MAX], D::ALL).unwrap_err().code, -7);
    assert_eq!(Tokenizer::version(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn all_families_and_scalar_match() {
    for data in [BYTE, SPM, UNI, WP] {
        let scalar = Tokenizer::from_bytes(data, Tier::Scalar).unwrap();
        let native = Tokenizer::from_bytes(data, Tier::Auto).unwrap();
        let mut a = scalar
            .encoder(ScratchOptions {
                memo_mib: Some(0),
                cache_mib: 0,
            })
            .unwrap();
        let mut b = native.encoder(ScratchOptions::default()).unwrap();
        for text in ["", "hello world", " café 中 👩🏽‍🚀\r\n", "<unk>  a\t b"] {
            for mode in [
                E::ALL,
                E::NONSPECIAL,
                E::NONE,
                E::CONTINUATION,
                E::NO_POSTPROCESS | E::NO_PAD | E::NO_TRUNCATE,
            ] {
                let ids = a.encode(text.as_bytes(), mode).unwrap();
                assert_eq!(b.encode(text.as_bytes(), mode).unwrap(), ids);
                assert_eq!(
                    a.pieces(text.as_bytes(), mode).unwrap(),
                    b.pieces(text.as_bytes(), mode).unwrap()
                );
                assert_eq!(
                    scalar.decode(&ids, D::ALL).unwrap(),
                    native.decode(&ids, D::ALL).unwrap()
                );
            }
        }
    }
}

#[test]
fn retained_owners_and_concurrent_encoders() {
    let tokenizer = Tokenizer::from_bytes(BYTE, Tier::Auto).unwrap();
    let mut encoder = tokenizer.encoder(ScratchOptions::default()).unwrap();
    let mut decoder = tokenizer.decoder(D::ALL).unwrap();
    let threads: Vec<_> = (0..4)
        .map(|i| {
            let clone = tokenizer.clone();
            std::thread::spawn(move || {
                let mut e = clone.encoder(ScratchOptions::default()).unwrap();
                let text = format!("thread {i}: ☀").repeat(200);
                for _ in 0..20 {
                    let ids = e.encode(text.as_bytes(), E::ALL).unwrap();
                    assert_eq!(clone.decode(&ids, D::ALL).unwrap(), text.as_bytes());
                }
            })
        })
        .collect();
    drop(tokenizer);
    let ids = encoder.encode(b"retained", E::ALL).unwrap();
    drop(encoder);
    assert_eq!(decoder.push(&ids).unwrap(), b"retained");
    assert!(decoder.flush().unwrap().is_empty());
    for t in threads {
        t.join().unwrap();
    }
}

#[test]
fn stream_grows_hold_and_preserves_failed_push() {
    let tokenizer = Tokenizer::from_bytes(SPM, Tier::Auto).unwrap();
    let mut encoder = tokenizer.encoder(ScratchOptions::default()).unwrap();
    let text = "𓀀".repeat(300);
    let ids = encoder.encode(text.as_bytes(), E::ALL).unwrap();
    let expected = tokenizer.decode(&ids, D::ALL).unwrap();
    assert_eq!(expected, text.as_bytes());
    let mut decoder = tokenizer.decoder(D::ALL).unwrap();
    for _ in 0..2 {
        let mut result = decoder.push(&ids).unwrap();
        assert_eq!(decoder.push(&[u32::MAX]).unwrap_err().code, -7);
        result.extend(decoder.flush().unwrap());
        assert_eq!(result, expected);
    }
    for chunk in [1, 7, 44, 45, 100] {
        let mut result = Vec::new();
        for part in ids.chunks(chunk) {
            result.extend(decoder.push(part).unwrap());
        }
        result.extend(decoder.flush().unwrap());
        assert_eq!(result, expected);
    }
}

#[test]
fn padding_and_template() {
    let tokenizer = Tokenizer::from_bytes(
        include_bytes!("../../tests/data/breadth/trunc_pad.json"),
        Tier::Auto,
    )
    .unwrap();
    let template = tokenizer.template().unwrap();
    assert_eq!(
        template.ids.len(),
        (tokenizer.info().n_template_prefix + tokenizer.info().n_template_suffix) as usize
    );
    assert_eq!(template.ids.len(), template.type_ids.len());
    let mut encoder = tokenizer.encoder(ScratchOptions::default()).unwrap();
    assert_eq!(encoder.encode(b"hello world", E::ALL).unwrap().len(), 12);
    let full = encoder
        .encode(&b"hello world ".repeat(100), E::NO_PAD | E::NO_TRUNCATE)
        .unwrap();
    assert!(full.len() > 12);
    assert!(tokenizer.added(0).is_some());
    assert!(tokenizer.added(u32::MAX).is_none());
}

#[test]
fn expanding_normalizer_retries_output_sizing() {
    let json = std::str::from_utf8(BYTE).unwrap().replace(
        "\"normalizer\": null",
        "\"normalizer\": {\"type\":\"NFKC\"}",
    );
    let tokenizer = Tokenizer::from_bytes(json.as_bytes(), Tier::Auto).unwrap();
    let mut encoder = tokenizer.encoder(ScratchOptions::default()).unwrap();
    let text = "ﷺ".repeat(128);
    let ids = encoder.encode(text.as_bytes(), E::ALL).unwrap();
    assert!(ids.len() > text.len() + 64);
    assert_eq!(
        tokenizer.decode(&ids, D::ALL).unwrap(),
        "صلى الله عليه وسلم".repeat(128).as_bytes()
    );
}
