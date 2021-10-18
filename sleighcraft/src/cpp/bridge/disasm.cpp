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
#include "disasm.h"
#include <sstream>
#include <iostream>
#include "proxies/address_proxy.hh"

void SleighProxy::set_spec(const rust::Str spec_content, int mode) {
    stringstream ss;
    ss << spec_content;

    Element *root = storage.parseDocument(ss)->getRoot();
    storage.registerTag(root);

    translator.initialize(storage);

    if (mode != 0) {
        this->ctx.setVariableDefault("addrsize",mode);
        this->ctx.setVariableDefault("opsize",mode);
    }

}

void SleighProxy::setSpecFromPath(const rust::Str path,int mode) {
    string cxxpath = string(path);
    Element *root = storage.openDocument(cxxpath)->getRoot();
    storage.registerTag(root);

    translator.initialize(storage);

    this->ctx.setVariableDefault("addrsize",mode); // Address size is 32-bit
    this->ctx.setVariableDefault("opsize",mode); // Operand size is 32-bit
}

unique_ptr<SleighProxy> new_sleigh_proxy(RustLoadImage &ld) {
    unique_ptr<SleighProxy> proxy(new SleighProxy(ld));
    return proxy;
}

int SleighProxy::print_assembly(RustAssemblyEmit& asm_emit, uint64_t start) {
    auto assemblyEmit = RustAssemblyEmitProxy{asm_emit};
    Address address(translator.getDefaultCodeSpace(), start);

    try {
        auto length = translator.printAssembly(assemblyEmit, address);
        return length;
    } catch (BadDataError &e) {
        throw std::invalid_argument(e.explain);
    } catch (UnimplError &e) {
        throw std::logic_error(e.explain);  // Pcode is not implemented for this constructor
    }

}

int SleighProxy::one_instruction(RustPcodeEmit& pcode_emit, uint64_t start) {
    auto pcodeEmit = RustPcodeEmitProxy{pcode_emit};
    Address address(translator.getDefaultCodeSpace(), start);

    try {
        auto length = translator.oneInstruction(pcodeEmit, address);
        return length;
    } catch (BadDataError &e) {
        throw std::invalid_argument(e.explain);
    } catch (UnimplError &e) {
        throw std::logic_error(e.explain);  // Pcode is not implemented for this constructor
    }

}

// RustLoadImageProxy
void RustLoadImageProxy::loadFill(uint1 *ptr, int4 size, const Address &address) {
    Address addr = const_cast <Address& > (address);
    uint8_t* array = (uint8_t*)ptr;
    rust::Slice<::std::uint8_t> slice{array,(unsigned long)size};
    const auto addr_proxy = AddressProxy{addr};
    load_image.load_fill(slice, addr_proxy);
}

void RustLoadImageProxy::adjustVma(long adjust) {
    this->load_image.adjust_vma(adjust);
}

string RustLoadImageProxy::getArchType(void) const {
    return "plain";
}

int4 RustLoadImageProxy::bufSize() {
    return load_image.buf_size();
}

std::unique_ptr<RustLoadImageProxy> from_rust(RustLoadImage& load_image) {
    return unique_ptr<RustLoadImageProxy>(new RustLoadImageProxy(load_image));
}

void RustAssemblyEmitProxy::dump(const Address &address, const string &mnemonic, const string &body) {
    Address addr = const_cast <Address& > (address);
    const auto addr_proxy = AddressProxy{addr};
    assemblyEmit.dump(addr_proxy, mnemonic, body);
}


void RustPcodeEmitProxy::dump(const Address &addr, OpCode opc, VarnodeData *outvar, VarnodeData *vars, int4 isize) {
    Address addrs = const_cast <Address& > (addr);
    const auto addr_proxy = AddressProxy{addrs};
    auto opcodes = (PcodeOpCode) opc;
    auto outvar_proxy = VarnodeDataProxy{outvar};

    vector<VarnodeDataProxy> vars_vec;

    for (auto i = 0; i < isize; ++i) {
        vars_vec.push_back(VarnodeDataProxy{&vars[i]});
    }

    rustPcodeEmit.dump(addr_proxy, opcodes, outvar_proxy, vars_vec);
}
