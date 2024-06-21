use std::collections::BTreeMap;
use std::str::FromStr;
use bitcoin::{Amount, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, Txid, TxIn, TxOut, Witness};
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::Input;
mod tests {
    use bitcoin::psbt::Output;
    use super::*;
    #[test]
    fn test_construct_unsigned_psbt(){
        /// Creates the PSBT, in BIP174 parlance this is the 'Creator'.
        let unsigned_tx = Transaction{
            version:bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::blockdata::locktime::absolute::LockTime::ZERO,
            input: vec![
                TxIn{
                    previous_output: OutPoint{
                        txid: Txid::from_str("4240b666b1ef8e22ad4a1cd6d0a312e71fc3d5b6ceb4a70df099635b5b5f88de").unwrap(),
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence:Sequence::MAX,
                    witness: Witness::default(),
                }
            ],
            output: vec![
                TxOut {
                    value: Amount::from_sat(51123), // (0.00051123 BTC == 51123 sats)
                    script_pubkey: ScriptBuf::from_hex("00147e30ec724ceb5670917b3e6b0849938cb0661936").unwrap(),
                },
                TxOut {
                    value: Amount::from_sat(126298), // 0.00126298 BTC == 126298 sats
                    script_pubkey: ScriptBuf::from_hex("0014b0006f6789046ab2265fb640b1f7c5961bc99f21").unwrap(),
                },
            ],
        };
        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();
        let serialized_psbt = psbt.serialize();
        let hex_str = hex::encode(serialized_psbt);
        assert_eq!(
            "70736274ff0100710200000001de885f5b5b6399f00da7b4ceb6d5c31fe712a3d0d61c4aad228eefb166b640420100000000ffffffff02b3c70000000000001600147e30ec724ceb5670917b3e6b0849938cb06619365aed010000000000160014b0006f6789046ab2265fb640b1f7c5961bc99f210000000000000000",
            hex_str
        );

        // Updates the PSBT, in BIP174 parlance this is the 'Updater'.
        // add the witness_utxo field to the first input
        let mut input = Input { witness_utxo: Some(
            TxOut { value:  Amount::from_sat(178443), script_pubkey: ScriptBuf::from_hex("001473aef83067037c4b8bdf35cb6abdfb281234552d").unwrap() }
        ), ..Default::default() };

        // add the bip32_derivation field to the first input
        let public_key = PublicKey::from_str(
            "022f6b01d937c358ac3feb2e9f08eb540aae8be500dbc70d694c5590496ce5b3ae"
        ).unwrap();
        let fingerprint = Fingerprint::from_str("52744703").unwrap();
        let path = DerivationPath::from_str("m/84'/0'/0'/1/8").unwrap();
        let mut map = BTreeMap::new();
        map.insert(public_key.inner, (fingerprint, path));
        input.bip32_derivation = map;


        // update the PSBT with the new input
        psbt.inputs = vec![input];

        // add output to the PSBT
        let output = Output {
            redeem_script: None,
            witness_script: None,
            bip32_derivation: Default::default(),
            tap_internal_key: None,
            tap_tree: None,
            tap_key_origins: Default::default(),
            proprietary: Default::default(),
            unknown: Default::default(),
        };
        let output_public_key = "03e3ce504419553a98bbe5979b94e45c60aa990213bdd0c920e3f2e133dc4e7093";
        let master_finger_print = "52744703";
        let path = "m/84'/0'/0'/1/9";

        let public_key = PublicKey::from_str(output_public_key).unwrap();
        let fingerprint = Fingerprint::from_str(master_finger_print).unwrap();
        let path = DerivationPath::from_str(path).unwrap();
        let mut map = BTreeMap::new();
        map.insert(public_key.inner, (fingerprint, path));

        let mut output2 = Output {
            redeem_script: None,
            witness_script: None,
            bip32_derivation: Default::default(),
            tap_internal_key: None,
            tap_tree: None,
            tap_key_origins: Default::default(),
            proprietary: Default::default(),
            unknown: Default::default(),
        };

        output2.bip32_derivation = map;
        psbt.outputs = vec![output, output2];
        // serialize the updated PSBT
        let serialized_psbt = psbt.serialize();
        let hex_str = hex::encode(serialized_psbt);
        assert_eq!(
            "70736274ff0100710200000001de885f5b5b6399f00da7b4ceb6d5c31fe712a3d0d61c4aad228eefb166b640420100000000ffffffff02b3c70000000000001600147e30ec724ceb5670917b3e6b0849938cb06619365aed010000000000160014b0006f6789046ab2265fb640b1f7c5961bc99f21000000000001011f0bb902000000000016001473aef83067037c4b8bdf35cb6abdfb281234552d2206022f6b01d937c358ac3feb2e9f08eb540aae8be500dbc70d694c5590496ce5b3ae185274470354000080000000800000008001000000080000000000220203e3ce504419553a98bbe5979b94e45c60aa990213bdd0c920e3f2e133dc4e70931852744703540000800000008000000080010000000900000000",
            hex_str
        );
    }

}
