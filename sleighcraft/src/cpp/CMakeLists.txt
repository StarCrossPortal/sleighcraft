project(sleigh)
cmake_minimum_required(VERSION 3.13)

set(SLEIGH_COMPILER sleighc)
set(SLEIGH_LIBRARY ghidra_sleigh)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(DECOMPILER_SOURCE_BASE_CXX
		space.cc
		float.cc
		address.cc
		pcoderaw.cc
		translate.cc
		opcodes.cc
		globalcontext.cc
		capability.cc
		architecture.cc
		options.cc
		graph.cc
		cover.cc
		block.cc
		cast.cc
		typeop.cc
		database.cc
		cpool.cc
		comment.cc
		fspec.cc
		action.cc
		loadimage.cc
		varnode.cc
		op.cc
		type.cc
		variable.cc
		varmap.cc
		jumptable.cc
		emulate.cc
		emulateutil.cc
		flow.cc
		userop.cc
		funcdata.cc
		funcdata_block.cc
		funcdata_varnode.cc
		funcdata_op.cc
		pcodeinject.cc
		heritage.cc
		prefersplit.cc
		rangeutil.cc
		ruleaction.cc
		subflow.cc
		blockaction.cc
		merge.cc
		double.cc
		coreaction.cc
		condexe.cc
		override.cc
		dynamic.cc
		crc32.cc
		prettyprint.cc
        printlanguage_proxy.cc
		printc.cc
		printjava.cc
		memstate.cc
		opbehavior.cc
		paramid.cc
        transform.cc
        stringmanage.cc
        string_ghidra.cc
        ghidra_arch.cc
        typegrp_ghidra.cc
        cpool_ghidra.cc
        loadimage_ghidra.cc
        inject_ghidra.cc
        database_ghidra.cc
        inject_sleigh.cc
        ghidra_translate.cc
        ghidra_context.cc
        comment_ghidra.cc
)

set(DECOMPILER_SOURCE_BASE_YACC
		xml.y)

set(DECOMPILER_SOURCE_SLEIGH_CXX
		sleigh_arch.cc
		sleigh.cc
		inject_sleigh.cc
		filemanage.cc
		semantics.cc
		slghsymbol.cc
		context.cc
		sleighbase.cc
		slghpatexpress.cc
		slghpattern.cc
		pcodecompile.cc)

set(DECOMPILER_SOURCE_SLEIGH_YACC
		pcodeparse.y
		grammar.y)

set(SLEIGH_COMPILER_SOURCE_CXX
		slgh_compile.cc
		slgh_compile.hh)

set(SLEIGH_COMPILER_SOURCE_YACC
		slghparse.y)

set(SLEIGH_COMPILER_SOURCE_FLEX
		slghscan.l)

# set(SLEIGH_SOURCE_DIR ghidra/Ghidra/Features/Decompiler/src/decompile/cpp)
set(SLEIGH_SOURCE_DIR .)

list(TRANSFORM DECOMPILER_SOURCE_BASE_CXX PREPEND "${SLEIGH_SOURCE_DIR}/")
list(TRANSFORM DECOMPILER_SOURCE_SLEIGH_CXX PREPEND "${SLEIGH_SOURCE_DIR}/")
list(TRANSFORM SLEIGH_COMPILER_SOURCE_CXX PREPEND "${SLEIGH_SOURCE_DIR}/")
set(SOURCE "${DECOMPILER_SOURCE_BASE_CXX}")

find_package(BISON REQUIRED)
file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/bison")
find_package(FLEX REQUIRED)
file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/flex")

# build yacc files and append the output files to OUT_VAR
macro(build_bison OUT_VAR)
	foreach(yacc_file ${ARGN})
		get_filename_component(yacc_name "${yacc_file}" NAME_WE)
		if("${yacc_name}" STREQUAL "slghparse")
			set(yacc_prefix yy)
		else()
			set(yacc_prefix "${yacc_name}")
		endif()
		if(BISON_VERSION VERSION_LESS 2.6)
			set(BISON_COMPILE_FLAGS "--name-prefix=${yacc_prefix}")
		else()
			set(BISON_COMPILE_FLAGS "-Dapi.prefix={${yacc_prefix}}")
		endif()
		BISON_TARGET(${yacc_name}
				"${CMAKE_CURRENT_SOURCE_DIR}/${SLEIGH_SOURCE_DIR}/${yacc_file}"
				"${CMAKE_CURRENT_BINARY_DIR}/bison/${yacc_name}.cpp"
				COMPILE_FLAGS ${BISON_COMPILE_FLAGS})
		list(APPEND ${OUT_VAR} "${BISON_${yacc_name}_OUTPUTS}")
	endforeach()
