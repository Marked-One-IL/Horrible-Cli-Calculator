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
        Data(double new_number)
            : number(new_number) {
        }
        Data(Lexer::Symbol new_symbol)
            : symbol(new_symbol) {
        }
        Data(std::string_view new_name)
            : name(new_name) {
        }

        double number;
        Lexer::Symbol symbol;
        std::string_view name;
    };
    struct Token
    {
        Token(Lexer::Tag new_tag, Lexer::Data new_data) :
            tag(new_tag), data(new_data) {
        }

        friend std::ostream& operator << (std::ostream &stream, Token &token)
        {
            if (token.tag == Lexer::Tag::NAME) {
                stream << std::format("[NAME : '{}']", token.data.name);
            }
            else if (token.tag == Lexer::Tag::SYMBOL) {
                switch (token.data.symbol)
                {
                case Lexer::Symbol::PLUS:
                    stream << "[SYMBOL : '+']";
                    break;
                case Lexer::Symbol::MIN:
                    stream << "[SYMBOL : '-']";
                    break;
                case Lexer::Symbol::MUL:
                    stream << "[SYMBOL : '*']";
                    break;
                case Lexer::Symbol::DIV:
                    stream << "[SYMBOL : '/']";
                    break;
                case Lexer::Symbol::MOD:
                    stream << "[SYMBOL : '%']";
                    break;
                case Lexer::Symbol::PAR_START:
                    stream << "[SYMBOL : '(']";
                    break;
                case Lexer::Symbol::PAR_END:
                    stream << "[SYMBOL : ')']";
                    break;
                case Lexer::Symbol::COMMA:
                    stream << "[SYMBOL : ',']";
                    break;
                }
            }
            else if (token.tag == Lexer::Tag::NUMBER) {
                stream << std::format("[NUMBER : {}]", token.data.number);
            }

            return stream;
        }

        Lexer::Tag tag;
        Lexer::Data data;
    };

private:
    static std::optional<Lexer::Token> extract_symbol(std::string_view &expr)
    {
        static auto symbols = std::to_array<std::pair<std::string_view, Lexer::Symbol>>({
            {"+", Lexer::Symbol::PLUS}, {"-", Lexer::Symbol::MIN}, {"*", Lexer::Symbol::MUL}, {"/", Lexer::Symbol::DIV},
            {"%", Lexer::Symbol::MOD},  {"(", Lexer::Symbol::PAR_START}, {")", Lexer::Symbol::PAR_END}, {",", Lexer::Symbol::COMMA}
        });

        for (auto &symbol : symbols)
        {
            if (expr.starts_with(symbol.first)) {
                expr.remove_prefix(symbol.first.length());
                return Lexer::Token(Lexer::Tag::SYMBOL, symbol.second);
            }
        }

        return std::nullopt;
    }
    static std::optional<Lexer::Token> extract_name(std::string_view &expr)
    {
        if (expr.length() == 0 || (!std::isalpha(expr.front()) && expr.front() != '_')) {
            return std::nullopt;
        }

        size_t pos = 1;
        for (; pos < expr.length(); pos++)
        {
            if (!std::isalnum(expr[pos]) && expr[pos] != '_') {
                break;
            }
        }

        std::string_view t = expr.substr(0, pos);
        expr.remove_prefix(pos);
        return Lexer::Token(Lexer::Tag::NAME, t);
    }
    static std::optional<Lexer::Token> extract_number(std::string_view &expr)
    {
        if (expr.length() == 0 || !std::isdigit(expr.front())) {
            return std::nullopt;
        }

        double number{};
        auto [ptr, ec] = std::from_chars(expr.data(), expr.data() + expr.size(), number);
        size_t pos = ptr - expr.data();

        if (pos < expr.length() && std::isalpha(expr[pos])) {
            throw std::runtime_error("Invalid number");
        }
        
        expr.remove_prefix(pos);
        return Lexer::Token(Lexer::Tag::NUMBER, number);
    }

public:
    static std::vector<Lexer::Token> build(std::string_view expr)
    {
        std::vector<Lexer::Token> tokens;

        while (!expr.empty()) 
        {
            std::optional<Lexer::Token> c;
            while (!expr.empty() && (expr.front() == ' ' || expr.front() == '\t')) {
                expr.remove_prefix(1);
            }
            if (expr.empty()) {
                break;
            }

            if (auto token = Lexer::extract_symbol(expr))
            {
                tokens.push_back(token.value());
                continue;
            }
            if (auto token = Lexer::extract_name(expr))
            {
                tokens.push_back(token.value());
                continue;
            }
            if (auto token = Lexer::extract_number(expr))
            {
                tokens.push_back(token.value());
                continue;
            }

            throw std::runtime_error("Invalid token");
        }

        return tokens;
    }
};