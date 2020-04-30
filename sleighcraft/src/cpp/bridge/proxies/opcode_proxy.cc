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
#include "opcode_proxy.hh"

int4 OpCodeProxy::count(void) const {
    return opcode.numInput();
}

unique_ptr <VarnodeProxy> OpCodeProxy::get_out(void) const {
    return unique_ptr<VarnodeProxy>(new VarnodeProxy{ opcode.getOut()});
}

unique_ptr <VarnodeProxy> OpCodeProxy::get_in(int4 slot) const {
    return unique_ptr<VarnodeProxy>(new VarnodeProxy{ opcode.getIn(slot)});
}

uintm OpCodeProxy::get_time(void) const {
    return opcode.getTime();
}

uint4 OpCodeProxy::get_eval_type(void) const {
    return opcode.getEvalType();
}

uint4 OpCodeProxy::get_halt_type(void) const {
    return opcode.getHaltType();
}

bool OpCodeProxy::is_dead(void) const {
    return opcode.isDead();
}

bool OpCodeProxy::is_assignment(void) const {
    return opcode.isAssignment();
}

bool OpCodeProxy::is_call(void) const {
    return opcode.isCall();
}

bool OpCodeProxy::is_call_without_spec(void) const {
    return opcode.isCallWithoutSpec();
}

bool OpCodeProxy::is_marker(void) const {
    return opcode.isMarker();
}

bool OpCodeProxy::is_indirect_creation(void) const {
    return opcode.isIndirectCreation();
}

bool OpCodeProxy::is_indirect_store(void) const {
    return opcode.isIndirectStore();
}

bool OpCodeProxy::not_printed(void) const {
    return opcode.notPrinted();
}

bool OpCodeProxy::is_bool_output(void) const {
    return opcode.isBoolOutput();
}

bool OpCodeProxy::is_branch(void) const {
    return opcode.isBranch();
}

bool OpCodeProxy::is_call_or_branch(void) const {
    return opcode.isCallOrBranch();
}

bool OpCodeProxy::is_flow_break(void) const {
    return opcode.isFlowBreak();
}

bool OpCodeProxy::is_boolean_flip(void) const {
    return opcode.isBooleanFlip();
}

bool OpCodeProxy::is_fallthru_true(void) const {
    return opcode.isFallthruTrue();
}

bool OpCodeProxy::is_code_ref(void) const {
    return opcode.isCodeRef();
}

bool OpCodeProxy::is_instruction_start(void) const {
    return opcode.isInstructionStart();
}

bool OpCodeProxy::is_block_start(void) const {
    return opcode.isBlockStart();
}

bool OpCodeProxy::is_modified(void) const {
    return opcode.isModified();
}

bool OpCodeProxy::is_mark(void) const {
    return opcode.isMark();
}

void OpCodeProxy::set_mark(void) const {
    return opcode.setMark();
}

bool OpCodeProxy::is_warning(void) const {
    return opcode.isWarning();
}

void OpCodeProxy::clear_mark(void) const {
    return opcode.clearMark();
}

bool OpCodeProxy::is_indirect_source(void) const {
    return opcode.isIndirectSource();
}

void OpCodeProxy::set_indirect_source(void) const {
    return opcode.setIndirectSource();
}

void OpCodeProxy::clear_indirect_source(void) const {
    return opcode.clearIndirectSource();
}

bool OpCodeProxy::is_ptr_flow(void) const {
    return opcode.isPtrFlow();
}

void OpCodeProxy::set_ptr_flow(void) const {
    return opcode.setPtrFlow();
}

bool OpCodeProxy::is_splitting(void) const {
    return opcode.isSplitting();
}

bool OpCodeProxy::does_special_propagation(void) const {
    return opcode.doesSpecialPropagation();
}

bool OpCodeProxy::does_special_printing(void) const {
    return opcode.doesSpecialPrinting();
}

bool OpCodeProxy::is_incidental_copy(void) const {
    return opcode.isIncidentalCopy();
}

bool OpCodeProxy::is_calculated_bool(void) const {
    return opcode.isCalculatedBool();
}

bool OpCodeProxy::is_cpool_transformed(void) const {
    return opcode.isCpoolTransformed();
}

bool OpCodeProxy::is_collapsible(void) const {
    return opcode.isCollapsible();
}

bool OpCodeProxy::uses_spacebase_ptr(void) const {
    return opcode.usesSpacebasePtr();
}

uintm OpCodeProxy::get_cse_hash(void) const {
    return opcode.getCseHash();
}

unique_ptr <TypeOpProxy> OpCodeProxy::get_opcode(void) const {
    return unique_ptr <TypeOpProxy> (new TypeOpProxy{opcode.getOpcode()});
}

PcodeOpCode OpCodeProxy::get_code(void) const {
    return (PcodeOpCode) opcode.code();
}

bool OpCodeProxy::is_commutative(void) const {
    return opcode.isCommutative();
}

unique_ptr <CoverProxy> OpCodeProxy::next_op(void) const {
    return unique_ptr <CoverProxy> (new CoverProxy{opcode.nextOp()});
}

unique_ptr <CoverProxy> OpCodeProxy::previous_op(void) const {
    return unique_ptr <CoverProxy> (new CoverProxy{opcode.previousOp()});
}

unique_ptr <CoverProxy> OpCodeProxy::get_start_op(void) const {
    return unique_ptr <CoverProxy> (new CoverProxy{opcode.target()});
}


//const unique_ptr <AddressProxy> OpCodeProxy::get_addr(void) const {
//    return unique_ptr<AddressProxy>(new AddressProxy{ opcode.getAddr()});
//}
