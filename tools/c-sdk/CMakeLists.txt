cmake_minimum_required(VERSION 3.20)
project(cqlib_tianyan_c_tests LANGUAGES C)
enable_testing()

# Test the staged SDK, never an implicit -l lookup in the Cargo build tree.
if(NOT CQLIB_TIANYAN_SDK_ROOT)
    message(FATAL_ERROR "Set CQLIB_TIANYAN_SDK_ROOT to an unpacked/staged C SDK")
endif()

add_library(cqlib_tianyan_static STATIC IMPORTED)
add_library(cqlib_tianyan_shared SHARED IMPORTED)
if(WIN32)
    set_target_properties(cqlib_tianyan_static PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/lib/binding_c.lib")
    set_target_properties(cqlib_tianyan_shared PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/bin/binding_c.dll"
        IMPORTED_IMPLIB "${CQLIB_TIANYAN_SDK_ROOT}/lib/binding_c.dll.lib")
elseif(APPLE)
    set_target_properties(cqlib_tianyan_static PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/lib/libbinding_c.a")
    set_target_properties(cqlib_tianyan_shared PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/lib/libbinding_c.dylib")
else()
    set_target_properties(cqlib_tianyan_static PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/lib/libbinding_c.a")
    set_target_properties(cqlib_tianyan_shared PROPERTIES
        IMPORTED_LOCATION "${CQLIB_TIANYAN_SDK_ROOT}/lib/libbinding_c.so")
endif()

if(UNIX AND NOT APPLE)
    find_package(OpenSSL REQUIRED)
endif()

foreach(kind static shared)
    add_executable(test_${kind} test_api.c)
    target_include_directories(test_${kind} PRIVATE "${CQLIB_TIANYAN_SDK_ROOT}/include")
    target_link_libraries(test_${kind} PRIVATE cqlib_tianyan_${kind})
    # The C tests put API calls inside assert()/EXPECT_*; retain them even in Release.
    if(MSVC)
        target_compile_options(test_${kind} PRIVATE /UNDEBUG)
    else()
        target_compile_options(test_${kind} PRIVATE -UNDEBUG)
    endif()
    if(WIN32)
        target_link_libraries(test_${kind} PRIVATE
            ntdll ws2_32 bcrypt userenv advapi32 crypt32 secur32 ncrypt iphlpapi)
    elseif(APPLE)
        target_link_libraries(test_${kind} PRIVATE
            c++
            "-framework CoreFoundation"
            "-framework Security"
            "-framework SystemConfiguration")
    else()
        target_link_libraries(test_${kind} PRIVATE
            pthread dl m OpenSSL::SSL OpenSSL::Crypto)
    endif()
    set_target_properties(test_${kind} PROPERTIES
        BUILD_RPATH "${CQLIB_TIANYAN_SDK_ROOT}/lib")
    add_test(NAME c_${kind} COMMAND test_${kind})
endforeach()


if(WIN32)
    add_custom_command(TARGET test_shared POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "$<TARGET_FILE:cqlib_tianyan_shared>" "$<TARGET_FILE_DIR:test_shared>")
endif()
