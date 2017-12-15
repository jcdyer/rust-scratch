extern crate argon2rs;

fn main() {
    let verifier = argon2rs::verifier::Encoded::default2i(b"12345", b"naclnacl", &[][..], &[][..]);
    let u = verifier.to_u8();
    println!("{}", ::std::str::from_utf8(&u).unwrap());
    let new_verifier = argon2rs::verifier::Encoded::from_u8(&u).unwrap();
    println!("Verify true: {:?}", new_verifier.verify(b"12345"));
    println!("Verify false: {:?}", new_verifier.verify(b"password"));

}
