#include <iostream>
#include <exception>
#include <string.h>

using namespace std;

class SUPER : public exception
{
    protected:
        char msg[100];
    public:
        SUPER (const char* e)
        {
            strcpy (msg, e);
        }

        virtual const char* what ()
        {
            return msg;
        }
};

int fatorial (int n)
{
    if (n < 0)
        throw "Numero negativo!!";
    if (n == 0 || n == 1)
        return 1;
    return n * fatorial (n-1);
}

int main ()
{
    try 
    {
        cout << "Fatorial de 5: " << fatorial (5) << endl;
        cout << "Fatorial de -1: " << fatorial (-1) << endl;
    }
    catch (SUPER e) 
    {
        cerr << "SUPER ERRO: " << e.what() << endl;
    }
    catch (const char *e)
    {
        cerr << "Erro: " << e << endl;
    }
    catch (...)
    {
        cerr << "Erro inesperado" << endl;
    }
    return 0;
}
