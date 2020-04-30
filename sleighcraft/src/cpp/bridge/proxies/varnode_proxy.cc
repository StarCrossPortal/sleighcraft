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
#include "varnode_proxy.hh"

void VarnodeProxy::set_high(const VariableProxy &tv, int2 mg) const {
    return varnode.setHigh(tv.highvn, mg);
}

unique_ptr <AddressProxy> VarnodeProxy::get_addr(void) const {

   Address addr = const_cast <Address& > (varnode.getAddr());
   return unique_ptr <AddressProxy> (new AddressProxy{addr});
}

unique_ptr<AddrSpaceProxy> VarnodeProxy::get_space(void) const {

    return unique_ptr<AddrSpaceProxy>(new AddrSpaceProxy{varnode.getSpace()});
}

uintb VarnodeProxy::get_offset(void) const {
    return varnode.getOffset();
}

int4 VarnodeProxy::get_size(void) const {
    return varnode.getSize();
}

int2 VarnodeProxy::get_merge_group(void) const {
    return varnode.getMergeGroup();
}

unique_ptr <CoverProxy> VarnodeProxy::get_def(void) const {
    return unique_ptr <CoverProxy> (new CoverProxy{ varnode.getDef()});
}

unique_ptr <VariableProxy> VarnodeProxy::get_high(void) const {
    return unique_ptr <VariableProxy> (new VariableProxy{varnode.getHigh()});
}

bool VarnodeProxy::equals(const VarnodeProxy& other) const {
    return varnode == other.varnode;
}

bool VarnodeProxy::not_equal(const VarnodeProxy& other) const {
    return varnode != other.varnode;
}

bool VarnodeProxy::less_than(const VarnodeProxy& other) const {
    return varnode < other.varnode;
}

