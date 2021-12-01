#include <fstream>
#include <string>
#include <vector>
#include <vector>
#include <numeric>
#include <algorithm>
#include <iostream>

auto solve(std::vector<int> input) -> int {
    std::adjacent_difference(
        input.begin(),
        input.end(),
        input.begin()
    );
    return std::count_if(
        input.begin()+1, 
        input.end(), 
        [](const auto& i) {return i > 0;}
    );
}

int main() 
{ 
    std::ifstream file("1.txt");
    std::string str; 
    std::vector<int> input;
    while (std::getline(file, str)){
        input.push_back(std::stoi(str));
    }

    std::cout << "\n" << solve(input) << std::endl;
}