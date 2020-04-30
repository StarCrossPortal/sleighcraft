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
#include "typeop_proxy.hh"

const string& TypeOpProxy::get_name(void) const {
    return typeop.getName();
}

PcodeOpCode TypeOpProxy::get_opcode(void) const {
    return (PcodeOpCode) typeop.getOpcode();
}

uint4 TypeOpProxy::get_flags(void) const {
    return typeop.getFlags();
}

unique_ptr <OpBehaviorProxy> TypeOpProxy::get_behavior(void) const {
    return unique_ptr<OpBehaviorProxy> (new OpBehaviorProxy{ typeop.getBehavior()});
}

bool TypeOpProxy::mark_explicit_unsigned(PcodeOp *op, int4 slot) const {
    return typeop.markExplicitUnsigned(op, slot);
}

uintb TypeOpProxy::evaluate_unary(int4 sizeout, int4 sizein, uintb in1) const {
    return typeop.evaluateUnary(sizeout, sizein, in1);
}

uintb TypeOpProxy::evaluate_binary(int4 sizeout, int4 sizein, uintb in1, uintb in2) const {
    return typeop.evaluateBinary(sizeout, sizein, in1, in2);
}

uintb TypeOpProxy::recover_input_binary(int4 slot, int4 sizeout, uintb out, int4 sizein, uintb in) const {
    return typeop.recoverInputBinary(slot, sizeout, out, sizein, in);
}

uintb TypeOpProxy::recover_input_unary(int4 sizeout, uintb out, int4 sizein) const {
    return typeop.recoverInputUnary(sizeout, out, sizein);
}

bool TypeOpProxy::is_commutative(void) const {
    return typeop.isCommutative();
}

bool TypeOpProxy::inherits_sign(void) const {
    return typeop.inheritsSign();
}
//
//string TypeOpProxy::get_operator_name(const PcodeOp *op) const {
//    return typeop.getOperatorName(op);
//}

