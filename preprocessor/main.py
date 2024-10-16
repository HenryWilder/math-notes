from process import process_line, meta, content
from combiner import collect_content


def process_file(src: str, dest: str, base: str):
    with open(src, "r") as data:
        for line in [line[:line.find('%')].strip() for line in data.readlines()]:
            process_line(line)
            print()

    template = None
    with open(base, "r") as file:
        template = file.read()
    content_str = collect_content(content)
    with open(dest, "w") as tex:
        tex.write(template
            .replace(r"@{TITLE}", meta["title"])
            .replace(r"@{AUTHOR}", meta["author"])
            .replace(r"@{CONTENT}", content_str)
        )


process_file(src="data.math", dest="tex/output.tex", base="tex/template.tex")
