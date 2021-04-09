/**
 *  Copyright 2021 StarCrossTech
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#ifndef BRIDGE_PROXIES_H
#define BRIDGE_PROXIES_H

// Other proxies might include the `sleigh.rs.h` where it includes again this file, causing a cyclic import.
// So, we need forward declaration here to avoid problems.
// When do we need to add forward declarations here? When you have a compliation error saying there's a type name
// in Rust but "is not named a type", add your declaration here to address that it IS a type.

enum class SpaceType: std::uint8_t;
enum class PcodeOpCode: std::uint8_t;
class AddressProxy;
class AddrSpaceProxy;
class CoverProxy;
class OpCodeProxy;
class RustLoadImageProxy;
class VarnodeProxy;
class VarnodeDataProxy;
class OpBehaviorProxy;
class TypeOpProxy;
class VariableProxy;
class InstructionProxy;
class SleighProxy;
class RustLoadImage;
class RustAssemblyEmit;
class RustPcodeEmit;
#include "proxies/address_proxy.hh"
#include "proxies/addrspace_proxy.hh"
#include "proxies/loadimage_proxy.hh"
#include "proxies/opcode_proxy.hh"
#include "proxies/varnode_proxy.hh"
#include "proxies/varnodedata_proxy.hh"
#include "proxies/cover_proxy.hh"
#include "proxies/block_proxy.hh"
#include "proxies/funcdata_proxy.hh"
#include "proxies/typeop_proxy.hh"
//#include "proxies/opcodes_proxy.hh"
#include "proxies/opbehavior_proxy.hh"
#include "proxies/variable_proxy.hh"
#include "proxies/varnodedata_proxy.hh"
#include "disasm.h"


#endif