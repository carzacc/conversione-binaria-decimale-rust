#!/usr/bin/env python3
from cffi import FFI

ffi = FFI()

ffi.cdef("""
    unsigned long long int tobin(unsigned long long int a);
    unsigned long long todec(unsigned long long int a);
    bool bin(unsigned long long int a);
    unsigned long long int pow(unsigned long long int a, unsigned long long int b);
""")

libconversione = ffi.dlopen("./target/release/libconversione.so")

bad = True
while bad:
    print("Versione scritta in Rust e Python di github.com/BiagioeCarmine/convertitore-binario-decimale")
    print("Binario->Decimale o Decimale->Binario?")
    risposta = input()
    if risposta == "Binario->Decimale" or risposta == "Decimale->Binario":
        bad = False
print("Inserisci un numero da convertire")
numeroin = int(input())
if risposta == "Decimale->Binario":
    convertito = libconversione.tobin(numeroin)
if risposta == "Binario->Decimale":
    convertito = libconversione.todec(numeroin)
print("Il numero convertito è", convertito)
