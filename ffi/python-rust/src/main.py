from ctypes import cdll, c_int, c_size_t

my_lib = cdll.LoadLibrary('target/debug/librust_ffi.so')

int_array_size = c_size_t(256)
int_array = (c_int * int_array_size.value)() 

my_lib.produce_us_some_numbers(int_array, int_array_size)
for i,n in enumerate(int_array):
    print(str(n) + ' ', end='')
    if (i + 1) % 16 == 0:
        print('')
print('')
