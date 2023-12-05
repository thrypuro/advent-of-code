//
// Created by thrypuro on 04/12/23.
//

#include "utils.h"
#include<fstream>
string read_file(char file_name[]){

    std::ifstream file(file_name);
    string s;
    if(file.is_open()){
        // until eof
        while(!file.eof()){
            string temp;
            file >> temp;
            if (temp.empty()) continue;
            s += temp;
            s += '\n';
        }
        s.pop_back();
        file.close();

        return s;

    }
    else{
        std::cerr<<"Failed to open file!";
        exit(-1);
    }
}

vector<string> split(string str,char separator){
    int startIndex = 0, endIndex = 0;
    vector<string> strings;
    for (int i = 0; i <= str.size(); i++) {

        // If we reached the end of the word or the end of the input.
        if (str[i] == separator || i == str.size()) {
            endIndex = i;
            string temp;
            temp.append(str, startIndex, endIndex - startIndex);
            strings.push_back(temp);
            startIndex = endIndex + 1;
        }
    }

    return strings;

}
