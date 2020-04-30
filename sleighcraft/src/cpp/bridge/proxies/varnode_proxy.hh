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
#include "varnode.hh"
#include "addrspace_proxy.hh"
#include "cover_proxy.hh"
#include "address_proxy.hh"
#include "variable_proxy.hh"
class VarnodeProxy {
public:
    Varnode& varnode;
    VarnodeProxy(Varnode& varnode): varnode(varnode) {};
    VarnodeProxy(Varnode* varnode): varnode(*varnode) {};

    void set_high(const VariableProxy &tv,int2 mg) const;
    unique_ptr <AddressProxy> get_addr(void) const;
    unique_ptr<AddrSpaceProxy> get_space(void) const;
    uintb get_offset(void) const;
    int4 get_size(void) const;
    int2 get_merge_group(void) const;
    unique_ptr<CoverProxy> get_def (void) const;
    unique_ptr<VariableProxy> get_high (void) const;
    bool equals(const VarnodeProxy& op2) const;
    bool not_equal(const VarnodeProxy& op2) const;
    bool less_than(const VarnodeProxy& op2) const;
    // TODO: complete the methods
};