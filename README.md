

## 0x1、input 中的sighash_type为什么设值？
```rust
    /// The sighash type to be used for this input. Signatures for this input
    /// must use the sighash type.
    pub sighash_type: Option<PsbtSighashType>,
```
在psbt中，签名有多种类型，比如sighash_all、sighash_single、sighash_none等。不同的签名类型的作用也不同。所以必须明确设置sighash_type，以便于后续的签名操作。
那么为什么在本例中sighash_type为none呢？设置为None不会影响到后续的流程吗？首先，不会影响后续的签名流程，这得益于rust-bitcoin的实现。在使用rust-bitcoin库进行签名时。
会进行一个验证，如果sighash_type为None，那么会自动设置为sighash_all。所以在这里设置为None也是可以的。

```rust
impl Input {
    /// Obtains the [`EcdsaSighashType`] for this input if one is specified. If no sighash type is
    /// specified, returns [`EcdsaSighashType::All`].
    ///
    /// # Errors
    ///
    /// If the `sighash_type` field is set to a non-standard ECDSA sighash value.
    pub fn ecdsa_hash_ty(&self) -> Result<EcdsaSighashType, NonStandardSighashTypeError> {
        self.sighash_type
            .map(|sighash_type| sighash_type.ecdsa_hash_ty())
            .unwrap_or(Ok(EcdsaSighashType::All))
    }
```

但正常情况下，应该由updater来显示声明sighash_type值为SIGHASH_ALL，以便于后续的签名操作。

## 0x2、为什么psbt中的output为空呢？
```rust
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
```
按照bip174上定义的角色的职责，updater可以有修改input和output的权限，所以在这里updater去修改psbt.output的信息是合情合理的。
按照协议所说，updater可以在了解相关信息的前提下去修改input和output的内容。换句话说，如果updater不知道output的相关信息，也不做强制的要求。所以这里的psbt中的output中只修改了一个output，另一个保持为空也没有任何问题。
退一万步讲，如果updater压根不知道ouput相关信息，全都为空也是可以的，有更好，没有也不影响。
```bazaar
Updater must only accept a PSBT. The Updater adds information to the PSBT that it has access to. If it has the UTXO for an input, it should add it to the PSBT. The Updater should also add redeemScripts, witnessScripts, and BIP 32 derivation paths to the input and output data if it knows them.

A single entity is likely to be both a Creator and Updater. 
```



