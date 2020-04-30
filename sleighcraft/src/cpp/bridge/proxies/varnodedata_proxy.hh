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
#include "pcoderaw.hh"
#include "address_proxy.hh"
#include "addrspace_proxy.hh"

class VarnodeDataProxy {
public:
    VarnodeData& vardata;

    VarnodeDataProxy(VarnodeData& vardata): vardata(vardata) {};
    VarnodeDataProxy(VarnodeData* vardata): vardata(*vardata) {};
    uintb get_offset(void) const;
    uint4 get_size(void) const;

    bool not_null() const;
    unique_ptr <AddressProxy> get_addr(void) const;
    unique_ptr <AddrSpaceProxy>get_space(void) const;
    bool is_contains(const VarnodeDataProxy &op2) const;

    //TODO: implemented void restoreXml
};

