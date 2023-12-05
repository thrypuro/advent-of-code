//
// Created by thrypuro on 04/12/23.
//

#include "day3.h"
#include "utils.h"
#include<bits/stdc++.h>

void day3::solve() {
    char file_name [] = "/home/thrypuro/Desktop/projects/advent-of-code/aoc-2015/inputs/day3.txt";
//    char file_name [] = "/home/thrypuro/Desktop/projects/advent-of-code/aoc-2015/inputs/tests/test_day2.txt";

    auto f = read_file(file_name);
    // print f

    int size = f.size();

    int sum = 1;

    std::unordered_map<int,bool> visted;

    visted[0] = true;

    int x = 0,y = 0;



    for (auto c : f){
        switch (c) {
            case '^':
                y++;
                break;
            case 'v':
                y--;
                break;
            case '>':
                x++;
                break;
            case '<':
                x--;
                break;
            default:
                std::cout <<" Error is " << c << std::endl;
                std::cerr << "Error in switch case" << std::endl;
                exit(-1);
        }
        int key = x*size + y;
        if(visted.find(key) == visted.end()){
            visted[key] = true;
            sum++;
        }
    }


    std::cout << "Total moves needed " << sum << std::endl;

    // part 2

    std::unordered_map<int,bool> visited;
    sum = 1;
    visited[0] = true;

    int sx = 0,sy = 0,rx= 0,ry = 0;
    int i = 0;
    std::unordered_map<int,bool> *v;
    for (auto c : f){
        if (i%2 == 0){
            x = sx;
            y = sy;

        }
        else{
            x = rx;
            y = ry;

        }
        switch (c) {
            case '^':
                y++;
                break;
            case 'v':
                y--;
                break;
            case '>':
                x++;
                break;
            case '<':
                x--;
                break;
            default:
                std::cout <<" Error is " << c << std::endl;
                std::cerr << "Error in switch case" << std::endl;
                exit(-1);
        }
        int key = x*size + y;
        if(visited.find(key) == visited.end()){
            visited[key] = true;
            sum++;
        }
        if (i%2 == 0){
            sx = x;
            sy = y;
        }
        else{
            rx = x;
            ry = y;
        }
        i++;

    }


    std::cout << "Total moves needed " << sum << std::endl;

    std::cout << "Total visited " << visited.size() << " houses" << std::endl;



}
