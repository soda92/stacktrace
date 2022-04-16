def a():
    import traceback
    traceback.print_stack()

def b():
    a()

def main():
    b()

if __name__=="__main__":
    main()
