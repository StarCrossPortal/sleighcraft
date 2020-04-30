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
#include "space.hh"
#include "sleighcraft/src/sleigh.rs.h"

class AddrSpaceProxy {
public:
    AddrSpace& space;

    AddrSpaceProxy(AddrSpace* space): space(*space) {}
    AddrSpaceProxy(AddrSpace& space): space(space) {}

    const string& get_name(void) const;
    SpaceType get_type(void) const;
    int4 get_delay(void) const;
    int4 get_deadcode_delay(void) const;
    int4 get_index(void) const;
    uint4 get_wordsize(void) const;
    uint4 get_addrsize(void) const;
    uintb get_highest(void) const;
    uintb get_pointer_lower_bound(void) const;
    uintb get_pointer_upper_bound(void) const;
    int4 get_minimum_ptr_size(void) const;
    uintb wrap_offset(uintb off) const;
    int8_t get_shortcut(void) const;
    bool is_heritaged(void) const;
    bool does_deadcode(void) const;
    bool has_physical(void) const;
    bool is_big_endian(void) const;
    bool is_reverse_justified(void) const;
    bool is_overlay(void) const;
    bool is_overlay_base(void) const;
    bool is_other_space(void) const;
    bool is_truncated(void) const;
    bool has_near_pointers(void) const;
    void print_offset(ostream &s,uintb offset) const;
    int4 num_spacebase(void) const;
//    unique_ptr <VarnodeDataProxy> get_spacebase(int4 i) const;
//    unique_ptr <VarnodeDataProxy> get_spacebase_full(int4 i) const;

    uintb address_to_byte(uintb val,uint4 ws) const;
    uintb byte_to_address(uintb val,uint4 ws) const;
    int4 address_to_byte_int(int4 val,uint4 ws) const;
    int4 byte_to_address_int(int4 val,uint4 ws) const;
//    bool compare_by_index(const AddrSpaceProxy *a, const AddrSpaceProxy *b) const;
    // TODO: complete the methods
};