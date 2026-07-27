#include <iostream>
#include <string>
#include <iomanip>
#include <cmath>
#include "Lexer.hpp"
#include "Parser.hpp"

static std::string input(const char *s)
{
    std::cout << s;
    std::string buffer;
    std::getline(std::cin, buffer);

    for (std::size_t i = 0; i < buffer.size(); i++) {
        buffer[i] = std::tolower(buffer[i]);
    }

    return buffer;
}
static std::string extract_argvs(int argc, char **argv)
{
    std::string string;
    for (int i = 1; i < argc; i++)
    {
        string += argv[i];
        string += ' ';
    }
    for (std::size_t i = 0; i < string.size(); i++) {
        string[i] = std::tolower(string[i]);
    }

    return string;
}

int main(int argc, char **argv)
{
    if (argc != 1)
    {
        try
        {
            std::vector<Lexer::Token> tokens = Lexer::build(extract_argvs(argc, argv));
            double results = Parser::solve(tokens);
            std::cout << std::fixed << std::setprecision(2) << results << '\n';
        }
        catch (std::runtime_error& e)
        {
            std::cout << e.what() << '\n';
        }

        return 0;
    }

    std::cout << "Functions list - sqrt, pow, log, sin, cos, tan\n";
    std::cout << "Enter 'exit' to exit\n";
    while (true)
    {
        try
        {
            std::string string = input(">> ");
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