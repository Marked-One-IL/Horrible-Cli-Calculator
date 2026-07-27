#pragma once
#include "Lexer.hpp"
#include <algorithm>
#include <array>
#include <stdexcept>
#include <string_view>
#include <list>
#include <vector>
#include <memory>
#include <utility>
#include <cmath>
#include <cstring>

class Parser
{
public:
    static double solve(std::vector<Lexer::Token> &tokens);

private:
    struct AST;
    static std::list<std::unique_ptr<Parser::AST>> arena;
    static constexpr std::size_t MAX_PARAMS_COUNT = 2;
    static constexpr std::size_t NPOS = SIZE_MAX;

    enum class Tag : uint8_t
    {
        VAL,
        BIN,
        UNARY,
        CALL,
    };
    union Data
    {
        Data(double new_value)
            : value(new_value) {
        }
        Data(Lexer::Symbol new_operation, Parser::AST *new_expr)
            : unary({ new_operation, new_expr }) {
        }
        Data(Parser::AST *new_left, Lexer::Symbol new_operation, Parser::AST *new_right)
            : bin({ new_left, new_operation, new_right }) {
        }
        Data(std::string_view new_name, std::size_t new_params_count, Parser::AST **new_params)
            : call({ new_name , new_params_count }) {
            std::memcpy(call.params, new_params, sizeof(Parser::AST*) * new_params_count);
        }

        double value;
        struct
        {
            Lexer::Symbol operation;
            Parser::AST *expr;
        } unary;
        struct
        {
            Parser::AST *left;
            Lexer::Symbol operation;
            Parser::AST *right;
        } bin;
        struct
        {
            std::string_view name;
            std::size_t params_count;
            Parser::AST *params[Parser::MAX_PARAMS_COUNT];
        } call;
    } data;
    struct AST
    {
        AST(Parser::Tag new_tag, Parser::Data new_data)
            : tag(new_tag), data(new_data) {
        }

        Parser::Tag tag;
        Parser::Data data;
    };
    
    static Parser::AST* allocate(Parser::Tag new_tag, Parser::Data new_data);
    static std::size_t get_level(Lexer::Token token);

    static Parser::AST* build_unary(std::vector<Lexer::Token> &tokens, size_t &start, size_t end);
    static Parser::AST* build_value(std::vector<Lexer::Token> &tokens, size_t &start, size_t end);
    static Parser::AST* build_sub(std::vector<Lexer::Token> &tokens, size_t &start, size_t end);
    static Parser::AST* build_call(std::vector<Lexer::Token> &tokens, size_t &start, size_t end);
    static Parser::AST* build_single(std::vector<Lexer::Token> &tokens, size_t &start, size_t end);
    static Parser::AST* build_binary_all(std::vector<Lexer::Token> &tokens, size_t &start, size_t end, size_t prev_level);
    static double solve_ast(Parser::AST *ast);
};