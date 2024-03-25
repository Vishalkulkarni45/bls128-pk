use std::io::Error;
use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::{short_weierstrass::{curves:: bls12_381::{compression::BLS12381FieldElement,curve::BLS12381Curve}, point::{Endianness, PointFormat, ShortWeierstrassProjectivePoint}}, traits::IsEllipticCurve}, traits::ByteConversion};

fn public_key_from_bls_secret_key()->Result<ShortWeierstrassProjectivePoint<BLS12381Curve>,Error>{

    //let sk=BLS12381FieldElement::from_hex("0x6C616D6264617370")?;
    let sk =match BLS12381FieldElement::from_hex("0x6C616D6264617370"){
        Ok(sk)=>sk,
        Err(err)=>{
            eprintln!("failed to create secret key because of :{:?}",err);
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Argument",
            ));
        }
    };
    let pk=BLS12381Curve::generator().operate_with_self(*sk.value());
    println!("public_key ={:?}",pk);
    Ok(pk)
}
#[test]
fn test_pk_from_bls_sk(){
    let pk=match public_key_from_bls_secret_key(){
        Ok(pk)=>pk,
        Err(_)=>{
            eprintln!("failed to create public key");
            return;
        }
    };
   let pk_hex=BLS12381FieldElement::from_bytes_le(&pk.serialize(PointFormat::Uncompressed,Endianness::LittleEndian)).unwrap().to_hex();
   println!("uncompressed public key={:?}",pk_hex);

    
}
