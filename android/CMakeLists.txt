cmake_minimum_required(VERSION 3.22.1)
project(ReactNativeRs)

set(CMAKE_VERBOSE_MAKEFILE ON)
set(CMAKE_CXX_STANDARD 17)

add_library(react-native-rs SHARED
    ../cpp/react-native-rs.cpp
    cpp-adapter.cpp
)

# Link against the correct Android library based on architecture
if(CMAKE_ANDROID_ARCH_ABI STREQUAL arm64-v8a)
    message(STATUS "[Android] Building for ARM64")
    target_link_directories(react-native-rs PUBLIC ../cpp/target/aarch64-linux-android/release/)
    target_link_libraries(react-native-rs libreact_native_rs.a)
elseif(CMAKE_ANDROID_ARCH_ABI STREQUAL x86_64)
    message(STATUS "[Android] Building for x86_64")
    target_link_directories(react-native-rs PUBLIC ../cpp/target/x86_64-linux-android/release/)
    target_link_libraries(react-native-rs libreact_native_rs.a)
else()
    message(FATAL_ERROR "[Android] Unsupported architecture: ${CMAKE_ANDROID_ARCH_ABI}")
endif()

# Include directories
target_include_directories(react-native-rs PRIVATE
    ../cpp
)
