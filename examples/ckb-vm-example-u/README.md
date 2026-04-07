
# Ckb-vm example using unified workspace
Inside `script_example` we have a dumped script traces that we need to generate a proof for.

### Command to run the example as ordinary jolt project
```bash
RUST_LOG=info cargo run -p ckb-vm-example-u --release   

RUST_LOG=info RUST_BACKTRACE=full cargo run -p ckb-vm-example-u --release  

JOLT_BACKTRACE=1 cargo run --release -p ckb-vm-example-u   
```

2. Execute test
### Both tests          
```bash
cd examples/ckb-vm-example-u && cargo test -- --nocapture                                                                                                                               
```                                                                                                                                                                    
                                                                                                                                                                                            
### Just no-asm                                                                                                                                                                             
```bash
cargo test -p ckb-vm-example-u debug_ckb_vm_no_asm -- --nocapture      
```                                                                                                                   
                                                                                                                                                                                        
# Just asm                                                                                                                                                                              
```bash
cargo test -p ckb-vm-example-u debug_ckb_vm_asm -- --nocapture  
```