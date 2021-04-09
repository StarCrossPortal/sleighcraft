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
    RustLoadImage& load_image;
//    string(load_image->get_filename())
    RustLoadImageProxy(RustLoadImage &load_image): load_image(load_image), LoadImage("nofile") {}

    RustLoadImageProxy(RustLoadImage *load_image): load_image(*load_image), LoadImage("nofile") {}

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
    RustAssemblyEmit& assemblyEmit;
    RustAssemblyEmitProxy(RustAssemblyEmit& assemblyEmit): assemblyEmit(assemblyEmit){}
    RustAssemblyEmitProxy(RustAssemblyEmit* assemblyEmit): assemblyEmit(*assemblyEmit){}

    virtual void dump(const Address &address, const string &mnemonic, const string &body);


};

class RustPcodeEmitProxy: public PcodeEmit {
public:

    RustPcodeEmit& rustPcodeEmit;
    RustPcodeEmitProxy(RustPcodeEmit& rustPcodeEmit): rustPcodeEmit(rustPcodeEmit){}
    RustPcodeEmitProxy(RustPcodeEmit* rustPcodeEmit): rustPcodeEmit(*rustPcodeEmit){}

    virtual void dump(const Address &addr,OpCode opc,VarnodeData *outvar,VarnodeData *vars,int4 isize);

};

class SleighProxy {
public:
    SleighProxy(RustLoadImage &ld): loader(ld), translator(&loader, &this->ctx) {}

    void setSpecFromPath(const rust::Str path);
    void set_spec(const rust::Str spec_content);
    void decode_with(RustAssemblyEmit& asm_emit, RustPcodeEmit& pcode_emit, uint64_t start);

private:
    RustLoadImageProxy loader;
    Sleigh translator;
    ContextInternal ctx;
    DocumentStorage storage;
};

//unique_ptr<SleighProxy> proxy_from_spec(rust::Str path, RustLoadImage &ld, RustAssemblyEmit &asm_emit, RustPcodeEmit &rustPcodeEmit);
//unique_ptr<SleighProxy> proxy_from_spec_path(rust::Str spec_content, RustLoadImage &ld, RustAssemblyEmit &asm_emit, RustPcodeEmit &rustPcodeEmit);
std::unique_ptr<RustLoadImageProxy> from_rust(RustLoadImage& load_image);
unique_ptr<SleighProxy> new_sleigh_proxy(RustLoadImage &ld);

#endif