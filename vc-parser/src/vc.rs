use crate::assertions::compile::{Compile, Compiling};
use crate::assertions::parser::program;
use crate::wasm::{module_by_compiling, run_module};
use chrono::{DateTime, Utc};
use metered_wasmi::RuntimeValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum VCType {
    VerifiableCredential,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProofType {
    Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSoure {
    #[serde(rename = "dataProviderId")]
    pub data_provider_id: u32,
    #[serde(rename = "dataProvider")]
    pub data_provider: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuer {
    pub id: String,
    pub name: String,
    #[serde(rename = "enclaveId")]
    pub enclave_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialSubject {
    pub id: String,
    pub description: String,
    #[serde(rename = "type")]
    pub types: String,
    pub tag: Vec<String>,
    #[serde(rename = "dataSoure")]
    pub data_soure: Vec<DataSoure>,
    pub assertions: String,
    pub values: Vec<bool>,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proof {
    pub created: DateTime<Utc>,
    #[serde(rename = "type")]
    pub proof_type: ProofType,
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    #[serde(rename = "proofValue")]
    pub proof_value: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VC {
    pub id: String,
    pub subject: String,
    #[serde(rename = "type")]
    pub types: Vec<VCType>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,
    pub issuer: Issuer,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: DateTime<Utc>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: DateTime<Utc>,
    pub proof: Proof,
}

impl VC {
    pub fn assertion_run(&self) -> Result<RuntimeValue, metered_wasmi::Error> {
        let s = &self.credential_subject.assertions.to_owned();
        let exp = program(s.as_str());
        let r = exp.unwrap();
        println!("{:?}", r);

        let ins = r.1.compile(Compiling::default());
        println!("{:?}", ins.instructions);
        println!("{:?}", ins.locals);

        let module = module_by_compiling(ins);
        run_module(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_simple_success() {
        let data = r#"
        {
            "@context": [
                "https://www.w3.org/2018/credentials/v1", 
                "https://w3id.org/security/suites/ed25519-2020/v1"
            ], 
            "id": "http://litentry.com/2022/credentials/twitter/follower", 
            "type": [
                "VerifiableCredential"
            ], 
            "issuer": {
                "id": "did:litentry:7f8ca8982f6cc6e8ea087bd9457ab8024bd2", 
                "enclaveId": "enclave id or registered hash", 
                "name": "Litentry TEE Worker"
            }, 
            "subject": "did:litentry:owner's Litentry address", 
            "issuanceDate": "2022-09-01T12:01:20Z", 
            "expirationDate": "2022-09-14T12:01:20Z", 
            "credentialSubject": {
                "id": "did:litentry:97c30de767f084ce3080168ee293053ba33b235d71", 
                "description": "1000-2000 Twitter followers", 
                "type": "TwitterFollower", 
                "tag": [
                    "Twitter", 
                    "IDHub"
                ], 
                "dataSoure": [
                    {
                        "dataProvider": "https://litentry.com/endpoint/graphql", 
                        "dataProviderId": 1
                    }
                ], 
                "assertions": "return 1+20", 
                "values": [
                    true
                ], 
                "endpoint": "https://litentry.com/parachain/extrinsic"
            }, 
            "proof": {
                "created": "2022-09-01T12:01:20Z", 
                "type": "Ed25519Signature2020", 
                "proofPurpose": "assertionMethod", 
                "proofValue": "f66944a454904a19f30a2b045ea80534547ffb522cdf2f8d9b949c76331d9d2c8359c4668b0775362d697985f52645d2479fbde0792dacdad9fdea09c4120c0d", 
                "verificationMethod": "did:litentry:issuer's Litentry pubkey"
            }
        }
        "#;

        let vc: VC = serde_json::from_str::<VC>(data).unwrap();
        println!("{:?}", vc);
        assert_eq!(vc.subject, "did:litentry:owner's Litentry address");
        assert_eq!(vc.proof.proof_purpose, "assertionMethod");
        assert_eq!(
            vc.credential_subject.id,
            "did:litentry:97c30de767f084ce3080168ee293053ba33b235d71"
        );

        let result = vc.assertion_run();
        match result {
            Ok(RuntimeValue::I32(rv)) => assert_eq!(rv, 21),
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn eval_simple_crediential() {
        let data = r#"
        {
            "id": "did:litentry:97c30de767f084ce3080168ee293053ba33b235d71", 
            "description": "1000-2000 Twitter followers", 
            "type": "TwitterFollower", 
            "tag": [
                "Twitter", 
                "IDHub"
            ], 
            "dataSoure": [
                {
                    "dataProvider": "https://litentry.com/endpoint/graphql", 
                    "dataProviderId": 1
                }
            ], 
            "assertions": [
                {
                    "and": [
                        {
                            "src": "$follower", 
                            "op": ">=", 
                            "dsc": "1000", 
                            "priority": 1, 
                            "dataProviderId": 1
                        }, 
                        {
                            "src": "$follower", 
                            "op": "<", 
                            "dsc": "2000", 
                            "priority": 2, 
                            "dataProviderId": 1
                        }
                    ]
                }
            ], 
            "values": [
                true
            ], 
            "endpoint": "https://litentry.com/parachain/extrinsic"
        }
        "#;
    }
}
