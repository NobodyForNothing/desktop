# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(flutter_fast_rss_data_library VERSION 0.0.1 LANGUAGES C)

add_library(flutter_fast_rss_data SHARED
  "flutter_fast_rss_data.c"
)

set_target_properties(flutter_fast_rss_data PROPERTIES
  PUBLIC_HEADER flutter_fast_rss_data.h
  OUTPUT_NAME "flutter_fast_rss_data"
)

target_compile_definitions(flutter_fast_rss_data PUBLIC DART_SHARED_LIB)
