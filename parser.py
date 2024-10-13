import re
from re import Match

title = None
author = None
content: list[tuple[str|None,list[str]]] = [(None,[])]

with open("data.math", "r") as data:
    scope: list[dict[str,str]] = [{}]
    def scope_lookup(match: Match[str]):
        global scope
        name = match.group(1)
        print(f"Looking for {name}...")
        for layer in reversed(scope):
            # print(layer)
            if name in layer:
                kind = layer[name]
                print(f"Found {name}: {kind}")
                return f"{{\\{kind}{{{name}}}}}"
        print(f"{name} not found")
        return name
    for line in [line.strip() for line in data.readlines()]:
        if len(line) == 0:
            continue
        elif line == "{":
            print("push scope")
            scope.append({})
        elif line == "}":
            print("pop scope")
            scope.pop()
        elif line.startswith("//! title: "):
            title = line[len("//! title: "):]
            print(f"title: \"{title}\"")
        elif line.startswith("//! author: "):
            author = line[len("//! author: "):]
            print(f"author: \"{author}\"")
        elif line.startswith("/// "):
            doc = line[len("/// "):]
            print(f"label: \"{doc}\"")
            content.append((doc,[]))
        elif line.startswith("let") or line.startswith("const") or line.startswith("fn"):
            statement = line.split(None, 2)
            kind = statement[0]
            name = statement[1]
            if kind == "let":
                hint = statement[2]
                scope[-1][name] = "var"
                print(f"new variable \"{name}\": \"{hint}\"")
            elif kind == "const":
                hint = statement[2]
                scope[-1][name] = "const"
                print(f"new constant \"{name}\": \"{hint}\"")
            elif kind == "fn":
                name = name[:-len("()")]
                hint = "be a function"
                scope[-1][name] = "fn"
                print(f"new function \"{name}\"")
            # hint = re.sub(r"\bin\b", r" \\in ", hint).replace("Real", "\\R").replace("Integer", "\\Z").replace("Real", "\\R")
            # content[-1][1].append(f"Let {name} {hint}")
        else:
            print(line)
            line = re.sub(r"d(?:(?P<order>\^\-?\d+))?/d(?P<wr2>[a-zA-Z])(?(order)(?P=order)|)", r"\\deriv\g<order>{\g<wr2>}", line)
            line = re.sub(r"lim\[(.+?)\]", r"\\fn{\\lim_{\g<1>}}", line)
            line = re.sub(r"(?<![a-z\\])([a-zA-Z])", scope_lookup, line)
            line = line.replace("(", "{\\group(").replace(")", ")}")
            line = line.replace("[", "{\\br[{").replace("]", "}]}")
            line = line.replace("<=>", " \\stmt{\\iff} ")
            line = line.replace("=>", " \\stmt{\\implies} ")
            line = line.replace("->", " \\to ")
            line = line.replace("!=", " \\stmt{\\ne} ")
            line = line.replace("<=", " \\stmt{\\le} ")
            line = line.replace(">=", " \\stmt{\\ge} ")
            line = line.replace("<", " \\stmt{<} ")
            line = line.replace(">", " \\stmt{>} ")
            line = line.replace("=", " \\stmt{=} ")
            line = line.replace("*", " \\op{\\cdot} ")
            line = re.sub(r"\-?\d+", r"{\\lit{\g<0>}}", line)
            content[-1][1].append(line)

template = None
with open("tex/template.tex", "r") as file:
    template = file.read()

with open("tex/output.tex", "w") as tex:
    content_str = "\n".join([
        (f"\section{{{name}}}\n" if name is not None else "") + f"\\begin{{gather*}}\n{"\\\\\n".join(items)}\n\\end{{gather*}}"
        for (name, items) in content if len(items) != 0
    ])
    print(f"\ngenerated:\n{content_str}")
    tex.write(template
        .replace(r"@{TITLE}", title)
        .replace(r"@{AUTHOR}", author)
        .replace(r"@{CONTENT}", content_str)
    )
