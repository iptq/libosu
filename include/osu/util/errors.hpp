#pragma once
#ifndef INCLUDE_ERRORS_HPP_
#define INCLUDE_ERRORS_HPP_

#include <exception>

namespace osu {

class ParseError : public std::exception {};

} // namespace osu

#endif
