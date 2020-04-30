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
#include "variable_proxy.hh"

unique_ptr <VarnodeProxy> VariableProxy::get_instance(int4 i) const {
    return unique_ptr <VarnodeProxy> (new VarnodeProxy{highvn->getInstance(i)});
}

bool VariableProxy::has_name(void) const {
    return highvn->hasName();
}

unique_ptr <VarnodeProxy> VariableProxy::get_tied_varnode(void) const {
    return unique_ptr <VarnodeProxy> (new VarnodeProxy{highvn->getTiedVarnode()});
}

unique_ptr <VarnodeProxy> VariableProxy::get_input_varnode(void) const {
    return unique_ptr <VarnodeProxy> (new VarnodeProxy{highvn->getInputVarnode()});
}
unique_ptr <VarnodeProxy> VariableProxy::get_type_representative(void) const {
    return unique_ptr <VarnodeProxy> (new VarnodeProxy{highvn->getTypeRepresentative()});
}
unique_ptr <VarnodeProxy> VariableProxy::get_name_representative(void) const {
    return unique_ptr <VarnodeProxy> (new VarnodeProxy{highvn->getNameRepresentative()});
}