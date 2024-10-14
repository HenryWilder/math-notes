_DEPTH_NAMES = [
    "chapter",
    "section",
    "subsection",
    "subsubsection"
]

def collect_content(content: list[tuple[int, str, list[str]]]):
    result = ""
    for (depth, name, items) in content:
        if name is not None:
            if 1 <= depth <= 4:
                depthname = _DEPTH_NAMES[depth - 1]
            else:
                print("Error: cannot have depth greater than 4")
                exit()
            header = f"    \{depthname}{{{name}}}\n"
        else:
            header = ""

        if len(items) > 0:
            items = " \\\\\n".join(["        " + item for item in items]) + "\n"
        else:
            items = ""

        result += f"\n{header}    \\begin{{gather*}}\n{items}    \\end{{gather*}}\n"
    return result
