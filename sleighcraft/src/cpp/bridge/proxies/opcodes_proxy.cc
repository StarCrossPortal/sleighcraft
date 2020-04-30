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
#include "opcodes_proxy.hh"

/*

static const char *opcode_name[] = {
        "BLANK", "COPY", "LOAD", "STORE",
        "BRANCH", "CBRANCH", "BRANCHIND", "CALL",
        "CALLIND", "CALLOTHER", "RETURN", "INT_EQUAL",
        "INT_NOTEQUAL", "INT_SLESS", "INT_SLESSEQUAL", "INT_LESS",
        "INT_LESSEQUAL", "INT_ZEXT", "INT_SEXT", "INT_ADD",
        "INT_SUB", "INT_CARRY", "INT_SCARRY", "INT_SBORROW",
        "INT_2COMP", "INT_NEGATE", "INT_XOR", "INT_AND",
        "INT_OR", "INT_LEFT", "INT_RIGHT", "INT_SRIGHT",
        "INT_MULT", "INT_DIV", "INT_SDIV", "INT_REM",
        "INT_SREM", "BOOL_NEGATE", "BOOL_XOR", "BOOL_AND",
        "BOOL_OR", "FLOAT_EQUAL", "FLOAT_NOTEQUAL", "FLOAT_LESS",
        "FLOAT_LESSEQUAL", "UNUSED1", "FLOAT_NAN", "FLOAT_ADD",
        "FLOAT_DIV", "FLOAT_MULT", "FLOAT_SUB", "FLOAT_NEG",
        "FLOAT_ABS", "FLOAT_SQRT", "INT2FLOAT", "FLOAT2FLOAT",
        "TRUNC", "CEIL", "FLOOR", "ROUND",
        "BUILD", "DELAY_SLOT", "PIECE", "SUBPIECE", "CAST",
        "LABEL", "CROSSBUILD", "SEGMENTOP", "CPOOLREF", "NEW",
        "INSERT", "EXTRACT", "POPCOUNT"
};
const char* OpCodesProxy::getOpname(OpCodes opc) {
    return opcode_name[(OpCode)opc];
}

OpCodes OpCodesProxy::getOpcode(const string &nm) {
    return (OpCodes) get_opcode(nm);
}

OpCodes OpCodesProxy::getBooleanflip(OpCodes opc, bool &reorder) {
    return (OpCodes) get_booleanflip((OpCode) opc, reorder);
}


*/