import langton


if __name__ == "__main__":
    expr = "1 + 2 + (40 + -3)"
    bytecode = langton.transpile(expr)
    print(";; expr:", expr)
    print(bytecode.encode())
