import re
from lexer import tokenize
from parser import parse
from converter import convert_token


meta: dict[str,str] = {
    "author": None,
    "title": None,
}
content: list[tuple[int,str,list[str]]] = []
objects: dict[str,str] = {}


def process_line(line: str):
    # Blank
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
        content.append((depth, doc,[]))
    # Definitions
    elif line.startswith("let") or line.startswith("const") or line.startswith("fn"):
        kind, rest = line.split(' ', 1)
        if kind == "let": kind = "var"
        if kind != "fn":
            items, _ = re.split(r" (?:are|is|be) |$", rest, 1)
        else:
            items = rest
        items = [item.strip() for item in items.split(',')]
        for item in items:
            objects[item] = kind
    # Math
    else:
        print("line:", line)
        tokens = tokenize(line, objects)
        ast = parse(tokens)
        # tokens = [convert_token(token) for token in tokens]
        # print(tokens)
        # content[-1][2].append(" ".join(tokens))
