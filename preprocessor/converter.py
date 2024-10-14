import re

_RX_OBJECT = re.compile(r"(var|const|fn)\(([a-zA-Z]+)\)")
_RX_NUMBER = re.compile(r"[0-9]+")

_MAPPINGS: dict[str,str] = {
    "pi": "\\pi",
    "varphi": "\\varphi",
    "theta": "\\theta",
    "phi": "\\phi",
    "psi": "\\psi",
    "cos": "\\cos",
    "sin": "\\sin",
    "tan": "\\tan",
    "csc": "\\csc",
    "sec": "\\sec",
    "cot": "\\cot",
    "arccos": "\\arccos",
    "arcsin": "\\arcsin",
    "arctan": "\\arctan",
    "arccsc": "\\arccsc",
    "arcsec": "\\arcsec",
    "arccot": "\\arccot",
    "cosh": "\\cosh",
    "sinh": "\\sinh",
    "tanh": "\\tanh",
    "csch": "\\csch",
    "sech": "\\sech",
    "coth": "\\coth",
    "arccosh": "\\arccosh",
    "arcsinh": "\\arcsinh",
    "arctanh": "\\arctanh",
    "arccsch": "\\arccsch",
    "arcsech": "\\arcsech",
    "arccoth": "\\arccoth",
    "sqrt": "\\sqrt",
    "log": "\\log",
    "ln": "\\ln",
    "sum": "\\sum",
    "prod": "\\prod",
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
