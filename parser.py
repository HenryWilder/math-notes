import re
from re import Match

meta: dict[str,str] = { "author": None, "title": None }
content: list[tuple[str|None,list[str]]] = [(None,[])]

def collect_content(content: list[tuple[str|None,list[str]]] = [(None,[])]):
    return "\n".join([
        (f"\section{{{name}}}\n" if name is not None else "") + f"\\begin{{gather*}}\n{"\\\\\n".join(items)}\n\\end{{gather*}}"
        for (name, items) in content if len(items) != 0
    ])


objects: dict[str,str] = {}
def name_lookup(name: Match[str]):
    name = name.group(1)
    if name in objects:
        return f"{{\\{objects[name]}{{{name}}}}}"
    else:
        return name


def tokenize(line: str) -> list[str]:
    tokens = re.findall(r"\b[a-zA-Z]+\b|[0-9]*\.?[0-9]+|<=>|=>|->|[<!>]=|[>=<]|\\/|/\\|\.\.|\+/\-|\-/\+|'+|[\+\-\*\/\^\(\)\[\]\{\}\.\!\,\;\:\_\|\\]", line)
    return tokens


def process_line(line: str):
    if len(line) == 0:
        return
    # Metadata
    elif line.startswith("@"):
        key, value = line[len("@"):].split(' ', 1)
        meta[key] = value
        print(f"{key}={value}")
    # Labels
    elif line.startswith("#"):
        depth, doc = line.split(' ', 1)
        depth = len(depth)
        print(f"{doc=} {depth=}")
        content.append((doc,[]))
    # Definitions
    elif line.startswith("let") or line.startswith("const") or line.startswith("fn"):
        kind, rest = line.split(' ', 1)
        if kind == "let": kind = "var"
        if kind != "fn":
            items, hint = re.split(r" (?:are|is|be) |$", rest, 1)
        else:
            items = rest
        items = [item.strip() for item in items.split(',')]
        for item in items:
            objects[item] = kind
    else:
        print(line)
        tokens = tokenize(line)
        print(tokens)
        # content[-1][1].append(line)


def process_file(src: str, dest: str, base: str):
    with open(src, "r") as data:
        for line in [line[:line.find('%')].strip() for line in data.readlines()]:
            process_line(line)

    template = None
    with open(base, "r") as file:
        template = file.read()

    with open(dest, "w") as tex:
        content_str = collect_content(content)
        tex.write(template
            .replace(r"@{TITLE}", meta["title"])
            .replace(r"@{AUTHOR}", meta["author"])
            .replace(r"@{CONTENT}", content_str)
        )


process_file(src="data.math", dest="tex/output.tex", base="tex/template.tex")
