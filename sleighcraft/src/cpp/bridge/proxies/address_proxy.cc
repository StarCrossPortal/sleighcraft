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
#include "address_proxy.hh"

bool AddressProxy::is_invalid() const {
    return addr.isInvalid();
}

int4 AddressProxy::get_addr_size() const {
    return addr.getAddrSize();
}

bool AddressProxy::is_big_endian() const {
    return addr.isBigEndian();
}

unique_ptr<AddrSpaceProxy> AddressProxy::get_space() const {
    return unique_ptr<AddrSpaceProxy>(new AddrSpaceProxy{addr.getSpace()});
}

uintb AddressProxy::get_offset() const {
    return addr.getOffset();
}

void AddressProxy::to_physical() {
    addr.toPhysical();
}

int8_t AddressProxy::get_shortcut() const {
    return addr.getShortcut();
}

bool AddressProxy::equals(const AddressProxy& other) const {
    return addr == other.addr;
}

bool AddressProxy::not_equal(const AddressProxy& other) const {
    return addr != other.addr;
}

bool AddressProxy::less_than(const AddressProxy& other) const {
    return addr < other.addr;
}

bool AddressProxy::less_equal(const AddressProxy& other) const {
    return addr <= other.addr;
}

unique_ptr<AddressProxy> AddressProxy::add(int4 off) const {
    return unique_ptr<AddressProxy>(new AddressProxy(addr) + off);
}

unique_ptr<AddressProxy> AddressProxy::sub(int4 off) const {
    return unique_ptr<AddressProxy>(new AddressProxy(addr) - off);
}

bool AddressProxy::contained_by(int4 size, const AddressProxy& op2, int4 size2) const {
    return addr.containedBy(size, op2.addr, size2);
}

int4 AddressProxy::justified_contain(int4 size, const AddressProxy& op2, int4 size2, bool forceleft) const {
    return addr.justifiedContain(size, op2.addr, size2, forceleft);
}

int4 AddressProxy::overlap(int4 skip, const AddressProxy& op, int4 size) const {
    return addr.overlap(skip, op.addr, size);
}

bool AddressProxy::is_contiguous(int4 size, const AddressProxy& loaddr, int4 losz) const {
    return addr.isContiguous(size, loaddr.addr, losz);
}

bool AddressProxy::is_constant(void) const {
    return addr.isConstant();
}

void AddressProxy::renormalize(int4 size) {
    addr.renormalize(size);
}

bool AddressProxy::is_join(void) const {
    return addr.isJoin();
}

unique_ptr<AddrSpaceProxy> addr_get_space_from_const(const AddressProxy& addr) {
    return unique_ptr<AddrSpaceProxy>(new AddrSpaceProxy{Address::getSpaceFromConst(addr.addr)});
}