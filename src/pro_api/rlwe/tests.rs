use crate::operators::math::Random;
use crate::pro_api;

#[test]
fn test_new_encode_encrypt_1_ciphertext_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode and encrypt and decrypt
    let ct = pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages, &encoder).unwrap();
    let decryptions = ct.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m, d, enc) in izip!(messages.iter(), decryptions.iter(), ct.encoders.iter()) {
        assert_eq_granularity!(m, d, enc);
        assert_eq!(precision1, enc.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encrypt_1_ciphertext_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode
    let enc_messages = encoder.encode(&messages).unwrap();

    // encrypt and decrypt
    let ct = pro_api::RLWE::new_encrypt_1_ciphertext(&sk, &enc_messages).unwrap();
    let decryptions = ct.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m, d, enc) in izip!(messages.iter(), decryptions.iter(), ct.encoders.iter()) {
        assert_eq_granularity!(m, d, enc);
        assert_eq!(precision1, enc.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_on_cst_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(20) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode and encrypt and decrypt
    let ct = pro_api::RLWE::new_encode_encrypt_on_cst(&sk, &messages, &encoder).unwrap();
    let decryptions = ct.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m, d, enc) in izip!(
        messages.iter(),
        decryptions.iter(),
        ct.encoders.chunks(polynomial_size)
    ) {
        assert_eq_granularity!(m, d, enc[0]);
        assert_eq!(precision1, enc[0].nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encrypt_on_cst_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(20) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode
    let enc_messages = encoder.encode(&messages).unwrap();

    // encrypt and decrypt
    let ct = pro_api::RLWE::new_encrypt_on_cst(&sk, &enc_messages).unwrap();
    let decryptions = ct.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m, d, enc) in izip!(
        messages.iter(),
        decryptions.iter(),
        ct.encoders.chunks(polynomial_size)
    ) {
        assert_eq_granularity!(m, d, enc[0]);
        assert_eq!(precision1, enc[0].nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_1_ciphertext_x_extract_1_lwe() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode and encrypt
    let ct = pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages, &encoder).unwrap();

    // convert into LWE secret key
    let lwe_sk = sk.to_lwe_secret_key();

    // extract a filled coefficient
    let index1 = random_index!(nb_messages);
    let ext1 = ct.extract_1_lwe(index1, 0).unwrap();

    // test
    let d1 = ext1.decrypt(&lwe_sk).unwrap();
    assert_eq_granularity!(messages[index1], d1[0], ext1.encoders[0]);
    assert_eq!(precision1, ext1.encoders[0].nb_bit_precision);
}

#[test]
fn test_new_encrypt_on_cst_x_extract_1_lwe() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random number of messages
    let nb_messages: usize = random_index!(20) + 1;

    // random settings for the encoder and some random messages
    let (min1, max1) = generate_random_interval!();
    let (precision1, padding1) = generate_precision_padding!(8, 8);
    let encoder = pro_api::Encoder::new(min1, max1, precision1, padding1).unwrap();
    let messages: Vec<f64> = random_messages!(min1, max1, nb_messages);

    // encode
    let enc_messages = encoder.encode(&messages).unwrap();

    // encrypt
    let ct = pro_api::RLWE::new_encrypt_on_cst(&sk, &enc_messages).unwrap();

    // convert into LWE secret key
    let lwe_sk = sk.to_lwe_secret_key();

    // extract a filled coefficient
    let index1 = random_index!(nb_messages);
    let ext1 = ct.extract_1_lwe(0, index1).unwrap();

    println!("lwe {}", ext1);
    // println!("rlwe key {}", sk);

    // test
    let d1 = ext1.decrypt(&lwe_sk).unwrap();
    assert_eq_granularity!(messages[index1], d1[0], ext1.encoders[0]);
    assert_eq!(precision1, ext1.encoders[0].nb_bit_precision);
}

#[test]
fn test_new_encode_encrypt_1_ciphertext_x_add_constant_static_encoder_inplace_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);
    let nb_messages: usize = random_index!(30) + 10;

    // encoder
    let encoder = pro_api::Encoder::new(min - 10., max + 10., precision, padding).unwrap();

    // two lists of messages
    let messages_1: Vec<f64> = random_messages!(min, max, nb_messages);
    let messages_2: Vec<f64> = random_messages!(-10., 10., nb_messages);

    // encode and encrypt
    let mut ciphertext =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages_1, &encoder).unwrap();

    // addition between ciphertext and messages_2
    ciphertext
        .add_constant_static_encoder_inplace(&messages_2)
        .unwrap();

    // decryption
    let decryptions: Vec<f64> = ciphertext.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages_1.iter(),
        messages_2.iter(),
        decryptions.iter(),
        ciphertext.encoders.iter()
    ) {
        assert_eq_granularity!(m1 + m2, d, e);
        cpt += 1;
        assert_eq!(precision, e.nb_bit_precision);
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encrypt_1_ciphertext_x_add_constant_dynamic_encoder_inplace_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, max2) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // encoder
    let encoder = pro_api::Encoder::new(min1, max1, precision, padding).unwrap();

    // two lists of messages
    let messages_1: Vec<f64> = random_messages!(min1, max1, nb_messages);
    let messages_2: Vec<f64> = random_messages!(min2, max2, nb_messages);

    // encode and encrypt
    let plaintext_1 = pro_api::Plaintext::new_encode(&messages_1, &encoder).unwrap();
    let mut ciphertext = pro_api::RLWE::new_encrypt_1_ciphertext(&sk, &plaintext_1).unwrap();

    // addition between ciphertext and messages_2
    ciphertext
        .add_constant_dynamic_encoder_inplace(&messages_2)
        .unwrap();

    // decryption
    let decryptions: Vec<f64> = ciphertext.decrypt(&sk).unwrap();

    // test
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages_1.iter(),
        messages_2.iter(),
        decryptions.iter(),
        ciphertext.encoders.iter()
    ) {
        assert_eq_granularity!(m1 + m2, d, e);
        cpt += 1;
        assert_eq!(precision, e.nb_bit_precision);
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_x_add_centered_inplace_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min, max) = generate_random_interval!();
    let (precision, padding) = generate_precision_padding!(8, 8);
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // encoders
    let encoder1 = pro_api::Encoder::new(min - 100., max + 100., precision, padding);
    let encoder1 = match encoder1 {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };
    let encoder2 = pro_api::Encoder::new(
        -(max - min) / 2. - 100.,
        (max - min) / 2. + 100.,
        precision,
        padding,
    );
    let encoder2 = match encoder2 {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };

    // two lists of messages
    let messages1: Vec<f64> = random_messages!(min, max, nb_messages);
    let messages2: Vec<f64> = random_messages!(-100., 100., nb_messages);

    // encode and encrypt
    let mut ciphertext1 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages1, &encoder1).unwrap();
    let ciphertext2 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages2, &encoder2).unwrap();

    // addition between ciphertext1 and ciphertext2
    ciphertext1.add_centered_inplace(&ciphertext2).unwrap();

    // decryption
    let decryptions: Vec<f64> = ciphertext1.decrypt(&sk).unwrap();

    // check the precision loss related to the encryption
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages1.iter(),
        messages2.iter(),
        decryptions.iter(),
        ciphertext1.encoders.iter()
    ) {
        assert_eq_granularity!(m1 + m2, d, e);
        assert_eq!(precision, e.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_1_ciphertext_x_add_with_padding_inplace_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, _max2) = generate_random_interval!();
    let max2 = min2 + max1 - min1; // same interval size
    let (precision, mut padding) = generate_precision_padding!(8, 3);
    padding += 1; // at least one bit
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // encoders
    let encoder1 = pro_api::Encoder::new(min1, max1, precision, padding).unwrap();
    let encoder2 = pro_api::Encoder::new(min2, max2, precision, padding).unwrap();

    // two lists of messages
    let messages1: Vec<f64> = random_messages!(min1, max1, nb_messages);
    let messages2: Vec<f64> = random_messages!(min2, max2, nb_messages);

    // encode and encrypt
    let mut ciphertext1 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages1, &encoder1).unwrap();
    let ciphertext2 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages2, &encoder2).unwrap();

    // addition between ciphertext and messages_2
    ciphertext1.add_with_padding_inplace(&ciphertext2).unwrap();

    // decryption
    let decryptions: Vec<f64> = ciphertext1.decrypt(&sk).unwrap();

    // check the precision loss related to the encryption
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages1.iter(),
        messages2.iter(),
        decryptions.iter(),
        ciphertext1.encoders.iter()
    ) {
        assert_eq_granularity!(m1 + m2, d, e);
        assert_eq!(precision, e.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_1_ciphertext_x_sub_with_padding_inplace_x_decrypt() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(40) as i32 + 20);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let sk = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min1, max1) = generate_random_interval!();
    let (min2, _max2) = generate_random_interval!();
    let max2 = min2 + max1 - min1; // same interval size
    let (precision, mut padding) = generate_precision_padding!(8, 3);
    padding += 1; // at least one bit
    let nb_messages: usize = random_index!(polynomial_size - 1) + 1;

    // encoders
    let encoder1 = pro_api::Encoder::new(min1, max1, precision, padding).unwrap();
    let encoder2 = pro_api::Encoder::new(min2, max2, precision, padding).unwrap();

    // two lists of messages
    let messages1: Vec<f64> = random_messages!(min1, max1, nb_messages);
    let messages2: Vec<f64> = random_messages!(min2, max2, nb_messages);

    // encode and encrypt
    let mut ciphertext1 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages1, &encoder1).unwrap();
    let ciphertext2 =
        pro_api::RLWE::new_encode_encrypt_1_ciphertext(&sk, &messages2, &encoder2).unwrap();

    // addition between ciphertext and messages_2
    ciphertext1.sub_with_padding_inplace(&ciphertext2).unwrap();

    // decryption
    let decryptions: Vec<f64> = ciphertext1.decrypt(&sk).unwrap();

    // check the precision loss related to the encryption
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages1.iter(),
        messages2.iter(),
        decryptions.iter(),
        ciphertext1.encoders.iter()
    ) {
        assert_eq_granularity!(m1 - m2, d, e);
        assert_eq!(precision, e.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_on_cst_x_mul_constant_static_encoder_inplace_x_decrypt_with_encoders() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(10) as i32 + 30);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let secret_key = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min, max) = generate_random_centered_interval!();
    let (precision, padding) = generate_precision_padding!(6, 2);
    let nb_messages: usize = random_index!(30) + 10;
    let b = min.abs().min(max.abs()) / 20.;

    // encoders
    let encoder = pro_api::Encoder::new(min, max, precision, padding).unwrap();

    // two lists of messages
    let messages1: Vec<f64> = random_messages!(-b, b, nb_messages);
    let messages2_float: Vec<f64> = random_messages!(-b, b, nb_messages);
    let mut messages2: Vec<i32> = vec![0; nb_messages];
    for (m, f) in izip!(messages2.iter_mut(), messages2_float.iter()) {
        *m = *f as i32;
    }

    // encode and encrypt
    let mut ciphertext =
        pro_api::RLWE::new_encode_encrypt_on_cst(&secret_key, &messages1, &encoder).unwrap();

    // multiplication between ciphertext and messages2
    ciphertext
        .mul_constant_static_encoder_inplace(&messages2)
        .unwrap();

    // decryption
    let (decryptions, dec_encoders) = ciphertext.decrypt_with_encoders(&secret_key).unwrap();

    println!(
        "nb messages: {}, len dec {}",
        nb_messages,
        decryptions.len()
    );

    // check the precision loss related to the encryption
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages1.iter(),
        messages2.iter(),
        decryptions.iter(),
        dec_encoders.iter()
    ) {
        assert_eq_granularity!(*m1 * (*m2 as f64), d, e);
        assert_eq!(precision, e.nb_bit_precision);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}

