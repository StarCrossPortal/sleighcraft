/* A Bison parser, made by GNU Bison 3.3.2.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2019 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */

#ifndef YY_PCODEPARSE_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_PCODEPARSE_HPP_INCLUDED
# define YY_PCODEPARSE_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_PCODEPARSE_HPP_INCLUDED
/* Debug traces.  */
#ifndef PCODEPARSEDEBUG
# if defined YYDEBUG
#if YYDEBUG
#   define PCODEPARSEDEBUG 1
#  else
#   define PCODEPARSEDEBUG 0
#  endif
# else /* ! defined YYDEBUG */
#  define PCODEPARSEDEBUG 0
# endif /* ! defined YYDEBUG */
#endif  /* ! defined PCODEPARSEDEBUG */
#if PCODEPARSEDEBUG
extern int pcodeparsedebug;
#endif

/* Token type.  */
#ifndef PCODEPARSETOKENTYPE
# define PCODEPARSETOKENTYPE
  enum pcodeparsetokentype
  {
    OP_BOOL_OR = 258,
    OP_BOOL_AND = 259,
    OP_BOOL_XOR = 260,
    OP_EQUAL = 261,
    OP_NOTEQUAL = 262,
    OP_FEQUAL = 263,
    OP_FNOTEQUAL = 264,
    OP_GREATEQUAL = 265,
    OP_LESSEQUAL = 266,
    OP_SLESS = 267,
    OP_SGREATEQUAL = 268,
    OP_SLESSEQUAL = 269,
    OP_SGREAT = 270,
    OP_FLESS = 271,
    OP_FGREAT = 272,
    OP_FLESSEQUAL = 273,
    OP_FGREATEQUAL = 274,
    OP_LEFT = 275,
    OP_RIGHT = 276,
    OP_SRIGHT = 277,
    OP_FADD = 278,
    OP_FSUB = 279,
    OP_SDIV = 280,
    OP_SREM = 281,
    OP_FMULT = 282,
    OP_FDIV = 283,
    OP_ZEXT = 284,
    OP_CARRY = 285,
    OP_BORROW = 286,
    OP_SEXT = 287,
    OP_SCARRY = 288,
    OP_SBORROW = 289,
    OP_NAN = 290,
    OP_ABS = 291,
    OP_SQRT = 292,
    OP_CEIL = 293,
    OP_FLOOR = 294,
    OP_ROUND = 295,
    OP_INT2FLOAT = 296,
    OP_FLOAT2FLOAT = 297,
    OP_TRUNC = 298,
    OP_NEW = 299,
    BADINTEGER = 300,
    GOTO_KEY = 301,
    CALL_KEY = 302,
    RETURN_KEY = 303,
    IF_KEY = 304,
    ENDOFSTREAM = 305,
    LOCAL_KEY = 306,
    INTEGER = 307,
    STRING = 308,
    SPACESYM = 309,
    USEROPSYM = 310,
    VARSYM = 311,
    OPERANDSYM = 312,
    STARTSYM = 313,
    ENDSYM = 314,
    LABELSYM = 315
  };
#endif

/* Value type.  */
#if ! defined PCODEPARSESTYPE && ! defined PCODEPARSESTYPE_IS_DECLARED

union PCODEPARSESTYPE
{
#line 26 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/./pcodeparse.y" /* yacc.c:1921  */

  uintb *i;
  string *str;
  vector<ExprTree *> *param;
  StarQuality *starqual;
  VarnodeTpl *varnode;
  ExprTree *tree;
  vector<OpTpl *> *stmt;
  ConstructTpl *sem;

  SpaceSymbol *spacesym;
  UserOpSymbol *useropsym;
  LabelSymbol *labelsym;
  StartSymbol *startsym;
  EndSymbol *endsym;
  OperandSymbol *operandsym;
  VarnodeSymbol *varsym;
  SpecificSymbol *specsym;

#line 147 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/build/bison/pcodeparse.hpp" /* yacc.c:1921  */
};

typedef union PCODEPARSESTYPE PCODEPARSESTYPE;
# define PCODEPARSESTYPE_IS_TRIVIAL 1
# define PCODEPARSESTYPE_IS_DECLARED 1
#endif


extern PCODEPARSESTYPE pcodeparselval;

int pcodeparseparse (void);

#endif /* !YY_PCODEPARSE_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_PCODEPARSE_HPP_INCLUDED  */
