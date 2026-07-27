#pragma once
#include <string_view>
#include <vector>
#include <array>
#include <optional>
#include <cctype>
#include <stdexcept>
#include <charconv>
#include <ostream>
#include <format>
#include <cstdint>

class Lexer
{
public:
    enum class Tag : uint8_t
    {
        NAME,
        SYMBOL,
        NUMBER,
    };
    enum class Symbol : uint8_t
    {
        PLUS,
        MIN,
        MUL,
        DIV,
        MOD,
        PAR_START,
        PAR_END,
        COMMA,
    };
    union Data
    {
        Data(double new_number);
        Data(Lexer::Symbol new_symbol);
        Data(std::string_view new_name);

        const double number;
        const Lexer::Symbol symbol;
        const std::string_view name;
    };
    struct Token
    {
        Token(Lexer::Tag new_tag, Lexer::Data new_data);

        friend std::ostream& operator << (std::ostream &stream, Token &token);

        const Lexer::Tag tag;
        const Lexer::Data data;
    };
    static std::vector<Lexer::Token> build(std::string_view expr);

private:
    static std::optional<Lexer::Token> extract_symbol(std::string_view &expr);
    static std::optional<Lexer::Token> extract_name(std::string_view &expr);
    static std::optional<Lexer::Token> extract_number(std::string_view &expr);
};