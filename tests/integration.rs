use std::env::set_var as set_env_var;
use api_snap::main;

#[test]
#[should_panic]
fn main_failed() {
    set_env_var("KUBECONFIG", "api-snap_invalid_path");
    main().unwrap()
}
