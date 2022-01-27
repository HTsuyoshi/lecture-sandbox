#include <iostream>
#include <fstream>

using namespace std;

int main ()
{
    ifstream in ("teste.txt");

    string texto;
    char c = in.get();
    texto.push_back(c);

    cout << "Mostrar o texto\n";

    // existe in.fail()
    while (in.good())
    {
        cout << c;
        c = in.get();
        texto.push_back(c);
    }
    return 0;
}
