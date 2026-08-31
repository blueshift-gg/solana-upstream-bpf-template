#![cfg_attr(target_arch = "bpf", no_std)]

#[cfg(target_arch = "bpf")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

pub fn log(msg: &str) {
    let sol_log_: unsafe extern "C" fn(message: *const u8, length: u64) = unsafe { core::mem::transmute(0x207559bd_usize) };
    unsafe { sol_log_(msg.as_ptr(), msg.len() as u64) }
}

#[unsafe(no_mangle)]
pub fn entrypoint(_input: *mut u8) -> u64 {
    log("Hello, world!");
    0
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{Mollusk, result::Check};
    use solana_instruction::Instruction;

    #[test]
    pub fn hello_world() {
        let mollusk = Mollusk::new(&[2u8;32].into(), "target/deploy/{{crate_name}}");
        mollusk.process_and_validate_instruction(&Instruction {
            program_id: [2u8;32].into(),
            accounts: vec![],
            data: vec![]
        }, &vec![], &[
            Check::success()
        ]);
    }
}