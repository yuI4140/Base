import ctypes
import os as s 
def initialize():
    if s.path.exists("./h.so"):
        s.system("sudo del ./h.so")
    s.system('g++ -fPIC -shared -o h.so ./h.cpp')
    clib=ctypes.CDLL("./h.so")
    clib.test()
    clib.cmd()