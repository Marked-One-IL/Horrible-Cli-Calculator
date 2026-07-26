#include <iostream>
#include <string>
#include <iomanip>
#include <cmath>
#include "Lexer.hpp"
#include "Parser.hpp"

static std::string input(const char* s)
{
    std::cout << s;
    std::string buffer;
    std::getline(std::cin, buffer);

    for (std::size_t i = 0; i < buffer.size(); i++) {
        buffer[i] = std::tolower(buffer[i]);
    }

    return buffer;
}

int main()
{
    std::cout << "Functions list - sqrt, pow, log, sin, cos, tan\n";

    while (true)
    {
        try
        {
            std::string string = input("Enter an expression: ");
            if (string == "exit") {
                break;
            }

            std::vector<Lexer::Token> tokens = Lexer::build(string);
            double results = Parser::solve(tokens);
            std::cout << std::fixed << std::setprecision(2) << results << '\n';
        }
        catch (std::runtime_error& e)
        {
            std::cout << e.what() << '\n';
        }
    }

    return 0;
}