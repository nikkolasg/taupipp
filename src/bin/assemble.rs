use paired::bls12_381::Bls12;
use taupipp::fetch;
use taupipp::powers;

fn main() {
    println!("reading zcash taus!");
    let zcash_params = powers::TauParams::new(1 << 21);
    let zcash_uri = fetch::URI::File("zcash_poweroftau.md".to_string());
    let _zcash_acc = fetch::read_powers_from::<Bls12>(zcash_params, zcash_uri);
    // last contribution for Filecoin's power of tau:
    // https://github.com/arielgabizon/perpetualpowersoftau/tree/master/0018_GolemFactory_response
    println!("reading filecoin taus!");
    let fil_params = powers::TauParams::new(1 << 27);
    let filecoin_uri =
        fetch::URI::HTTP("https://trusted-setup.filecoin.io/phase1/challenge_19".to_string());
    let _filecoin_acc = fetch::read_powers_from::<Bls12>(fil_params, filecoin_uri);
}
