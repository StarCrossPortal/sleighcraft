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
#include "varnodedata_proxy.hh"

uintb VarnodeDataProxy::get_offset(void) const {
    return vardata.offset;
}

uint4 VarnodeDataProxy::get_size(void) const {
    return vardata.size;
}


unique_ptr <AddressProxy> VarnodeDataProxy::get_addr(void) const {
    auto addr = vardata.getAddr();
    return unique_ptr<AddressProxy> (new AddressProxy{addr});
}

bool VarnodeDataProxy::is_contains(const VarnodeDataProxy &other) const {
    return vardata.contains(other.vardata);
}

unique_ptr <AddrSpaceProxy> VarnodeDataProxy::get_space(void) const {
    auto sp = vardata.space;
    return unique_ptr<AddrSpaceProxy> (new AddrSpaceProxy{sp});
}

bool VarnodeDataProxy::not_null() const {
    return &vardata != (VarnodeData *)0;
}

