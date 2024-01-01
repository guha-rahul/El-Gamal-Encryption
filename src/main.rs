#![allow(unused)]
extern crate bn;

extern crate rand;  
use bn::{G1, Fr};
use bn::Group;

extern crate bincode;
extern crate rustc_serialize;
use bincode::SizeLimit::Infinite; 
use bincode::rustc_serialize::{encode};
use rustc_serialize::{Encodable};
use rustc_serialize::hex::{ToHex};

pub fn into_hex<T: Encodable>(obj: T) -> Option<String> {
   encode(&obj, Infinite).ok().map(|e| e.to_hex())
}

fn diffie_hellman() {
   println!("Base point {:?}", into_hex(&G1::one()).unwrap());

   let rng = &mut rand::thread_rng();

   // Construct private keys  
   let a = Fr::random(rng);
   let b = Fr::random(rng);
  // let c = Fr::random(rng);

   println!("\n\nAlice priv: {:?}\n\nBob priv {:?}\n\n",  
           into_hex(a).unwrap(),
           into_hex(b).unwrap());


   // Public keys
   let alice_pk = G1::one() * a;
   let bob_pk = G1::one() * b;  
   

   println!("\n\nAlice pub: {:?}\n\nBob pub {:?}\n\n",
           into_hex(alice_pk).unwrap(), 
           into_hex(bob_pk).unwrap());


    // Round 1
    let alice_1 = bob_pk * b; 
    let bob_1 = alice_1 * a;
    

    // Round 2
    let alice_share =alice_1 * a;
    let bob_share = bob_1 * b;

    print!("\n\nAlice share: {:?}\n\nBob share {:?}\n\n",  into_hex(alice_share).unwrap(), into_hex(bob_share).unwrap());

 
}


fn elgamal() {
    println!("Base point {:?}", into_hex(&G1::one()).unwrap());
 
    let rng = &mut rand::thread_rng();
 
    // Construct private keys  
    let a = Fr::random(rng);
    let b = Fr::random(rng);
   // let c = Fr::random(rng);
 
    println!("\n\nAlice priv: {:?}\n\nBob priv {:?}\n\n",  
            into_hex(a).unwrap(),
            into_hex(b).unwrap());
 
 
    // Public keys
    let alice_pk = G1::one() * a;
    let bob_pk = G1::one() * b;  
    
 
    println!("\n\nAlice pub: {:?}\n\nBob pub {:?}\n\n",
            into_hex(alice_pk).unwrap(), 
            into_hex(bob_pk).unwrap());
 
 
     // Round 1
     let alice_1 = bob_pk * b; 
     let bob_1 = alice_1 * a;
     
 
     // Round 2
     let alice_share =alice_1 * a;
     let bob_share = bob_1 * b;
     let m = 69i32;
     let m_as_fr = Fr::from(m);
     let message = G1::from(m_as_fr);
     let encryption = alice_share +message;

     print!("\n\nAlice share: {:?}\n\nBob share {:?}\n\n",  into_hex(alice_share).unwrap(), into_hex(bob_share).unwrap());
 
  
 }
 
fn main(){
    diffie_hellman();
    elgamal();
}