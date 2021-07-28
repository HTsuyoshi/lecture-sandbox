#include <iostream>
#include <vector>

using namespace std;

int main()
{
    vector<int> a(2, 20);
    vector<int> b(3, 10);

    a.swap(b);

    cout << "a" << endl;
    for (unsigned int i = 0; i < a.size(); i++)
        cout << a[i] << endl;

    cout << "b" << endl;
    for (unsigned int i = 0; i < b.size(); i++)
        cout << b[i] << endl;

    a.clear();

    if (a.empty())
        cout << "vazio\n";

    return 0;
}

