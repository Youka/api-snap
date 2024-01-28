use std::env::set_var as set_env_var;
use api_snap::main;

#[test]
#[should_panic]
fn main_panic() {
    set_env_var("KUBECONFIG", "invalid_path");

    main(None).unwrap()
}
