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
#include "addrspace_proxy.hh"

const string& AddrSpaceProxy::get_name(void) const {
    return space.getName();
}

SpaceType AddrSpaceProxy::get_type(void) const {
    return (SpaceType) space.getType();
}

int4 AddrSpaceProxy::get_delay(void) const {
    return space.getDelay();
}

int4 AddrSpaceProxy::get_deadcode_delay(void) const {
    return space.getDeadcodeDelay();
}

int4 AddrSpaceProxy::get_index(void) const {
    return space.getIndex();
}

uint4 AddrSpaceProxy::get_wordsize(void) const {
    return space.getWordSize();
}

uint4 AddrSpaceProxy::get_addrsize(void) const {
    return space.getAddrSize();
}

uintb AddrSpaceProxy::get_highest(void) const {
    return space.getHighest();
}

uintb AddrSpaceProxy::get_pointer_lower_bound(void) const {
    return space.getPointerLowerBound();
}

uintb AddrSpaceProxy::get_pointer_upper_bound(void) const {
    return space.getPointerUpperBound();
}

int4 AddrSpaceProxy::get_minimum_ptr_size(void) const {
    return space.getMinimumPtrSize();
}

uintb AddrSpaceProxy::wrap_offset(uintb off) const {
    return space.wrapOffset(off);
}

int8_t AddrSpaceProxy::get_shortcut(void) const {
    return space.getShortcut();
}

bool AddrSpaceProxy::is_heritaged(void) const {
    return space.isHeritaged();
}

bool AddrSpaceProxy::does_deadcode(void) const {
    return space.doesDeadcode();
}

bool AddrSpaceProxy::has_physical(void) const {
    return space.hasPhysical();
}

bool AddrSpaceProxy::is_big_endian(void) const {
    return space.isBigEndian();
}

bool AddrSpaceProxy::is_reverse_justified(void) const {
    return space.isReverseJustified();
}

bool AddrSpaceProxy::is_overlay(void) const {
    return space.isOverlay();
}

bool AddrSpaceProxy::is_overlay_base(void) const {
    return space.isOverlayBase();
}

bool AddrSpaceProxy::is_other_space(void) const {
    return space.isOtherSpace();
}

bool AddrSpaceProxy::is_truncated(void) const {
    return space.isTruncated();
}

bool AddrSpaceProxy::has_near_pointers(void) const {
    return space.hasNearPointers();
}

void AddrSpaceProxy::print_offset(ostream &s, uintb offset) const {
    return space.printOffset(s, offset);
}

int4 AddrSpaceProxy::num_spacebase(void) const {
    return space.numSpacebase();
}
//
//unique_ptr <VarnodeDataProxy> AddrSpaceProxy::get_spacebase(int4 i) const {
//    return unique_ptr <VarnodeDataProxy> (new VarnodeDataProxy{ const_cast<VarnodeData& > (space.getSpacebase(i))});
//}
//
//unique_ptr <VarnodeDataProxy> AddrSpaceProxy::get_spacebase_full(int4 i) const {
//    return unique_ptr <VarnodeDataProxy> (new VarnodeDataProxy{ const_cast<VarnodeData& > (space.getSpacebaseFull(i))});
//}

uintb AddrSpaceProxy::address_to_byte(uintb val, uint4 ws) const {
    return space.addressToByte(val, ws);
}

uintb AddrSpaceProxy::byte_to_address(uintb val, uint4 ws) const {
    return space.byteToAddress(val, ws);
}

int4 AddrSpaceProxy::byte_to_address_int(int4 val, uint4 ws) const {
    return space.byteToAddressInt(val, ws);
}

int4 AddrSpaceProxy::address_to_byte_int(int4 val, uint4 ws) const {
    return space.addressToByteInt(val, ws);
}
//
//bool AddrSpaceProxy::compare_by_index(const AddrSpaceProxy &a, const AddrSpaceProxy &b) const {
//
//    return space.compareByIndex(a->space, b->space);
//}


