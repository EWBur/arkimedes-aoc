#include <fstream>
#include <iostream>
#include <vector>
void readMatrixFromFile(const std::string &filename,
                        std::vector<std::vector<char>> &matrix) {
  std::ifstream file(filename);

  std::string line;
  while (std::getline(file, line)) {
    std::vector<char> row;
    for (char c : line) {
      if (c != ' ' && c != '\t') {
        row.push_back(c);
      }
    }

    if (!row.empty()) {
      matrix.push_back(row);
    }
  }

  file.close();
}

bool isWithinBounds(const std::vector<std::vector<char>> &matrix, int x,
                    int y) {
  return x >= 0 && y >= 0 && x < matrix[0].size() && y < matrix.size();
}

void releaseTheMasGnome(const std::vector<std::vector<char>> &matrix) {
  int totFindings = 0;

  std::string pattern1 = "MMSS";
  std::string pattern2 = "MSMS";
  std::string pattern3 = "SMSM";
  std::string pattern4 = "SSMM";

  for (int i = 1; i < matrix.size() - 1; ++i) {
    for (int j = 1; j < matrix[0].size() - 1; ++j) {

      /* Check for the middle of the cross */
      if (matrix[i][j] == 'A') {
        std::string corners;
        corners += matrix[i - 1][j - 1]; /* Upper Left  */
        corners += matrix[i - 1][j + 1]; /* Upper Right */
        corners += matrix[i + 1][j - 1]; /* Lower Left  */
        corners += matrix[i + 1][j + 1]; /* Lower Right */

        if (corners == pattern1 || corners == pattern2 || corners == pattern3 ||
            corners == pattern4) {
          ++totFindings;
        }
      }
    }
  }
  std::cout << "Total XMAS findings: " << totFindings << std::endl;
}

void releaseTheXmasGnome(std::vector<std::vector<char>> &matrix) {
  int totFindings = 0;
  for (int i = 0; i < matrix.size(); ++i) {
    for (int j = 0; j < matrix[i].size(); ++j) {

      /* Rows */
      if (j + 3 < matrix[i].size() && matrix[i][j] == 'X' &&
          matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' &&
          matrix[i][j + 3] == 'S') {
        totFindings++;
      }

      if (j + 3 < matrix[i].size() && matrix[i][j] == 'S' &&
          matrix[i][j + 1] == 'A' && matrix[i][j + 2] == 'M' &&
          matrix[i][j + 3] == 'X') {
        totFindings++;
      }

      /* Columns */
      if (i + 3 < matrix.size() && matrix[i][j] == 'X' &&
          matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' &&
          matrix[i + 3][j] == 'S') {
        totFindings++;
      }

      if (i + 3 < matrix.size() && matrix[i][j] == 'S' &&
          matrix[i + 1][j] == 'A' && matrix[i + 2][j] == 'M' &&
          matrix[i + 3][j] == 'X') {
        totFindings++;
      }

      /* Diagonal \ */
      if (i + 3 < matrix.size() && j + 3 < matrix[i].size() &&
          matrix[i][j] == 'X' && matrix[i + 1][j + 1] == 'M' &&
          matrix[i + 2][j + 2] == 'A' && matrix[i + 3][j + 3] == 'S') {
        totFindings++;
      }

      if (i + 3 < matrix.size() && j + 3 < matrix[i].size() &&
          matrix[i][j] == 'S' && matrix[i + 1][j + 1] == 'A' &&
          matrix[i + 2][j + 2] == 'M' && matrix[i + 3][j + 3] == 'X') {
        totFindings++;
      }

      /* Diagonal / */
      if (i - 3 >= 0 && j + 3 < matrix[i].size() && matrix[i][j] == 'X' &&
          matrix[i - 1][j + 1] == 'M' && matrix[i - 2][j + 2] == 'A' &&
          matrix[i - 3][j + 3] == 'S') {
        totFindings++;
      }
      if (i - 3 >= 0 && j + 3 < matrix[i].size() && matrix[i][j] == 'S' &&
          matrix[i - 1][j + 1] == 'A' && matrix[i - 2][j + 2] == 'M' &&
          matrix[i - 3][j + 3] == 'X') {
        totFindings++;
      }
    }
  }
  std::cout << "Total XMAS/SAMX: " << totFindings << std::endl;
}

int main(int argc, char *argv[]) {

  std::vector<std::vector<char>> matrix;
  readMatrixFromFile("data.txt", matrix);

  releaseTheXmasGnome(matrix);
  releaseTheMasGnome(matrix);
  return 0;
}
