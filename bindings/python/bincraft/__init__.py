#
#  Copyright 2021 StarCrossTech
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
#     http://www.apache.org/licenses/LICENSE-2.0
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
from .bincraft import *

__all__ = [
    'Sleigh',
    'ARCH_6502',
    'ARCH_6805',
    'ARCH_6809',
    'ARCH_8048',
    'ARCH_8051',
    'ARCH_8085',
    'ARCH_68020',
    'ARCH_68030',
    'ARCH_68040',
    'ARCH_80251',
    'ARCH_80390',
    'ARCH_AARCH64',
    'ARCH_AARCH64BE',
    'ARCH_ARM4_BE',
    'ARCH_ARM4_LE',
    'ARCH_ARM4T_BE',
    'ARCH_ARM4T_LE',
    'ARCH_ARM5_BE',
    'ARCH_ARM5_LE',
    'ARCH_ARM5T_BE',
    'ARCH_ARM5T_LE',
    'ARCH_ARM6_BE',
    'ARCH_ARM6_LE',
    'ARCH_ARM7_BE',
    'ARCH_ARM7_LE',
    'ARCH_ARM8_BE',
    'ARCH_ARM8_LE',
    'ARCH_AVR8',
    'ARCH_AVR8E',
    'ARCH_AVR8EIND',
    'ARCH_AVR8XMEGA',
    'ARCH_AVR32A',
    'ARCH_COLDFIRE',
    'ARCH_CP1600',
    'ARCH_CR16B',
    'ARCH_CR16C',
    'ARCH_DALVIK',
    'ARCH_DATA_BE_64',
    'ARCH_DATA_LE_64',
    'ARCH_DSPIC30F',
    'ARCH_DSPIC33C',
    'ARCH_DSPIC33E',
    'ARCH_DSPIC33F',
    'ARCH_HC05',
    'ARCH_HC08',
    'ARCH_HCS08',
    'ARCH_HCS12',
    'ARCH_JVM',
    'ARCH_M8C',
    'ARCH_MCS96',
    'ARCH_MIPS32BE',
    'ARCH_MIPS32LE',
    'ARCH_MIPS32R6BE',
    'ARCH_MIPS32R6LE',
    'ARCH_MIPS64BE',
    'ARCH_MIPS64LE',
    'ARCH_MX51',
    'ARCH_PA_RISC32BE',
    'ARCH_PIC12C5XX',
    'ARCH_PIC16',
    'ARCH_PIC16C5X',
    'ARCH_PIC16F',
    'ARCH_PIC17C7XX',
    'ARCH_PIC18',
    'ARCH_PIC24E',
    'ARCH_PIC24F',
    'ARCH_PIC24H',
    'ARCH_PPC_32_4XX_BE',
    'ARCH_PPC_32_4XX_LE',
    'ARCH_PPC_32_BE',
    'ARCH_PPC_32_LE',
    'ARCH_PPC_32_QUICCIII_BE',
    'ARCH_PPC_32_QUICCIII_LE',
    'ARCH_PPC_64_BE',
    'ARCH_PPC_64_ISA_ALTIVEC_BE',
    'ARCH_PPC_64_ISA_ALTIVEC_LE',
    'ARCH_PPC_64_ISA_ALTIVEC_VLE_BE',
    'ARCH_PPC_64_ISA_BE',
    'ARCH_PPC_64_ISA_LE',
    'ARCH_PPC_64_ISA_VLE_BE',
    'ARCH_PPC_64_LE',
    'ARCH_RISCV',
    'ARCH_SH_1',
    'ARCH_SH_2',
    'ARCH_SH_2A',
    'ARCH_SPARCV9_32',
    'ARCH_SPARCV9_64',
    'ARCH_SUPERH4_BE',
    'ARCH_SUPERH4_LE',
    'ARCH_TI_MSP430',
    'ARCH_TI_MSP430X',
    'ARCH_TOY_BE_POSSTACK',
    'ARCH_TOY_BE',
    'ARCH_TOY_BUILDER_BE_ALIGN2',
    'ARCH_TOY_BUILDER_BE',
    'ARCH_TOY_BUILDER_LE_ALIGN2',
    'ARCH_TOY_BUILDER_LE',
    'ARCH_TOY_LE',
    'ARCH_TOY_WSZ_BE',
    'ARCH_TOY_WSZ_LE',
    'ARCH_TOY64_BE_HARVARD',
    'ARCH_TOY64_BE',
    'ARCH_TOY64_LE',
    'ARCH_TRICORE',
    'ARCH_V850',
    'ARCH_X86_64',
    'ARCH_X86',
    'ARCH_Z80',
    'ARCH_Z180',
    'MODE_16',
    'MODE_32',
    'MODE_64',
]

