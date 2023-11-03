# Analysis to check correctness of output for jane.in i.e
    # https://blog.janestreet.com/what-the-interns-have-wrought-2023/ Jane 1

# links from DOM API on page for jane.in, de-duped and filtered non-empty
jane_list = [
    "https://www.janestreet.com/ad-cookie-policy",
    "https://blog.janestreet.com/",
    "https://blog.janestreet.com/what-the-interns-have-wrought-2023/#menu",
    "https://blog.janestreet.com/",
    "https://blog.janestreet.com/archive",
    "https://blog.janestreet.com/authors",
    "http://www.janestreet.com/",
    "https://blog.janestreet.com/what-the-interns-have-wrought-2023/#",
    "https://blog.janestreet.com/what-the-interns-have-wrought-2023/#",
    "https://blog.janestreet.com/applying-to-jane-street",
    "https://blog.janestreet.com/using-ascii-waveforms-to-test-hardware-designs/",
    "https://blog.janestreet.com/finding-memory-leaks-with-memtrace/",
    "https://blog.janestreet.com/what-the-interns-have-wrought-2023/#",
    "https://blog.janestreet.com/tag/async",
    "https://blog.janestreet.com/tag/book",
    "https://blog.janestreet.com/tag/c",
    "https://blog.janestreet.com/tag/camlp4",
    "https://blog.janestreet.com/tag/code-review",
    "https://blog.janestreet.com/tag/comments",
    "https://blog.janestreet.com/tag/compiler",
    "https://blog.janestreet.com/tag/core",
    "https://blog.janestreet.com/tag/hackerschool",
    "https://blog.janestreet.com/tag/hg",
    "https://blog.janestreet.com/tag/incremental",
    "https://blog.janestreet.com/tag/internship",
    "https://blog.janestreet.com/tag/interviewing",
    "https://blog.janestreet.com/tag/ocaml",
    "https://blog.janestreet.com/tag/parallel-programming",
    "https://blog.janestreet.com/tag/performance",
    "https://blog.janestreet.com/tag/ppx",
    "https://blog.janestreet.com/tag/real-world-ocaml",
    "https://blog.janestreet.com/tag/registers",
    "https://blog.janestreet.com/tag/speed",
    "https://blog.janestreet.com/tag/ui",
    "https://blog.janestreet.com/feed.xml",
    "https://opensource.janestreet.com/",
    "https://www.janestreet.com/technology/",
    "https://blog.janestreet.com/",
    "https://blog.janestreet.com/what-the-interns-have-wrought-2023/#search",
    "https://www.facebook.com/sharer.php?u=https://blog.janestreet.com/what-the-interns-have-wrought-2023/",
    "https://twitter.com/intent/tweet?source=https://blog.janestreet.com/what-the-interns-have-wrought-2023/&text=Jane%20Street%20Tech%20Blog%20-%20What%20the%20interns%20have%20wrought,%202023%20edition%20https://blog.janestreet.com/what-the-interns-have-wrought-2023/",
    "http://www.linkedin.com/shareArticle?mini=true&url=https://blog.janestreet.com/what-the-interns-have-wrought-2023/&title=Jane%20Street%20Tech%20Blog%20-%20What%20the%20interns%20have%20wrought,%202023%20edition&summary=We%E2%80%99re%20once%20again%20at%20the%20end%20of%20our%20internship%20season,%20and%20it%E2%80%99s%20my%20task%20to%20provide%20a%20few%20highlights%20of%20what%20the%20dev%20interns%20accomplished%20while%20they%20were%20here.%20The%20program%20was%20big!%20We%20had%20152%20software%20engineering%20interns,%20drawn%20from%2058%20schools%20across%2019%20different%20countries.%20And%20that%E2%80%99s%20not...&source=https://blog.janestreet.com/what-the-interns-have-wrought-2023/",
    "https://blog.janestreet.com/author/yminsky",
    "https://blog.janestreet.com/author/yminsky",
    "https://www.janestreet.com/tech-talks/building-an-exchange/",
    "https://signalsandthreads.com/state-machine-replication-and-why-you-should-care/",
    "https://en.wikipedia.org/wiki/Linear_temporal_logic",
    "https://github.com/inhabitedtype/angstrom",
    "https://github.com/openai/tiktoken",
    "https://blog.janestreet.com/code-review-that-isnt-boring/",
    "https://www.janestreet.com/join-jane-street/programs-and-events/internships-all-cycles/",
    "https://blog.janestreet.com/applying-to-jane-street/",
    "https://blog.janestreet.com/tag/internship",
    "https://blog.janestreet.com/oxidizing-ocaml-parallelism/",
    "https://signalsandthreads.com/",
    "https://signalsandthreads.com/a-poets-guide-to-product-management/",
    "https://www.youtube.com/watch?v=Rt3XyeFHvt4",
    "https://www.youtube.com/channel/UCDsVC_ewpcEW_AQcO-H-RDQ/videos",
    "https://blog.janestreet.com/applying-to-jane-street",
    "https://blog.janestreet.com/using-ascii-waveforms-to-test-hardware-designs/",
    "https://blog.janestreet.com/finding-memory-leaks-with-memtrace/",
    "https://blog.janestreet.com/tag/async",
    "https://blog.janestreet.com/tag/book",
    "https://blog.janestreet.com/tag/c",
    "https://blog.janestreet.com/tag/camlp4",
    "https://blog.janestreet.com/tag/code-review",
    "https://blog.janestreet.com/tag/comments",
    "https://blog.janestreet.com/tag/compiler",
    "https://blog.janestreet.com/tag/core",
    "https://blog.janestreet.com/tag/hackerschool",
    "https://blog.janestreet.com/tag/hg",
    "https://blog.janestreet.com/tag/incremental",
    "https://blog.janestreet.com/tag/internship",
    "https://blog.janestreet.com/tag/interviewing",
    "https://blog.janestreet.com/tag/ocaml",
    "https://blog.janestreet.com/tag/parallel-programming",
    "https://blog.janestreet.com/tag/performance",
    "https://blog.janestreet.com/tag/ppx",
    "https://blog.janestreet.com/tag/real-world-ocaml",
    "https://blog.janestreet.com/tag/registers",
    "https://blog.janestreet.com/tag/speed",
    "https://blog.janestreet.com/tag/ui",
    "https://blog.janestreet.com/feed.xml",
    "https://opensource.janestreet.com/",
    "https://www.janestreet.com/technology/",
    "https://www.janestreet.com/who-we-are/",
    "https://www.janestreet.com/what-we-do/overview/",
    "https://www.janestreet.com/what-we-do/client-offering/",
    "https://www.janestreet.com/the-latest/",
    "https://www.janestreet.com/culture/overview/",
    "https://www.janestreet.com/join-jane-street/overview/",
    "https://www.janestreet.com/contact-us/",
    "https://www.janestreet.com/disclosures-and-policies/",
    "https://www.finra.org/",
    "https://www.janestreet.com/privacy-policy/",
    "https://www.janestreet.com/ad-cookie-policy/"
]

