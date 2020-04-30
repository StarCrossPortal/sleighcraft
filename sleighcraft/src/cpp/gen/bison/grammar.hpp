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

#ifndef YY_GRAMMAR_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_GRAMMAR_HPP_INCLUDED
# define YY_GRAMMAR_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_GRAMMAR_HPP_INCLUDED
/* Debug traces.  */
#ifndef GRAMMARDEBUG
# if defined YYDEBUG
#if YYDEBUG
#   define GRAMMARDEBUG 1
#  else
#   define GRAMMARDEBUG 0
#  endif
# else /* ! defined YYDEBUG */
#  define GRAMMARDEBUG 0
# endif /* ! defined YYDEBUG */
#endif  /* ! defined GRAMMARDEBUG */
#if GRAMMARDEBUG
extern int grammardebug;
#endif

/* Token type.  */
#ifndef GRAMMARTOKENTYPE
# define GRAMMARTOKENTYPE
  enum grammartokentype
  {
    DOTDOTDOT = 258,
    BADTOKEN = 259,
    STRUCT = 260,
    UNION = 261,
    ENUM = 262,
    DECLARATION_RESULT = 263,
    PARAM_RESULT = 264,
    NUMBER = 265,
    IDENTIFIER = 266,
    STORAGE_CLASS_SPECIFIER = 267,
    TYPE_QUALIFIER = 268,
    FUNCTION_SPECIFIER = 269,
    TYPE_NAME = 270
  };
#endif

/* Value type.  */
#if ! defined GRAMMARSTYPE && ! defined GRAMMARSTYPE_IS_DECLARED

union GRAMMARSTYPE
{
#line 25 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/./grammar.y" /* yacc.c:1921  */

  uint4 flags;
  TypeDeclarator *dec;
  vector<TypeDeclarator *> *declist;
  TypeSpecifiers *spec;
  vector<uint4> *ptrspec;
  Datatype *type;
  Enumerator *enumer;
  vector<Enumerator *> *vecenum;
  string *str;
  uintb *i;

#line 95 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/build/bison/grammar.hpp" /* yacc.c:1921  */
};

typedef union GRAMMARSTYPE GRAMMARSTYPE;
# define GRAMMARSTYPE_IS_TRIVIAL 1
# define GRAMMARSTYPE_IS_DECLARED 1
#endif


extern GRAMMARSTYPE grammarlval;

int grammarparse (void);

#endif /* !YY_GRAMMAR_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_GRAMMAR_HPP_INCLUDED  */
