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
#ifndef BRIDGE_PROXIES_VARIABLE_PROXY
#define BRIDGE_PROXIES_VARIABLE_PROXY

#include "variable.hh"
#include "varnode_proxy.hh"
class VariableProxy {
public:
    HighVariable* highvn;

    VariableProxy(HighVariable* highvn): highvn(highvn) {};
    VariableProxy(HighVariable& highvn): highvn(&highvn) {};
    unique_ptr <VarnodeProxy> get_instance(int4 i) const;
    bool has_name(void) const;
    unique_ptr <VarnodeProxy> get_tied_varnode(void) const;
    unique_ptr <VarnodeProxy> get_input_varnode(void) const;
    unique_ptr <VarnodeProxy> get_type_representative(void) const;
    unique_ptr <VarnodeProxy> get_name_representative(void) const;

    // TODO: complete the methods
};

#endif