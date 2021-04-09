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
#ifndef BRIDGE_PROXIES_TYPEOP_PROXY
#define BRIDGE_PROXIES_TYPEOP_PROXY

#include "typeop.hh"
#include "opbehavior_proxy.hh"
#include "cover_proxy.hh"
#include "sleighcraft/src/sleigh.rs.h"

class TypeOpProxy {
public:
    TypeOp& typeop;

    TypeOpProxy(TypeOp& typeop): typeop(typeop) {};
    TypeOpProxy(TypeOp* typeop): typeop(*typeop) {};

    const string& get_name(void) const;
    PcodeOpCode get_opcode(void) const;
    uint4 get_flags(void) const;
    unique_ptr<OpBehaviorProxy> get_behavior(void) const;
    bool mark_explicit_unsigned(PcodeOp *op,int4 slot) const; //PcodeOp_proxy -> cover_proxy
    uintb evaluate_unary(int4 sizeout,int4 sizein,uintb in1) const;
    uintb evaluate_binary(int4 sizeout,int4 sizein,uintb in1,uintb in2) const;
    uintb recover_input_binary(int4 slot,int4 sizeout,uintb out,int4 sizein,uintb in) const;
    uintb recover_input_unary(int4 sizeout,uintb out,int4 sizein) const;
    bool is_commutative(void) const;
    bool inherits_sign(void) const;
    //    virtual string get_operator_name(const PcodeOp *op) const;

    // TODO: complete the methods
};

#endif