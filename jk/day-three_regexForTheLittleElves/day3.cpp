#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>

std::string readFileToString(const std::string &data) {
  std::ifstream file(data);

  std::stringstream buffer;
  buffer << file.rdbuf();

  file.close();

  return buffer.str();
}

int main(int argc, char *argv[]) {

  std::string input = readFileToString("data.txt");
  // std::regex pattern("mul\\((\\d+),(\\d+)\\)");

  std::regex pattern("(mul\\((\\d+),(\\d+)\\))|(do\\(\\))|(don't\\(\\))");
  std::sregex_iterator it(input.begin(), input.end(), pattern);
  std::sregex_iterator end;
  std::string result;

  int num1 = 0;
  int num2 = 0;
  int totSum = 0;
  std::smatch match;
  bool isLastKeyWordDo = true;
  while (it != end) {
    match = *it;

    if (match[1].matched) {
      if (isLastKeyWordDo) {

        num1 = std::stoi(match[2].str());
        num2 = std::stoi(match[3].str());
        totSum += (num1 * num2);
      }
    } else if (match[4].matched) {
      isLastKeyWordDo = true;
    } else if (match[5].matched) {
      isLastKeyWordDo = false;
    }
    it++;
  }
  std::cout << "tot sum! : " << totSum << std::endl;
  return 0;
}
