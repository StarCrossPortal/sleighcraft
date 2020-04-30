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
#pragma once
#include "opbehavior.hh"
#include "sleighcraft/src/sleigh.rs.h"

class OpBehaviorProxy {
public:
    OpBehavior& opbehavior;

    OpBehaviorProxy(OpBehavior& opbehavior): opbehavior(opbehavior) {};
    OpBehaviorProxy(OpBehavior* opbehavior): opbehavior(*opbehavior) {};

    PcodeOpCode get_opcode(void) const;
    bool is_special(void) const;
    bool is_unary(void) const;
    virtual uintb evaluate_unary(int4 sizeout,int4 sizein,uintb in1) const;
    virtual uintb evaluate_binary(int4 sizeout,int4 sizein,uintb in1,uintb in2) const;
    virtual uintb recover_input_binary(int4 slot,int4 sizeout,uintb out,int4 sizein,uintb in) const;
    virtual uintb recover_input_unary(int4 sizeout,uintb out,int4 sizein) const;

    // TODO: complete the methods
};