import pyfathom


def main():
    msg = pyfathom.greet("Eric")
    print(msg)
    print("Fathom version:", pyfathom.fathom_version())


if __name__ == "__main__":
    main()
