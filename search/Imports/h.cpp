#include <iostream>
using namespace std;
extern "C"
{
void test()
{
    cout <<"successful"<<endl;
}
void cmd()
{
    system("PY ./UI.py");
}
}