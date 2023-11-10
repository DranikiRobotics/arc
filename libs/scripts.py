def main():
    raise Exception("TODO!")

if __name__ == "__main__":
    try: main()
    except KeyboardInterrupt:
        print("Interrupted by user")
        exit(1)
    except Exception as e:
        print("Error: {}".format(e))
        exit(1)
    exit(0)