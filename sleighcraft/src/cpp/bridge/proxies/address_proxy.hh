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
#ifndef BRIDGE_PROXIES_ADDRESS_PROXY
#define BRIDGE_PROXIES_ADDRESS_PROXY

#include "address.hh"
#include "addrspace_proxy.hh"
class AddrSpaceProxy;

class AddressProxy {
public:
    Address& addr;

    AddressProxy(Address &addr): addr(addr) {}
    AddressProxy(Address *addr): addr(*addr) {}
    bool is_invalid() const;
    int4 get_addr_size() const;
    bool is_big_endian() const;
    unique_ptr<AddrSpaceProxy> get_space() const;
    uintb get_offset() const;
    void to_physical();
    int8_t get_shortcut() const;
    bool equals(const AddressProxy& op2) const;
    bool not_equal(const AddressProxy& op2) const;
    bool less_than(const AddressProxy& op2) const;
    bool less_equal(const AddressProxy& op2) const;
    unique_ptr<AddressProxy> add(int4 off) const;
    unique_ptr<AddressProxy> sub(int4 off) const;
    bool contained_by(int4 size, const AddressProxy& op2, int4 size2) const;
    int4 justified_contain(int4 size, const AddressProxy& op2, int4 size2, bool forceleft) const;
    int4 overlap(int4 skip, const AddressProxy& op, int4 size) const;
    bool is_contiguous(int4 size, const AddressProxy& loaddr, int4 losz) const;
    bool is_constant(void) const;
    void renormalize(int4 size);
    bool is_join(void) const;
};

unique_ptr<AddrSpaceProxy> addr_get_space_from_const(const AddressProxy& addr);

#endif