from scripts import run_cmd, hlp

# Parse command line arguments
def get_args() -> tuple[str, list[str]]:
    # Import sys
    import sys
    # Get the directory path (This is always supplied via the './epearl' script files)
    dir_path = sys.argv[1]
    # Replace backslashes with forward slashes
    if '\\' in dir_path: dir_path = dir_path.replace('\\', '/')
    # Add a trailing slash if there isn't one
    if dir_path[-1] != "/": dir_path += "/"
    # Get the other arguments
    other_args = sys.argv[2:]
    # Return the directory path and the other arguments
    return (dir_path, other_args)

# Run the main function
def main() -> str | int | None:
    # Get the command line arguments
    (cd, args) = get_args()
    # Try to get the command and its arguments
    try:
        # Get the command and its arguments
        (cmd, args) = (args[0], args[1:])
    except IndexError:
        # If there is no command, run the help command
        return hlp()
    # Run the command
    return run_cmd(cmd, cd, args)

# Run main() if this file is run directly
if __name__ == "__main__":
    # Run main() and return its exit code
    try: exit(main())
    # If we were interrupted by the user, exit with code -1
    except KeyboardInterrupt:
        print("Interrupted by user")
        exit(-1)
    # If there was an error, print it and exit with code 1
    except Exception as e:
        print("Error: {}".format(e))
        exit(1)
