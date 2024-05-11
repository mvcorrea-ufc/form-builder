#!/usr/bin/env python

import os
def concat_files_in_repo(directory, file_types, header_comment_prefix="### "):
    """
    Concatenates files of specific types in a given directory and subdirectories,
    including a header with the relative path to the file from the given directory.

    Args:
    directory (str): The root directory to start searching for files.
    file_types (list): List of file extensions to include (e.g., ['.py', '.txt']).
    header_comment_prefix (str): The prefix to use for header comments.
    
    Returns:
    str: A single string with all file contents concatenated for the specified file types.
    """
    result = ""
    for root, _, files in os.walk(directory):
        for file in files:
            file_path = os.path.join(root, file)
            relative_path = os.path.relpath(file_path, start=directory)  # get the relative path
            _, file_extension = os.path.splitext(file_path)
            if file_extension in file_types:
                # Add a header with the relative path of the file
                result += f"{header_comment_prefix} File: {relative_path}\n\n"
                try:
                    with open(file_path, 'r', encoding='utf-8') as file:
                        result += file.read() + "\n\n"
                except Exception as e:
                    # Handle possible exceptions (like trying to read binary files)
                    result += f"{header_comment_prefix} Error reading {relative_path}: {e}\n\n"
    return result

# Example usage
directory = "./"
desired_file_types = ['.rs', '.toml', ".html", ".js"]  # Only process Python and Markdown files
all_files_content = concat_files_in_repo(directory, desired_file_types)
print(all_files_content)
