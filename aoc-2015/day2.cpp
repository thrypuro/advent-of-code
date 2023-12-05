//
// Created by thrypuro on 04/12/23.
//

#include "day2.h"
#include "utils.h"
void day2::solve() {
    char file_name [] = "/home/thrypuro/Desktop/projects/advent-of-code/aoc-2015/inputs/day2.txt";
//    char file_name [] = "/home/thrypuro/Desktop/projects/advent-of-code/aoc-2015/inputs/tests/test_day2.txt";

    auto f = read_file(file_name);
    // print f
    auto ve = split(f,'\n');


    int sum = 0;

    int ribbon_sum = 0;

    for(const auto& line : ve){
        int l,w,h;
        int i = 0;
        // print line
        for(size_t j=0; j < line.length(); j++){
            int temp = 0;
            while(j<line.length() && line[j] != 'x' ){
                temp = temp*10 + (line[j]%0x10);
                j++;
            }
            switch (i) {
                case 0:
                    l = temp;
                    i++;
                    break;
                case 1:
                    w = temp;
                    i++;
                    break;
                case 2:
                    h = temp;
                    break;
                default:
                    std::cerr << "Error in switch case" << std::endl;
                    exit(-1);
            }
        }
        // print l w h
//        std::cout << l << " " << w << " " << h << std::endl;


        sum += 2*(l*h + l*w + w*h) + std::min(std::min(l*h,l*w),w*h);

        int smallest_side [] =  {2*(l+h),2*(l+w),2*(w+h) };

        ribbon_sum+= *std::min_element(smallest_side,smallest_side+3) + l*w*h;


    }

    std::cout << "Total  wrapping paper need is " << sum << std::endl;

    std::cout << "Total ribbon dimensions need is " << ribbon_sum << std::endl;

}
