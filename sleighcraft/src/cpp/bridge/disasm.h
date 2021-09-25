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
#ifndef BRIDGE_DISASM_H
#define BRIDGE_DISASM_H

#include "sleigh.hh"
#include "emulate.hh"
#include "loadimage.hh"
#include <memory>
#include "rust/cxx.h"
#include "sleighcraft/src/sleigh.rs.h"

class RustLoadImage;
class RustAssemblyEmit;
class RustPcodeEmit;
class OpCodeProxy;
class VarnodeDataProxy;
class RustLoadImageProxy: public LoadImage {
public:
    rust::Box<RustLoadImage> load_image;
    RustLoadImageProxy(rust::Box<RustLoadImage> load_image): load_image(std::move(load_image)), LoadImage("nofile") {}

    virtual void loadFill(uint1 *ptr, int4 size, const Address &address);

    virtual string getArchType(void) const;
    virtual void adjustVma(long adjust);
    int4 bufSize();

};

struct InstructionProxy {
    string space;
    uint64_t offset;
    string mnemonic;
    string body;
//    Instruction& instruction;
//    InstructionProxy(Instruction& instruction): instruction(instruction) {}

    const string& get_space() const {
        return space;
    }

    uint64_t get_offset() const {
        return offset;
    }

    const string& get_mnemonic() const {
        return mnemonic;
    }

    const string& get_body() const {
        return body;
    }
};


class RustAssemblyEmitProxy: public AssemblyEmit {
public:
    rust::Box<RustAssemblyEmit> assemblyEmit;
    RustAssemblyEmitProxy(rust::Box<RustAssemblyEmit> assemblyEmit): assemblyEmit(std::move(assemblyEmit)){}

    virtual void dump(const Address &address, const string &mnemonic, const string &body);


};

class RustPcodeEmitProxy: public PcodeEmit {
public:

    rust::Box<RustPcodeEmit> rustPcodeEmit;
    RustPcodeEmitProxy(rust::Box<RustPcodeEmit> rustPcodeEmit): rustPcodeEmit(std::move(rustPcodeEmit)){}

    virtual void dump(const Address &addr,OpCode opc,VarnodeData *outvar,VarnodeData *vars,int4 isize);

};

class SleighProxy {
public:
    SleighProxy(rust::Box<RustLoadImage> ld): loader(std::make_unique<RustLoadImageProxy>(std::move(ld))), translator(loader.get(), &this->ctx) {}

    void set_asm_emit(rust::Box<RustAssemblyEmit> asm_emit);
    void set_pcode_emit(rust::Box<RustPcodeEmit> pcode_emit);
    rust::Box<RustAssemblyEmit>& get_asm_emit_mut();
    rust::Box<RustPcodeEmit>& get_pcode_emit_mut();
    const rust::Box<RustAssemblyEmit>& get_asm_emit() const;
    const rust::Box<RustPcodeEmit>& get_pcode_emit() const;

    void setSpecFromPath(const rust::Str path, int mode);
    void set_spec(const rust::Str spec_content, int mode);
    int32_t decode_asm_at(uint64_t start);
    void decode_pcode_at(uint64_t start);
    //void decode_with(RustAssemblyEmit& asm_emit, RustPcodeEmit& pcode_emit, uint64_t start, uint64_t inst_size);
    void set_loader(rust::Box<RustLoadImage> ld);
    rust::Box<RustLoadImage>& get_loader_mut();
    const rust::Box<RustLoadImage>& get_loader() const;

private:
    std::unique_ptr<RustLoadImageProxy> loader;
    std::unique_ptr<RustAssemblyEmitProxy> asm_emit;
    std::unique_ptr<RustPcodeEmitProxy> pcode_emit;

    Sleigh translator;
    ContextInternal ctx;
    DocumentStorage storage;
};

//unique_ptr<SleighProxy> proxy_from_spec(rust::Str path, RustLoadImage &ld, RustAssemblyEmit &asm_emit, RustPcodeEmit &rustPcodeEmit);
//unique_ptr<SleighProxy> proxy_from_spec_path(rust::Str spec_content, RustLoadImage &ld, RustAssemblyEmit &asm_emit, RustPcodeEmit &rustPcodeEmit);
std::unique_ptr<RustLoadImageProxy> from_rust(rust::Box<RustLoadImage> load_image);
unique_ptr<SleighProxy> new_sleigh_proxy(rust::Box<RustLoadImage> ld);

#endif