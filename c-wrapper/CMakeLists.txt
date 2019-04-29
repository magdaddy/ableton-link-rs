cmake_minimum_required(VERSION 3.9.1)

project(link-rs)

include(../ableton/link/AbletonLinkConfig.cmake)

add_library(linkrs STATIC link_rs.cpp)
target_link_libraries(linkrs Ableton::Link)