# arch
ARCH_6502 = "6502"
ARCH_6805 = "6805"
ARCH_6809 = "6809"
ARCH_8048 = "8048"
ARCH_8051 = "8051"
ARCH_8085 = "8085"
ARCH_68020 = "68020"
ARCH_68030 = "68030"
ARCH_68040 = "68040"
ARCH_80251 = "80251"
ARCH_80390 = "80390"
ARCH_AARCH64 = "AARCH64"
ARCH_AARCH64BE = "AARCH64BE"
ARCH_ARM4_BE = "ARM4_be"
ARCH_ARM4_LE = "ARM4_le"
ARCH_ARM4T_BE = "ARM4t_be"
ARCH_ARM4T_LE = "ARM4t_le"
ARCH_ARM5_BE = "ARM5_be"
ARCH_ARM5_LE = "ARM5_le"
ARCH_ARM5T_BE = "ARM5t_be"
ARCH_ARM5T_LE = "ARM5t_le"
ARCH_ARM6_BE = "ARM6_be"
ARCH_ARM6_LE = "ARM6_le"
ARCH_ARM7_BE = "ARM7_be"
ARCH_ARM7_LE = "ARM7_le"
ARCH_ARM8_BE = "ARM8_be"
ARCH_ARM8_LE = "ARM8_le"
ARCH_AVR8 = "avr8"
ARCH_AVR8E = "avr8e"
ARCH_AVR8EIND = "avr8eind"
ARCH_AVR8XMEGA = "avr8xmega"
ARCH_AVR32A = "avr32a"
ARCH_COLDFIRE = "coldfire"
ARCH_CP1600 = "CP1600"
ARCH_CR16B = "CR16B"
ARCH_CR16C = "CR16C"
ARCH_DALVIK = "Dalvik"
ARCH_DATA_BE_64 = "data-be-64"
ARCH_DATA_LE_64 = "data-le-64"
ARCH_DSPIC30F = "dsPIC30F"
ARCH_DSPIC33C = "dsPIC33C"
ARCH_DSPIC33E = "dsPIC33E"
ARCH_DSPIC33F = "dsPIC33F"
ARCH_HC05 = "HC05"
ARCH_HC08 = "HC08"
ARCH_HCS08 = "HCS08"
ARCH_HCS12 = "HCS12"
ARCH_JVM = "JVM"
ARCH_M8C = "m8c"
ARCH_MCS96 = "MCS96"
ARCH_MIPS32BE = "mips32be"
ARCH_MIPS32LE = "mips32le"
ARCH_MIPS32R6BE = "mips32R6be"
ARCH_MIPS32R6LE = "mips32R6le"
ARCH_MIPS64BE = "mips64be"
ARCH_MIPS64LE = "mips64le"
ARCH_MX51 = "mx51"
ARCH_PA_RISC32BE = "pa-risc32be"
ARCH_PIC12C5XX = "pic12c5xx"
ARCH_PIC16 = "pic16"
ARCH_PIC16C5X = "pic16c5x"
ARCH_PIC16F = "pic16f"
ARCH_PIC17C7XX = "pic17c7xx"
ARCH_PIC18 = "pic18"
ARCH_PIC24E = "PIC24E"
ARCH_PIC24F = "PIC24F"
ARCH_PIC24H = "PIC24H"
ARCH_PPC_32_4XX_BE = "ppc_32_4xx_be"
ARCH_PPC_32_4XX_LE = "ppc_32_4xx_le"
ARCH_PPC_32_BE = "ppc_32_be"
ARCH_PPC_32_LE = "ppc_32_le"
ARCH_PPC_32_QUICCIII_BE = "ppc_32_quicciii_be"
ARCH_PPC_32_QUICCIII_LE = "ppc_32_quicciii_le"
ARCH_PPC_64_BE = "ppc_64_be"
ARCH_PPC_64_ISA_ALTIVEC_BE = "ppc_64_isa_altivec_be"
ARCH_PPC_64_ISA_ALTIVEC_LE =  "ppc_64_isa_altivec_le"
ARCH_PPC_64_ISA_ALTIVEC_VLE_BE = "ppc_64_isa_altivec_vle_be"
ARCH_PPC_64_ISA_BE = "ppc_64_isa_be"
ARCH_PPC_64_ISA_LE = "ppc_64_isa_le"
ARCH_PPC_64_ISA_VLE_BE = "ppc_64_isa_vle_be"
ARCH_PPC_64_LE = "ppc_64_le"
ARCH_RISCV = "riscv"
ARCH_SH_1 = "sh-1"
ARCH_SH_2 = "sh-2"
ARCH_SH_2A = "sh-2a"
ARCH_SPARCV9_32 = "SparcV9_32"
ARCH_SPARCV9_64 = "SparcV9_64"
ARCH_SUPERH4_BE = "SuperH4_be"
ARCH_SUPERH4_LE = "SuperH4_le"
ARCH_TI_MSP430 = "TI_MSP430"
ARCH_TI_MSP430X = "TI_MSP430X"
ARCH_TOY_BE_POSSTACK = "toy_be_posStack"
ARCH_TOY_BE = "toy_be"
ARCH_TOY_BUILDER_BE_ALIGN2 = "toy_builder_be_align2"
ARCH_TOY_BUILDER_BE = "toy_builder_be"
ARCH_TOY_BUILDER_LE_ALIGN2 = "toy_builder_le_align2"
ARCH_TOY_BUILDER_LE = "toy_builder_le"
ARCH_TOY_LE = "toy_le"
ARCH_TOY_WSZ_BE = "toy_wsz_be"
ARCH_TOY_WSZ_LE = "toy_wsz_le"
ARCH_TOY64_BE_HARVARD = "toy64_be_harvard"
ARCH_TOY64_BE = "toy64_be"
ARCH_TOY64_LE = "toy64_le"
ARCH_TRICORE = "tricore"
ARCH_V850 = "V850"
ARCH_X86_64 = "x86-64"
ARCH_X86 = "x86"
ARCH_Z80 = "z80"
ARCH_Z180 = "z180"

# disasm mode
MODE_16 = 0
MODE_32 = 1
MODE_64 = 2

