import os
import sys

def rename_files(directory):
    for root, dirs, files in os.walk(directory):
        for file_name in files:
            if file_name.endswith(".c") or file_name.endswith(".cpp"):
                base_name = os.path.splitext(file_name)[0]
                new_name = base_name + ".rs"
                old_path = os.path.join(root, file_name)
                new_path = os.path.join(root, new_name)
                os.rename(old_path, new_path)
                print(f"Renamed: {old_path} -> {new_path}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py /path/to/your/directory")
        sys.exit(1)

    base_directory = sys.argv[1]
    
    if not os.path.isdir(base_directory):
        print(f"Error: {base_directory} is not a valid directory.")
        sys.exit(1)

    rename_files(base_directory)

