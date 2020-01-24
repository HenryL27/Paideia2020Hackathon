#include <iostream>
#include <algorithm>
#include <random>

using namespace std;

void shuffle_arr(int arr[], int n)
{
    unsigned seed = 0;
    shuffle (arr, arr+n, default_random_engine(seed));
    for(int i = 0; i < n; i++){
	cout << arr[i] << " ";
    }
    cout << endl;
}

int* box()
{
    int* ans = new int[9];
    for (int i = 1; i < 10; i++)
    {
	ans[i] = i;
    }
    shuffle_arr(ans, 9);
    return ans;
}

int* diagonals()
{
    int* ans = new int[81];
    for (int i = 0; i < 81; i++) { ans[i] = 0; }
    int* box1 = box();
    int* box2 = box();
    int* box3 = box();
    cout << endl;
    for (int i = 0; i < 81; i++)
    {
	ans[i+(i/3)*6] = box1[i];
	ans[30+i+(i/3)*6] = box2[i];
	ans[60+i+(i/3)*6] = box3[i];
    }
    for (int i = 0; i < 9; i ++){
	for (int j = 0; j < 9; j ++){
	    cout << ans[9*i+j] << " ";
	}
	cout << endl;
    }
    return ans;
}
int main(){
    diagonals();
    return 0;
}
