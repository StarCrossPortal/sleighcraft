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
#include "opbehavior_proxy.hh"

PcodeOpCode OpBehaviorProxy::get_opcode(void) const {
    return (PcodeOpCode) opbehavior.getOpcode();
}

bool OpBehaviorProxy::is_special(void) const {
    return opbehavior.isSpecial();
}

bool OpBehaviorProxy::is_unary(void) const {
    return opbehavior.isUnary();
}

uintb OpBehaviorProxy::evaluate_unary(int4 sizeout, int4 sizein, uintb in1) const {
    return opbehavior.evaluateUnary(sizeout, sizein, in1);
}

uintb OpBehaviorProxy::evaluate_binary(int4 sizeout, int4 sizein, uintb in1, uintb in2) const {
    return opbehavior.evaluateBinary(sizeout, sizein, in1, in2);
}

uintb OpBehaviorProxy::recover_input_binary(int4 slot, int4 sizeout, uintb out, int4 sizein, uintb in) const {
    return opbehavior.recoverInputBinary(slot, sizeout, out, sizein, in);
}

uintb OpBehaviorProxy::recover_input_unary(int4 sizeout, uintb out, int4 sizein) const {
    return opbehavior.recoverInputUnary(sizeout, out, sizein);
}
