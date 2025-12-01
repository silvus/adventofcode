#!/usr/bin/env python3

"""
"""


import argparse
import logging
import sys

__version__ = "202412"
__author__ = "Silvus"


# ---------------------------------------------------------------------------------------
# Logs
# ---------------------------------------------------------------------------------------
COLOR_RESET = "\033[0m"
COLOR_RED = "\033[31m"
COLOR_GREEN = "\033[32m"
COLOR_YELLOW = "\033[33m"
COLOR_BLUE = "\033[34m"
COLOR_GREY = "\033[90m"
TAB = "  "

DEBUG = True

logging.basicConfig(
    stream=sys.stdout,
    level=logging.INFO,
    format=COLOR_GREY + "[%(levelname).4s]" + COLOR_RESET + " %(message)s",
)
logger = logging.getLogger("adventofcode")

# ---------------------------------------------------------------------------------------
# File processing
# ---------------------------------------------------------------------------------------

def read_file_input(file_path):
    with open(file_path) as f:
        lines = f.read().splitlines()
        data_left = []
        data_right = []
        for line in lines:
            data = line.split()
            data_left.append(int(data[0].strip()))
            data_right.append(int(data[1].strip()))


        return sorted(data_left), sorted(data_right)


def get_distance_between_left_right(data_left, data_right):
    total = 0
    for index, dl in enumerate(data_left):
        distance = abs(dl - data_right[index])
        total += distance

    return total



# ---------------------------------------------------------------------------------------
# Launch
# ---------------------------------------------------------------------------------------


def init_cli_arg():
    # Parse args
    parser = argparse.ArgumentParser(description="Advent of code 2024")
    parser.add_argument(
        "-v",
        "--version",
        action="version",
        version="%(prog)s {0}".format(__version__),
        help="show program's version number and exit",
    )
    parser.add_argument(
        "-d", "--debug", action="store_true", help="debug flag", default=DEBUG
    )

    args = parser.parse_args()

    return args


def main():
    global DEBUG

    # Parse args
    args = init_cli_arg()
    DEBUG = args.debug

    logger.setLevel(logging.DEBUG if DEBUG else logging.INFO)

    try:

        logger.debug(args)

        data_left, data_right = read_file_input('input_day_1.txt')
        total = get_distance_between_left_right(data_left, data_right)
        logger.info(total)

        # logger.debug(data_left)
        # logger.debug(data_right)


    except KeyboardInterrupt:
        logger.error("Aborting", True)
        sys.exit(1)

    return 0


if __name__ == "__main__":
    sys.exit(main())
