use vc_parser::vc::VC;

fn main() {
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
    println!("{}", vc.subject);
    println!("{}", vc.credential_subject.id);
    //println!("{:?}", vc);

    let re = vc_parser::run_vc(data, None);
    println!("{:?}", re);
}
