#include <cstdlib>
#include <iostream>

using namespace std;

class my_object
{
    public:
        my_object();
        ~my_object();
        void SetI(int i_value);
        int GetI();

    private:
        int i;
};

my_object::my_object() {
    cout << "HELLO CONSTRUCTOR HEREEEEEEEEEEEEEEEE" << endl;
    i = 0;
}

my_object::~my_object() {
    cout << "HELLO DECONSTRUCTOR BWAHAHHAA" << endl;
}

void my_object::SetI(int iValue)
{
    i = iValue;
}

int my_object::GetI()
{
    return i;
}

namespace haha {
    void haha() {
        while(true)
            cout << "haha";
    };
}

struct bitmap_region
{
    int top,left,bottom,right;
};

int main(int argc, char*argv[])
{
    my_object anObject;

    anObject.SetI(1);
    int i = anObject.GetI();

    cout << "i=" << i << endl;

    //haha::haha();

    return (EXIT_SUCCESS);
}