#[test]
fn test_new_encode_encrypt_on_cst_x_mul_constant_with_padding_inplace_x_decrypt_with_encoders() {
    // generate a secret key
    let dimension: usize = random_index!(4) + 1;
    let polynomial_size: usize = 1024;
    let log_std_dev: i32 = -(random_index!(10) as i32 + 30);
    let params = pro_api::RLWEParams::new(polynomial_size, dimension, log_std_dev).unwrap();
    let secret_key = pro_api::RLWESecretKey::new(&params);

    // random settings
    let (min, max) = generate_random_centered_interval!();
    let precision: usize = random_index!(5) + 3;
    let padding = random_index!(3) + precision;
    let nb_messages: usize = random_index!(30) + 10;
    let nb_bit_padding_mult = precision;
    let b = (random_index!(300) + 3) as f64;

    // encoders
    let encoder = pro_api::Encoder::new(min, max, precision, padding).unwrap();

    // two lists of messages
    let messages1: Vec<f64> = random_messages!(min, max, nb_messages);
    let messages2: Vec<f64> = random_messages!(-b, b, nb_messages);

    // encode and encrypt
    let mut ciphertext =
        pro_api::RLWE::new_encode_encrypt_on_cst(&secret_key, &messages1, &encoder).unwrap();

    // multiplication between ciphertext and messages2
    ciphertext
        .mul_constant_with_padding_inplace(&messages2, b, nb_bit_padding_mult)
        .unwrap();

    // decryption
    let (decryptions, dec_encoders) = ciphertext.decrypt_with_encoders(&secret_key).unwrap();

    // check the precision loss related to the encryption
    let mut cpt: usize = 0;
    for (m1, m2, d, e) in izip!(
        messages1.iter(),
        messages2.iter(),
        decryptions.iter(),
        dec_encoders.iter()
    ) {
        assert_eq_granularity!(m1 * m2, d, e);
        cpt += 1;
    }
    assert_eq!(cpt, nb_messages);
}
