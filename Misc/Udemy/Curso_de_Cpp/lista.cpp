#include <iostream>
#include <list>

using namespace std;

int main ()
{
    list<int> l1;
    list<int> l2(3,10);
    list<int>::iterator it;

    l1.push_back(10);
    l1.push_back(20);
    l1.push_back(30);
    l1.push_back(40);

    for(it = l1.begin(); it != l1.end(); it++)
        cout << *it << endl;

    cout << l1.front() << endl;
    cout << l1.back() << endl;

    l1.pop_front();
    l1.pop_back();

    int vet[] = {1, 2, 3, 4};

    l1.assign (vet, vet+4);
    l1.insert(l1.begin(), 10);
    for(it = l1.begin(); it != l1.end(); it++)
        cout << *it << endl;


    return 0;
}
