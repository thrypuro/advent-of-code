//
// Created by thrypuro on 04/12/23.
//

#include "day1.h"
#include "utils.h"
void day1::solve() {

    char file_name [] = "/home/thrypuro/Desktop/projects/advent-of-code/aoc-2015/inputs/day1.txt";

    auto f = read_file(file_name);

    long sum = 0;
    auto ba = false;
    for(int i = 0; i< f.length();i++){
     if (f[i]=='(') sum+=1;
     if (f[i]==')') sum-=1;
     if (sum==-1 && !ba) {std::cout << "First position to enter basement is "<<i +1 << "\n"; ba=true;}
    }

    std::cout << "Sum = " << sum << "\n";

}
