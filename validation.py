from typing import List
import glob

in_files = glob.glob("input/*.in")


# TEST with one file only
in_files = in_files[:1]

LINE_START="Found:"
PATH_SPLIT="=>"


# for each <x>.in, look at <x>.out (print err if DNE)
    # get pattern from 2nd arg (whitespace delim) of x.in
    # look through <x>.out, all the Found: xxx lines
    # take the thing after Found (xxx), split by '=>' to get titles array for each path
    # now validate each path titles array

for in_file in in_files:
    out_file = in_file.replace(".in", ".out")
    out_file = out_file.replace("input", "output")

    try:
        with open(in_file, 'r') as f_in:
            # Read the pattern from the second whitespace-delimited field in the .in file
            pattern = f_in.read().split()[1]

        with open(out_file, 'r') as f_out:
            found_lines = [line for line in f_out if line.startswith(LINE_START)]

        # Extract titles for each "Found" line

        # List[List[str]]
            # each List[str] is one found path titles array e.g ['index.html', 'about.html']
        titles:List[List[str]] = []
        for line in found_lines:
            # print("Line:", line)
            actual = line.split(LINE_START)[1].strip() # everything aft Found:
            path_titles = actual.split(PATH_SPLIT)
            path_titles = list(map(lambda s: s.strip(), path_titles))
            # print("PATH TITLES:", path_titles)
            titles.append(path_titles)
        
            

        # Now you can validate each title in the titles array
        # for title in titles:
        #     # Add your validation logic here
        # print("Titles:", titles)

        for path in titles:
            print(path)
        print("\n")

    except FileNotFoundError:
        print(f"Error: {out_file} not found")