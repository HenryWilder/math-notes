import re

_RX_OBJECT = re.compile(r"(var|const|fn)\(([a-zA-Z]+)\)")
_RX_NUMBER = re.compile(r"[0-9]+")
_RX_TRIG = re.compile(r"(?:arc)?(?:sin|cos|tan|csc|sec|cot)h?")

_COMMANDS: set[str] = {
    "pi",
    "varphi",
    "theta",
    "phi",
    "psi",
    "sqrt",
    "log",
    "ln",
    "sum",
    "prod",
}

_MAPPINGS: dict[str,str] = {
    "<=>": "\\iff",
    "=>": "\\implies",
    "<=": "\\le",
    ">=": "\\ge",
    "!=": "\\ne",
    "+/-": "\\pm",
    "-/+": "\\mp",
    "*": "\\cdot",
    "(": "{\\group(",
    ")": ")}",
    "'": "^{\\prime}",
    "\\/": "\\lor",
    "/\\": "\\land",
    "where": "\\mathrel{\\textrm{where}}",
}

_LITERALS = {
    "e"
    "pi",
    "varphi",
}

_RELATIONSHIPS: set[str] = {
    "<=>",
    "=>",
    "<=",
    ">=",
    "!=",
    "'",
    "where",
}

_OPERATORS: set[str] = {
    "sqrt",
    "sum",
    "prod",
    "+/-",
    "-/+",
    "\\/",
    "/\\",
}

_FUNCTIONS: set[str] = {
    "cos",
    "sin",
    "tan",
    "csc",
    "sec",
    "cot",
    "log",
    "ln",
    "'",
}

def convert_token(token: str) -> str:
    if (match := _RX_OBJECT.match(token)) is not None:
        kind = match.group(1)
        name = convert_token(match.group(2))
        return f"{{\\{kind}{{{name}}}}}"
    else:
        tkn_is_number = _RX_NUMBER.match(token)
        if tkn_is_number:
            mapped_token = token
        elif token in _MAPPINGS:
            mapped_token = _MAPPINGS[token]
        else:
            mapped_token = token

        if tkn_is_number:
            return f"{{\\lit{{{mapped_token}}}}}"
        if token in _LITERALS:
            return f"{{\\lit{{{mapped_token}}}}}"
        elif token in _RELATIONSHIPS:
            return f"{{\\stmt{{{mapped_token}}}}}"
        elif token in _OPERATORS:
            return f"{{\\op{{{mapped_token}}}}}"
        elif token in _FUNCTIONS:
            return f"{{\\fn{{{mapped_token}}}}}"
        return mapped_token
