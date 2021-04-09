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
#ifndef BRIDGE_PROXIES_OPCODE_PROXY
#define BRIDGE_PROXIES_OPCODE_PROXY

#include "op.hh"
#include "varnode_proxy.hh"
#include "address_proxy.hh"
#include "typeop_proxy.hh"
class OpCodeProxy {
public:
    PcodeOp& opcode;
    OpCodeProxy(PcodeOp& opcode): opcode(opcode) {};
    OpCodeProxy(PcodeOp* opcode): opcode(*opcode) {};

    int4 count(void) const;
    unique_ptr<VarnodeProxy> get_out(void) const;
    unique_ptr<VarnodeProxy> get_in(int4 slot) const;
    // const unique_ptr<AddressProxy> get_addr(void) const;
    uintm get_time(void) const ;
    uint4 get_eval_type(void) const;
    uint4 get_halt_type(void) const;

    bool is_dead(void) const;
    bool is_assignment(void) const;
    bool is_call(void) const;
    bool is_call_without_spec(void) const;
    bool is_marker(void) const;
    bool is_indirect_creation(void) const;
    bool is_indirect_store(void) const;
    bool not_printed(void) const;
    bool is_bool_output(void) const;
    bool is_branch(void) const;
    bool is_call_or_branch(void) const;
    bool is_flow_break(void) const;
    bool is_boolean_flip(void) const;
    bool is_fallthru_true(void) const;
    bool is_code_ref(void) const;
    bool is_instruction_start(void) const;
    bool is_block_start(void) const;
    bool is_modified(void) const;
    bool is_mark(void) const;
    void set_mark(void) const;
    bool is_warning(void) const;
    void clear_mark(void) const;
    bool is_indirect_source(void) const;
    void set_indirect_source(void) const;
    void clear_indirect_source(void) const;
    bool is_ptr_flow(void) const;
    void set_ptr_flow(void) const;
    bool is_splitting(void) const;
    bool does_special_propagation(void) const;
    bool does_special_printing(void) const;
    bool is_incidental_copy(void) const;
    bool is_calculated_bool(void) const;
    bool is_cpool_transformed(void) const;
    bool is_collapsible(void) const;
    bool uses_spacebase_ptr(void) const;

    uintm get_cse_hash(void) const;
//    bool is_cse_match(const CoverProxy &op) const;
//    bool is_moveable(const PcodeOp *point) const;
    unique_ptr <TypeOpProxy> get_opcode(void) const;
    PcodeOpCode get_code(void) const;
    bool is_commutative(void) const;
    unique_ptr <CoverProxy> next_op(void) const;
    unique_ptr <CoverProxy> previous_op(void) const;
    unique_ptr <CoverProxy> get_start_op(void) const;

    // TODO: complete the methods
};

#endif