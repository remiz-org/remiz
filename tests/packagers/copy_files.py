#!/usr/bin/python3

import argparse
import os
import shutil


"""
Simple subpackager that copy files and deploy it else where...
--- Build ---
Take a list of key/value as argument.
Ignore all keys and copy all values (paths) in the folder specified by --path
--- Deploy ---
Put all files from the folder in the destination folder.
"""


def get_args():
    parser = argparse.ArgumentParser(description='copy_files packager.')

    subparser = parser.add_subparsers(dest="subcommand", required=True)
    build_parser = subparser.add_parser("build")
    build_parser.add_argument('--path', required=True)

    deploy_parser = subparser.add_parser("deploy")
    deploy_parser.add_argument('--path', required=True)
    deploy_parser.add_argument('--env', required=True)

    return parser.parse_known_args()


def build(folder_path, files_to_copy):
    for file in filter(lambda x: not x.startswith("-"), files_to_copy):
        print("Copying {} to {}".format(file, folder_path))
        dest_fpath = os.path.join(folder_path, file)
        os.makedirs(os.path.dirname(dest_fpath), exist_ok=True)
        try:
            shutil.copy(file, dest_fpath)
        except FileNotFoundError:
            print("File {} not found".format(file))


def deploy(path, env):
    print(path)
    print(env)

    for subdir, dirs, files in os.walk(path):
        for file in files:
            filepath = subdir + os.sep + file

            print(filepath)
            # print file size
            print(os.path.getsize(filepath))


if __name__ == "__main__":
    # get args into variable "args"
    args, unknown_args = get_args()

    if args.subcommand == "build":
        build(args.path, unknown_args)

    if args.subcommand == "deploy":
        deploy(args.path, args.env)
