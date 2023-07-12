// use std::env;  
// use copy_to_output::copy_to_output;  
  
fn main() {  
    // Re-runs script if any files in assets are changed and copies them to the output directory
    //println!("cargo:rerun-if-changed=assets/*");  
    //copy_to_output("assets", &env::var("PROFILE").unwrap()).expect("Could not copy");  

    // Used to enable the high performance GPU on laptops with both integrated and discrete GPUs
    // println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");
    // println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");
}