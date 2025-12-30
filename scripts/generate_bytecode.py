from langton import Transpiler

expr = "2 + 3 + -4"

if __name__ == "__main__":
    transpiler = Transpiler()
    bytecode = transpiler.compile(expr)
    print(";; expr:", expr)
    print(bytecode.encode())