print("Initial length:", (len(jane_list)))
jane_list = list(map(lambda x: x[:-1] if x[-1] == '/' else x, jane_list))

jane = set(jane_list)
print("Length of set (de-dup):", len(jane))

with open('output/jane.out') as jane_out:
    text = jane_out.read()
    text = text.split('\n')
    child_arr = [] # set of child links from 1st pg
    
    for line in text:
        child = line.split("=>")[-1]
        child = child.strip()
        if child:
            child_arr.append(child)
        
    print("Length of jane.out:", len(child_arr))
    print("Contents of jane.out:")
    for x in sorted(child_arr):
        print(x)
    print("-----")
    print("DOM API:", len(jane))
    for y in sorted(jane):
        print(y)
    
    print()
    print("Difference: what is in DOM API scrape but not in findw output?")
    print("-----")
    st = jane.difference(set(sorted(child_arr)))
    
    for link in sorted(st):
        if (link+'/') in child_arr:
            continue
        print(link)
        
        
# Diff output:
    # All correct: none have Jane in the title
# http://www.linkedin.com/shareArticle?mini=true&url=https://blog.janestreet.com/what-the-interns-have-wrought-2023/&title=Jane%20Street%20Tech%20Blog%20-%20What%20the%20interns%20have%20wrought,%202023%20edition&summary=We%E2%80%99re%20once%20again%20at%20the%20end%20of%20our%20internship%20season,%20and%20it%E2%80%99s%20my%20task%20to%20provide%20a%20few%20highlights%20of%20what%20the%20dev%20interns%20accomplished%20while%20they%20were%20here.%20The%20program%20was%20big!%20We%20had%20152%20software%20engineering%20interns,%20drawn%20from%2058%20schools%20across%2019%20different%20countries.%20And%20that%E2%80%99s%20not...&source=https://blog.janestreet.com/what-the-interns-have-wrought-2023
# https://en.wikipedia.org/wiki/Linear_temporal_logic
# https://github.com/inhabitedtype/angstrom
# https://github.com/openai/tiktoken
# https://signalsandthreads.com
# https://signalsandthreads.com/a-poets-guide-to-product-management
# https://signalsandthreads.com/state-machine-replication-and-why-you-should-care
# https://twitter.com/intent/tweet?source=https://blog.janestreet.com/what-the-interns-have-wrought-2023/&text=Jane%20Street%20Tech%20Blog%20-%20What%20the%20interns%20have%20wrought,%202023%20edition%20https://blog.janestreet.com/what-the-interns-have-wrought-2023
# https://www.facebook.com/sharer.php?u=https://blog.janestreet.com/what-the-interns-have-wrought-2023
# https://www.finra.org
# https://www.youtube.com/watch?v=Rt3XyeFHvt4
    