#include <Parser.hpp>

std::list<std::unique_ptr<Parser::AST>> Parser::arena;
constexpr std::size_t Parser::MAX_PARAMS_COUNT;
constexpr std::size_t Parser::NPOS;

Parser::AST* Parser::allocate(Parser::Tag new_tag, Parser::Data new_data)
{
    arena.push_back(std::make_unique<Parser::AST>(new_tag, new_data));
    return arena.back().get();
}
std::size_t Parser::get_level(Lexer::Token token)
{
    if (token.tag != Lexer::Tag::SYMBOL) {
        throw std::runtime_error("Invalid operation");
    }

    switch (token.data.symbol)
    {
    case Lexer::Symbol::PLUS:
    case Lexer::Symbol::MIN:
        return 1;

    case Lexer::Symbol::MUL:
    case Lexer::Symbol::DIV:
    case Lexer::Symbol::MOD:
        return 2;

    default:
        throw std::runtime_error("Invalid operation");
    }

    return 0;
}

double Parser::solve(std::vector<Lexer::Token> &tokens)
{
    if (tokens.size() == 0) {
        throw std::runtime_error("No tokens");
    }

    std::size_t startTemp = 0;
    try
    {
        Parser::AST *ast = Parser::build_binary_all(tokens, startTemp, tokens.size() - 1, 0);
        double results = Parser::solve_ast(ast);
        Parser::arena.clear();
        return results;
    }
    catch (std::runtime_error &e)
    {
        Parser::arena.clear();
        throw;
    }

    return 0.0;
}
Parser::AST* Parser::build_unary(std::vector<Lexer::Token> &tokens, size_t &start, size_t end)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    if (tokens[start].tag != Lexer::Tag::SYMBOL ||
        (tokens[start].data.symbol != Lexer::Symbol::MIN && tokens[start].data.symbol != Lexer::Symbol::PLUS)) {
        return nullptr;
    }

    Lexer::Symbol symbol = tokens[start].data.symbol;
    start++;
    return Parser::allocate(Parser::Tag::UNARY, Parser::Data(symbol, build_single(tokens, start, end)));
}
Parser::AST* Parser::build_value(std::vector<Lexer::Token> &tokens, size_t &start, size_t end)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    if (tokens[start].tag != Lexer::Tag::NUMBER) {
        return nullptr;
    }

    return Parser::allocate(Parser::Tag::VAL, Parser::Data(tokens[start++].data.number));
}
Parser::AST* Parser::build_sub(std::vector<Lexer::Token> &tokens, size_t &start, size_t end)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    if (tokens[start].tag != Lexer::Tag::SYMBOL ||
        tokens[start].data.symbol != Lexer::Symbol::PAR_START) {
        return nullptr;
    }

    std::size_t origin = start + 1;
    std::size_t endPos = NPOS;
    std::size_t level = 0;
    for (start++; start <= end; start++)
    {
        if (tokens[start].tag == Lexer::Tag::SYMBOL) 
        {
            if (tokens[start].data.symbol == Lexer::Symbol::PAR_START) {
                level++;
            }
            else if (tokens[start].data.symbol == Lexer::Symbol::PAR_END)
            {
                if (level == 0) {
                    endPos = start;
                    break;
                }
                level--;
            }
        }
    }
    if (endPos == NPOS) {
        throw std::runtime_error("'(' must end with an closing ')'");
    }

    start = endPos + 1;
    return Parser::build_binary_all(tokens, origin, endPos - 1, 0);
}
Parser::AST* Parser::build_call(std::vector<Lexer::Token> &tokens, size_t &start, size_t end)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    if (tokens[start].tag != Lexer::Tag::NAME) {
        return nullptr;
    }
    std::string_view name = tokens[start].data.name;

    static auto funcs = std::to_array<std::string_view>({
        "sqrt", "pow", "log", "sin", "cos", "tan"
    });
    if (std::find(funcs.begin(), funcs.end(), tokens[start].data.name) == funcs.end()) {
        throw std::runtime_error("Unknown function");
    }
    start++;
    if ((start > end) || (tokens[start].tag != Lexer::Tag::SYMBOL || tokens[start].data.symbol != Lexer::Symbol::PAR_START)) {
        throw std::runtime_error("Function call must start with a '('");
    }

    std::size_t origin = ++start;
    std::vector<std::size_t> commas;
    std::size_t endPos = NPOS;
    std::size_t level = 0;
    for (; start <= end; start++)
    {
        if (tokens[start].tag == Lexer::Tag::SYMBOL)
        {
            if (tokens[start].data.symbol == Lexer::Symbol::PAR_START) {
                level++;
            }
            else if (tokens[start].data.symbol == Lexer::Symbol::PAR_END)
            {
                if (level == 0) {
                    endPos = start;
                    break;
                }
                level--;
            }
            else if (tokens[start].data.symbol == Lexer::Symbol::COMMA && level == 0) {
                commas.push_back(start);
            }
        }
    }
    if (endPos == NPOS) {
        throw std::runtime_error("Function call must end with a ')'");
    }
    std::size_t prev = origin;
    std::size_t c = 1;
    Parser::AST* params[Parser::MAX_PARAMS_COUNT];
    std::memset(params, 0, sizeof(params));
    for (auto &comma : commas)
    {
        Parser::AST* param = Parser::build_binary_all(tokens, prev, comma - 1, 0);
        prev = comma + 1;

        if (c >= Parser::MAX_PARAMS_COUNT) {
            throw std::runtime_error("Exceeded allowed max parameters");
        }
        params[c - 1] = param;
        c++;
    }
    params[c - 1] = Parser::build_binary_all(tokens, prev, endPos - 1, 0);
    start = endPos + 1;
    return Parser::allocate(Parser::Tag::CALL, Parser::Data(name, c, params));
}
Parser::AST* Parser::build_single(std::vector<Lexer::Token> &tokens, size_t &start, size_t end)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    Parser::AST* ast = Parser::build_unary(tokens, start, end);
    if (ast != nullptr) {
        return ast;
    }
    ast = Parser::build_value(tokens, start, end);
    if (ast != nullptr) {
        return ast;
    }
    ast = Parser::build_sub(tokens, start, end);
    if (ast != nullptr) {
        return ast;
    }
    ast = Parser::build_call(tokens, start, end);
    if (ast != nullptr) {
        return ast;
    }

    throw std::runtime_error("Unknown expression");
}
Parser::AST* Parser::build_binary_all(std::vector<Lexer::Token> &tokens, size_t &start, size_t end, size_t prev_level)
{
    if (start > end) {
        throw std::runtime_error("Empty expression");
    }

    Parser::AST* left = Parser::build_single(tokens, start, end);
    while (start <= end)
    {
        std::size_t curr_level = Parser::get_level(tokens[start]);
        Lexer::Symbol operation = tokens[start].data.symbol;

        if (curr_level <= prev_level) {
            break;
        }
        
        start++;
        Parser::AST* right = Parser::build_binary_all(tokens, start, end, curr_level);
        left = Parser::allocate(Parser::Tag::BIN, Parser::Data(left, operation, right));
    }

    return left;
}
double Parser::solve_ast(Parser::AST *ast)
{
    switch (ast->tag)
    {
    case Parser::Tag::VAL:
        return ast->data.value;

    case Parser::Tag::BIN:
    {
        double left = Parser::solve_ast(ast->data.bin.left);
        double right = Parser::solve_ast(ast->data.bin.right);

        switch (ast->data.bin.operation)
        {
        case Lexer::Symbol::PLUS:
            return left + right;
        case Lexer::Symbol::MIN:
            return left - right;
        case Lexer::Symbol::MUL:
            return left * right;
        case Lexer::Symbol::DIV:
            if (right == 0.0) throw std::runtime_error("Divison by 0");
            return left / right;
        case Lexer::Symbol::MOD:
            if (right == 0.0) throw std::runtime_error("Divison by 0");
            return std::fmod(left, right);
        }
    }
    case Parser::Tag::UNARY:
    {
        double value = Parser::solve_ast(ast->data.unary.expr);

        switch (ast->data.unary.operation)
        {
        case Lexer::Symbol::PLUS:
            return +value;
        case Lexer::Symbol::MIN:
            return -value;
        }
    }
    case Parser::Tag::CALL:
    {           
        static auto funcs = std::to_array<std::string_view>({
            "sqrt", "pow", "log", "sin", "cos", "tan"
        });

        if (ast->data.call.name == "sqrt")
        {
            if (ast->data.call.params_count != 1) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::sqrt(Parser::solve_ast(ast->data.call.params[0]));
        }
        if (ast->data.call.name == "pow")
        {
            if (ast->data.call.params_count != 2) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::pow(Parser::solve_ast(ast->data.call.params[0]), Parser::solve_ast(ast->data.call.params[1]));
        }
        if (ast->data.call.name == "log")
        {
            if (ast->data.call.params_count != 2) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::log(Parser::solve_ast(ast->data.call.params[1])) / std::log(Parser::solve_ast(ast->data.call.params[0]));
        }
        if (ast->data.call.name == "sin")
        {
            if (ast->data.call.params_count != 1) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::sin(Parser::solve_ast(ast->data.call.params[0]) * (3.1415926535897931 / 180.0));
        }
        if (ast->data.call.name == "cos")
        {
            if (ast->data.call.params_count != 1) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::cos(Parser::solve_ast(ast->data.call.params[0]) * (3.1415926535897931 / 180.0));
        }
        if (ast->data.call.name == "tan")
        {
            if (ast->data.call.params_count != 1) {
                throw std::runtime_error("Invalid parameters count");
            }
            return std::tan(Parser::solve_ast(ast->data.call.params[0]) * (3.1415926535897931 / 180.0));
        }
    }
    }

    return 0.0;
}