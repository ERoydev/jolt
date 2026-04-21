use serde::{Deserialize, Serialize};

use crate::{declare_riscv_instr, emulator::cpu::Cpu};

use super::{format::format_assert_align::FormatAssert, RISCVInstruction, RISCVTrace};

declare_riscv_instr!(
    name = VirtualAssertWordAlignment,
    mask = 0,
    match = 0,
    format = FormatAssert,
    ram = ()
);

// ====== Original code
// impl VirtualAssertWordAlignment {
//     fn exec(
//         &self,
//         cpu: &mut Cpu,
//         _: &mut <VirtualAssertWordAlignment as RISCVInstruction>::RAMAccess,
//     ) {
//         let address = cpu.x[self.operands.rs1 as usize] + self.operands.imm;
//         assert!(
//             address & 3 == 0,
//             "RAM access (LW or LWU) is not word aligned: addr={address:x} pc={:x}",
//             cpu.pc
//         );
//     }
// }

impl VirtualAssertWordAlignment {
    fn exec(
        &self,
        cpu: &mut Cpu,
        _: &mut <VirtualAssertWordAlignment as RISCVInstruction>::RAMAccess,
    ) {
        let rs1 = self.operands.rs1;
        let imm = self.operands.imm;
        let base = cpu.x[rs1 as usize];
        let address = base.wrapping_add(imm);
        let misalign = (address as u64) & 3;

        assert!(
            misalign == 0,
            "unaligned LW: orig LW PC=0x{:x} cpu.pc=0x{:x} rs1=x{} base=0x{:x} imm={} (0x{:x}) address=0x{:x} misalignment={} xlen={:?}",
            self.address,
            cpu.pc,
            rs1,
            base as u64,
            imm,
            imm as u64,
            address as u64,
            misalign,
            cpu.xlen,
        );
    }
}

impl RISCVTrace for VirtualAssertWordAlignment {}
