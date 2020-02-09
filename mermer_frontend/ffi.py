from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int ffi_test(int);
""")

C = ffi.dlopen("../target/debug/libmermer_ffi.so")

print(C.ffi_test(9))
