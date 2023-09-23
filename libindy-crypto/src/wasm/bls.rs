use crate::wasm_bindgen::prelude::*;

use crate::bls;
use crate::errors::IndyCryptoError;
use crate::errors::ToErrorCode;
use serde;

impl From<IndyCryptoError> for JsValue {
    fn from(err: IndyCryptoError) -> JsValue {
        let error_code = err.to_error_code();
        JsValue::from_serde(&error_code).unwrap()
    }
}

fn convert_from_js<T>(val: &JsValue) -> Result<T, IndyCryptoError>
where
    for<'a> T: serde::Deserialize<'a>,
{
    match val.into_serde() {
        Ok(unwrapped) => Ok(unwrapped),
        Err(_) => Err(IndyCryptoError::InvalidStructure(
            "Invalid argument".to_string(),
        )),
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsGenerator() -> Result<JsValue, JsValue> {
    let gen = bls::Generator::new()?;
    Ok(JsValue::from_serde(&gen).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsGeneratorAsBytes(generator: &JsValue) -> Result<Vec<u8>, JsValue> {
    let gen: bls::Generator = convert_from_js(generator)?;
    Ok(gen.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsGeneratorFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let gen = bls::Generator::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&gen).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSignKey(seed: Option<Vec<u8>>) -> Result<JsValue, JsValue> {
    let seedOption = seed.as_ref().map(|v| v.as_slice());
    let sk = bls::SignKey::new(seedOption)?;
    Ok(JsValue::from_serde(&sk).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSignKeyAsBytes(signKey: &JsValue) -> Result<Vec<u8>, JsValue> {
    let sk: bls::SignKey = convert_from_js(signKey)?;
    Ok(sk.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSignKeyFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let sk = bls::SignKey::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&sk).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSign(message: &[u8], signKey: &JsValue) -> Result<JsValue, JsValue> {
    let sk: bls::SignKey = convert_from_js(signKey)?;
    let signature: bls::Signature = bls::Bls::sign(message, &sk)?;
    Ok(JsValue::from_serde(&signature).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerKey(generator: &JsValue, signKey: &JsValue) -> Result<JsValue, JsValue> {
    let gen: bls::Generator = convert_from_js(generator)?;
    let sk: bls::SignKey = convert_from_js(signKey)?;
    let vk = bls::VerKey::new(&gen, &sk)?;
    Ok(JsValue::from_serde(&vk).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerKeyAsBytes(verKey: &JsValue) -> Result<Vec<u8>, JsValue> {
    let vk: bls::VerKey = convert_from_js(verKey)?;
    Ok(vk.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerKeyFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let vk = bls::VerKey::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&vk).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsProofOfPossession(verKey: &JsValue, signKey: &JsValue) -> Result<JsValue, JsValue> {
    let vk: bls::VerKey = convert_from_js(verKey)?;
    let sk: bls::SignKey = convert_from_js(signKey)?;
    let pop = bls::ProofOfPossession::new(&vk, &sk)?;
    Ok(JsValue::from_serde(&pop).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsProofOfPossessionAsBytes(proofOfPossession: &JsValue) -> Result<Vec<u8>, JsValue> {
    let pop: bls::ProofOfPossession = convert_from_js(proofOfPossession)?;
    Ok(pop.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsProofOfPossessionFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let pop = bls::ProofOfPossession::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&pop).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsMultiSignature(signatures: Vec<JsValue>) -> Result<JsValue, JsValue> {
    let sigs: Vec<bls::Signature> = signatures.iter().map(|x| x.into_serde().unwrap()).collect();
    let ms = bls::MultiSignature::new(sigs.iter().collect::<Vec<_>>().as_slice())?;
    Ok(JsValue::from_serde(&ms).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsMultiSignatureAsBytes(multiSignature: &JsValue) -> Result<Vec<u8>, JsValue> {
    let ms: bls::MultiSignature = convert_from_js(multiSignature)?;
    Ok(ms.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsMultiSignatureFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let ms = bls::MultiSignature::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&ms).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerify(
    signature: &JsValue,
    message: &[u8],
    verKey: &JsValue,
    generator: &JsValue,
) -> Result<bool, JsValue> {
    let sig: bls::Signature = convert_from_js(signature)?;
    let vk: bls::VerKey = convert_from_js(verKey)?;
    let gen: bls::Generator = convert_from_js(generator)?;
    Ok(bls::Bls::verify(&sig, message, &vk, &gen)?)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerifyProofOfPossession(
    proofOfPossession: &JsValue,
    verKey: &JsValue,
    generator: &JsValue,
) -> Result<bool, JsValue> {
    let pop: bls::ProofOfPossession = convert_from_js(proofOfPossession)?;
    let vk: bls::VerKey = convert_from_js(verKey)?;
    let gen: bls::Generator = convert_from_js(generator)?;
    Ok(bls::Bls::verify_proof_of_posession(&pop, &vk, &gen)?)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsVerifyMultiSig(
    multiSig: &JsValue,
    message: &[u8],
    verKeys: Vec<JsValue>,
    generator: &JsValue,
) -> Result<bool, JsValue> {
    let ms: bls::MultiSignature = convert_from_js(multiSig)?;
    let vks: Vec<bls::VerKey> = verKeys
        .iter()
        .map(|x| {
            // TODO: Handle error case
            convert_from_js(x).unwrap()
        }).collect();
    let gen: bls::Generator = convert_from_js(generator)?;
    Ok(bls::Bls::verify_multi_sig(
        &ms,
        message,
        vks.iter().collect::<Vec<_>>().as_slice(),
        &gen,
    )?)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSignatureAsBytes(signature: &JsValue) -> Result<Vec<u8>, JsValue> {
    let sig: bls::Signature = convert_from_js(signature)?;
    Ok(sig.as_bytes().to_vec())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blsSignatureFromBytes(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let sig = bls::Signature::from_bytes(bytes)?;
    Ok(JsValue::from_serde(&sig).unwrap())
}
