import re

_RX_TOKENIZE = re.compile(r"\b[a-zA-Z]+\b|[0-9]*\.?[0-9]+|<=>|=>|->|[<!>]=|[>=<]|\\/|/\\|\.\.|\+/\-|\-/\+|'+|[\+\-\*\/\^\(\)\[\]\{\}\.\!\,\;\:\_\|\\]")

def tokenize(line: str, objects: dict[str,str]) -> list[str]:
    tokens = _RX_TOKENIZE.findall(line)
    print("tokens:", tokens)
    return tokens
