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

unique_ptr<SleighProxy> new_sleigh_proxy(rust::Box<RustLoadImage> ld) {
    return std::make_unique<SleighProxy>(std::move(ld));
}

rust::Box<RustLoadImage>& SleighProxy::get_loader_mut() {
    return this->loader->load_image;
}

const rust::Box<RustLoadImage>& SleighProxy::get_loader() const {
    return this->loader->load_image;
}

void SleighProxy::set_loader(rust::Box<RustLoadImage> ld) {
    this->loader = make_unique<RustLoadImageProxy>(std::move(ld));
    this->translator.reset(this->loader.get(), &this->ctx);
    this->translator.initialize(storage);
}

rust::Box<RustAssemblyEmit>& SleighProxy::get_asm_emit_mut() {
    return this->asm_emit->assemblyEmit;
}

rust::Box<RustPcodeEmit>& SleighProxy::get_pcode_emit_mut() {
    return this->pcode_emit->rustPcodeEmit;
}

const rust::Box<RustAssemblyEmit>& SleighProxy::get_asm_emit() const {
    return this->asm_emit->assemblyEmit;
}

const rust::Box<RustPcodeEmit>& SleighProxy::get_pcode_emit() const {
    return this->pcode_emit->rustPcodeEmit;
}

void SleighProxy::set_asm_emit(rust::Box<RustAssemblyEmit> asm_emit) {
    this->asm_emit = std::make_unique<RustAssemblyEmitProxy>(std::move(asm_emit));
}

void SleighProxy::set_pcode_emit(rust::Box<RustPcodeEmit> pcode_emit) {
    this->pcode_emit = std::make_unique<RustPcodeEmitProxy>(std::move(pcode_emit));
}

int32_t SleighProxy::decode_asm_at(uint64_t start) {
    Address address(translator.getDefaultCodeSpace(), start);

    try {
        auto length = translator.printAssembly(*asm_emit, address);
        return length;
    } catch (BadDataError& e) {
        throw std::invalid_argument("bad data when decode asm: " + e.explain);
    } catch (UnimplError& e) {
        throw std::logic_error("pcode not implemented");
    }

    return 0;
}

void SleighProxy::decode_pcode_at(uint64_t start) {
    Address address(translator.getDefaultCodeSpace(), start);

    try {
        translator.oneInstruction(*pcode_emit, address);
    } catch (BadDataError& e) {
        throw std::invalid_argument("bad data when decode pcode: " + e.explain);
    } catch (UnimplError& e) {
        throw std::logic_error("pcode not implemented");
    }
}

/*
void SleighProxy::decode_with(RustAssemblyEmit& asm_emit, RustPcodeEmit& pcode_emit, uint64_t start, uint64_t inst_size) {

    auto assemblyEmit = RustAssemblyEmitProxy{asm_emit};
    auto pcodeEmit = RustPcodeEmitProxy{pcode_emit};

    Address address(translator.getDefaultCodeSpace(), start);

    auto length = 0;
    auto buf_used = 0;
    auto buf_size = loader->bufSize();
    auto total_insts = 0;

    while (buf_used < buf_size) {
        try {
            length = translator.printAssembly(assemblyEmit, address);
            translator.oneInstruction(pcodeEmit, address);
            address = address + length;
            buf_used = buf_used + length;
            total_insts ++;
            if (inst_size > 0 && total_insts >= inst_size) {
                break;
            }

        } catch (BadDataError &e) {
            throw std::invalid_argument("BadDataError");
        } catch (UnimplError &e) {
            throw std::logic_error("UnimplError");  // Pcode is not implemented for this constructor
        }

        //TODO: implement exception

    }

}
*/

// RustLoadImageProxy
void RustLoadImageProxy::loadFill(uint1 *ptr, int4 size, const Address &address) {
    Address addr = const_cast <Address& > (address);
    uint8_t* array = (uint8_t*)ptr;
    rust::Slice<::std::uint8_t> slice{array,(unsigned long)size};
    const auto addr_proxy = AddressProxy{addr};
    load_image->load_fill(slice, addr_proxy);
}

void RustLoadImageProxy::adjustVma(long adjust) {
    this->load_image->adjust_vma(adjust);
}

string RustLoadImageProxy::getArchType(void) const {
    return "plain";
}

int4 RustLoadImageProxy::bufSize() {
    return load_image->buf_size();
}

std::unique_ptr<RustLoadImageProxy> from_rust(rust::Box<RustLoadImage> load_image) {
    return std::make_unique<RustLoadImageProxy>(std::move(load_image));
}

void RustAssemblyEmitProxy::dump(const Address &address, const string &mnemonic, const string &body) {
    Address addr = const_cast <Address& > (address);
    const auto addr_proxy = AddressProxy{addr};
    assemblyEmit->dump(addr_proxy, mnemonic, body);
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

    rustPcodeEmit->dump(addr_proxy, opcodes, outvar_proxy, vars_vec);
}
