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

#ifndef YY_XML_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_XML_HPP_INCLUDED
# define YY_XML_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_XML_HPP_INCLUDED
/* Debug traces.  */
#ifndef XMLDEBUG
# if defined YYDEBUG
#if YYDEBUG
#   define XMLDEBUG 1
#  else
#   define XMLDEBUG 0
#  endif
# else /* ! defined YYDEBUG */
#  define XMLDEBUG 0
# endif /* ! defined YYDEBUG */
#endif  /* ! defined XMLDEBUG */
#if XMLDEBUG
extern int xmldebug;
#endif

/* Token type.  */
#ifndef XMLTOKENTYPE
# define XMLTOKENTYPE
  enum xmltokentype
  {
    CHARDATA = 258,
    CDATA = 259,
    ATTVALUE = 260,
    COMMENT = 261,
    CHARREF = 262,
    NAME = 263,
    SNAME = 264,
    ELEMBRACE = 265,
    COMMBRACE = 266
  };
#endif

/* Value type.  */
#if ! defined XMLSTYPE && ! defined XMLSTYPE_IS_DECLARED

union XMLSTYPE
{
#line 123 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/./xml.y" /* yacc.c:1921  */

  int4 i;
  string *str;
  Attributes *attr;
  NameValue *pair;

#line 85 "/home/anciety/Desktop/Code/bincraft/sleighcraft/src/cpp/build/bison/xml.hpp" /* yacc.c:1921  */
};

typedef union XMLSTYPE XMLSTYPE;
# define XMLSTYPE_IS_TRIVIAL 1
# define XMLSTYPE_IS_DECLARED 1
#endif


extern XMLSTYPE xmllval;

int xmlparse (void);

#endif /* !YY_XML_HOME_ANCIETY_DESKTOP_CODE_BINCRAFT_SLEIGHCRAFT_SRC_CPP_BUILD_BISON_XML_HPP_INCLUDED  */
