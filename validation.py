# Script to validate correctness of generated out files (in output/) as per corresponding in files
# contains specified pattern at the end of each path
# no duplicate paths (same path printed twice)
# paths should not contain cycles along them (titles)
# shouldn't see the same path twice in output

from typing import List, Optional, Tuple
import glob
from sys import argv
from urllib.parse import urlparse

INPUT_FOLDER="input"
OUTPUT_FOLDER="output"
LINE_START="" # start of a path print
PATH_SPLIT="=>" # delimiter inside a path
EMPTY_TITLE = "(Empty title)"
PRINT_TITLE_SHORT = "-t"

# check if -t flag there
def check_prints_title(inp:str)->bool:
    return PRINT_TITLE_SHORT in inp.split()
    

# contains specified pattern at the end of each path
# no duplicate paths (same path printed twice)
# paths should not contain cycles along them (titles)
    # unless the title is (Empty title), in which case its ok to see repetition
def validate_path(path_num:int, path:List[str], pattern:str, prints_title:bool)->Optional[str]:
    # print(f'Validating path {path_num} with pattern {pattern}...')
    # print(path)
    last = path[-1]
    if prints_title and (not pattern in last):
        return f'ERROR: last title "{last}" of path {path_num} does not contain pattern "{pattern}"'
    
    path_set:set[str] = set()
    for title in path:
        if title != EMPTY_TITLE and title in path_set:
            return f'ERROR: title "{title}" seen twice in path {path_num}: {path}'
        
        if not prints_title:
            parsed=urlparse(title) # url
            if bool(parsed.fragment):
                return f'ERROR: URL "{title}" has fragment in path {path_num}: {path}'
                
            
        path_set.add(title)


# for each <x>.in, look at <x>.out (print err if DNE)
# get pattern from 2nd arg (whitespace delim) of x.in
# look through <x>.out, all the Found: xxx lines
# take the thing after Found (xxx), split by '=>' to get titles array for each path
# now validate each path titles array
def process(in_files:List[str]):
    for in_file in in_files:
        out_file = in_file.replace(".in", ".out")
        out_file = out_file.replace(INPUT_FOLDER, OUTPUT_FOLDER)
        pattern = None
        prints_title=False
        

        try:
            # Read the pattern from the second whitespace-delimited field in the .in file
            with open(in_file, 'r') as f_in:
                inp = f_in.read()
                prints_title = check_prints_title(inp)
                pattern = inp.split()[1]

            # Read out file "Found: xxx" lines (individual path prints)
            with open(out_file, 'r') as f_out:
                # found_lines = [line for line in f_out if line.startswith(LINE_START)]
                found_lines = [line for line in f_out]

            # List[List[str]]
                # each List[str] is one found path titles array e.g ['index.html', 'about.html']
            titles:List[List[str]] = []
            for line in found_lines:
                # actual = line.split(LINE_START)[1].strip() # everything aft Found:
                path_titles = line.split(PATH_SPLIT)
                path_titles = list(map(lambda s: s.strip(), path_titles))
                titles.append(path_titles)
            

            print(f'Checking case {in_file} with pattern:{pattern}')
            seen_paths:set[Tuple[str]] = set()

            for (idx, path) in enumerate(titles):
                path_num = idx+1
                path_tup = tuple(path)

                if tuple(path) in seen_paths:
                    print(f'ERROR: path {path_num} was seen before in {out_file}')
                    continue
                
                seen_paths.add(path_tup)
                
                err = validate_path(path_num, path, pattern, prints_title)
                if err:
                    print(err)

        except FileNotFoundError:
            print(f"ERROR: {out_file} not found")

if __name__ == "__main__":
    if len(argv) < 2:
        in_files = glob.glob(f'{INPUT_FOLDER}/*.in')
        in_files.sort()
        process(in_files)
    else:
        # get case names
        names = argv[1:]
        in_files = list(map(lambda s: f'{INPUT_FOLDER}/{s}.in', names))
        process(in_files)



        


