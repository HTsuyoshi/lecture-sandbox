#include <iostream>
#include <fstream>

using namespace std;

int main ()
{
    ofstream output ("teste.txt");   
    output << "eu sou um teste meu comaptriota";
    output << "OLHE PARA MIM";
    return 0;
}
