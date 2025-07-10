#include <stdio.h>

int main() {
int x;
x = 10;
int y;
y = 4;
int z;
z =(y + x);
char s[]="stringlit";
printf("%d\n", z);
if (z < 5) {
printf("%s\n", "true!");
}
else {
printf("%s\n", "false!");
}
printf("%s\n", s);
    return 0;
}
