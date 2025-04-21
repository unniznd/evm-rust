mod evm;
mod opcode;

use crate::evm::EVM;

fn main() {
    let hex_input = "6009600a10";

    match EVM::new(hex_input) {
        Ok(mut evm) => match evm.execute() {
            Ok(()) => println!("Execution successful. Stack: {:?}", evm.get_stack()),
            Err(e) => println!("Execution failed: {}", e),
        },
        Err(e) => println!("Failed to initialize EVM: {}", e),
    }
}
