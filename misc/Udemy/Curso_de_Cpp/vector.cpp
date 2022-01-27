#include <iostream>
#include <vector>

using namespace std;

int main()
{
    vector <int> v(3);
    v[0] = 10;
    v[1] = 20;
    v[2] = 30;

    for(unsigned int i = 0; i < v.size(); i++)
        cout << v[i] << endl;

    vector <int> v1;

    v1.push_back(10);
    v1.push_back(10);
    v1.push_back(10);

    for(unsigned int i = 0; i < v1.size(); i++)
        cout << v1.at(i) << endl;

    vector <int>::iterator it;
    
    cout << "primeiro elemento: " << *v.begin() << endl;
    cout << "ultimo elemento: " << *(--v.end()) << endl;

    vector <int>::reverse_iterator rit;

    int i = 0;
    for(rit = v.rbegin(); rit != v.rend(); rit++)
        *rit = ++i;

    for(it = v.begin(); it != v.end(); it++)
        cout << *it << endl;

    for(unsigned int i = 0; i < v1.size(); i++)
        v.erase(v.begin());

    if(!v.empty())
        cout << "Nao esta vazio\n";
    else
        cout << "Esta vazio\n";
    return 0;
}