endmacro()

build_bison(SOURCE ${DECOMPILER_SOURCE_BASE_YACC})
build_bison(DECOMPILER_SOURCE_SLEIGH_CXX ${DECOMPILER_SOURCE_SLEIGH_YACC})
build_bison(SLEIGH_COMPILER_SOURCE_CXX ${SLEIGH_COMPILER_SOURCE_YACC})

FLEX_TARGET(slghscan "${SLEIGH_SOURCE_DIR}/${SLEIGH_COMPILER_SOURCE_FLEX}" "${CMAKE_CURRENT_BINARY_DIR}/flex/slghscan.cpp")
list(APPEND SLEIGH_COMPILER_SOURCE_CXX "${CMAKE_CURRENT_BINARY_DIR}/flex/slghscan.cpp")
file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/flex/slghparse.tab.hh" "#include \"../bison/slghparse.hpp\"")

add_library(${SLEIGH_LIBRARY} SHARED ${SOURCE} ${DECOMPILER_SOURCE_SLEIGH_CXX})
target_include_directories(${SLEIGH_LIBRARY} PUBLIC "${SLEIGH_SOURCE_DIR}")
set_target_properties(${SLEIGH_LIBRARY} PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_executable(${SLEIGH_COMPILER} ${SLEIGH_COMPILER_SOURCE_CXX})
target_link_libraries(${SLEIGH_COMPILER} ${SLEIGH_LIBRARY})
target_include_directories(${SLEIGH_COMPILER}
	PUBLIC "${SLEIGH_SOURCE_DIR}"
	PRIVATE "${CMAKE_CURRENT_BINARY_DIR}/flex"
)

#install(TARGETS ${SLEIGH_COMPILER} ${SLEIGH_LIBRARY}
#	RUNTIME DESTINATION "bin"
#	LIBRARY DESTINATION "lib"
#)

# build sla files

file(GLOB_RECURSE SLASPEC_FILES "../sleigh/*.slaspec")
#file(GLOB_RECURSE CPEC_FILES "../sleigh/*.cspec")
#file(GLOB_RECURSE LDEFS_FILES "../sleigh/*.ldefs")
#file(GLOB_RECURSE PSPEC_FILES "../sleigh/*.pspec")

set(SLAFILES "")
set(SLEIGH_BASE "${CMAKE_CURRENT_BINARY_DIR}/specfiles")

file(MAKE_DIRECTORY "${SLEIGH_BASE}")
foreach(slaspec ${SLASPEC_FILES})
	get_filename_component(sleigh_name "${slaspec}" NAME_WE)
	get_filename_component(sleigh_dir "${slaspec}" DIRECTORY)
	set(sla_file "${SLEIGH_BASE}/${sleigh_name}.sla")

	add_custom_command(OUTPUT "${sla_file}"
			COMMAND sleighc "${slaspec}" "${sla_file}"
			MAIN_DEPENDENCY "${slaspec}"
			WORKING_DIRECTORY "${sleigh_dir}")
	list(APPEND SLAFILES "${sla_file}")
endforeach()

add_custom_target(sla ALL DEPENDS ${SLAFILES})
set(SLEIGH_INSTALL_DEST "share/specfiles")
#install(FILES ${SLAFILES} ${CPEC_FILES} ${LDEFS_FILES} ${PSPEC_FILES} DESTINATION "${CMAKE_SOURCE_DIR}/../sla/")
install(FILES ${SLAFILES} DESTINATION "${CMAKE_SOURCE_DIR}/../sla/")
get_filename_component(SLEIGHHOME_DEFAULT "${SLEIGH_INSTALL_DEST}" ABSOLUTE BASE_DIR "${CMAKE_INSTALL_PREFIX}")

#set(SLEIGH_LIBRARY "${SLEIGH_LIBRARY}" PARENT_SCOPE)
#set(SLEIGHHOME_DEFAULT "${SLEIGHHOME_DEFAULT}" PARENT_SCOPE)
