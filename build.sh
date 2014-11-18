rustc monkey.rs -o libmonkey.so
gcc -std=c99 -Wall -g main.c -o main -L. -lmonkey
echo "run!"
./run.sh
