_BINDING_POWER = [
    { "'", },
    { "(", ")", "[", "]", "{", "}" },
    { "!", },
    { "^", },
    { "*", "/", },
    { "+", "-", "+/-", "-/+", },
    { "/\\", },
    { "\\/", },
    { "->", "<-", "<->", "|=>", "<=|", },
    { ">", ">=", "<", "<=", },
    { "=", "==", "===", "!=", "=/=", },
    { ",", },
    { "=>", "==>", "<==", "<=>", "=/=>", "<=/=", "<=/=>", },
    { "so", "then" },
    { "where", "if", },
]


def _binding_power(token: str) -> int:
    for i in range(len(_BINDING_POWER)):
        precedence = _BINDING_POWER[i]
        if token in precedence:
            return len(_BINDING_POWER) - i
    return 0

def _group(tokens: list[tuple[str, int]]) -> list[tuple[str, int]|list]:
    result = []
    i = 0
    for i in range(len(tokens)):
        token_text, _ = tokens[i]
        if tokens[i] in { "(", "[", "{" }:
            grouped, n = _group(tokens[i + 1:])
            result.append([tokens[i], *grouped])
            i += n
        else:
            result.append(tokens[i])

        if tokens[i] in { ")", "]", "}" }:
            break

    return (result, i)

def _bind(grouped_tokens: list[tuple[str, int]|list]) -> list:
    for i in range(len(grouped_tokens)):
        if isinstance(grouped_tokens[i], list):
            grouped_tokens[i] = _bind(grouped_tokens[i])

    while max([item[1] for item in grouped_tokens if isinstance(item, tuple[str, int])]) > 0:
        highest_power: int
        highest_power_index: int
        for i in range(len(grouped_tokens)):
            if isinstance(grouped_tokens[i], tuple[str, int]):
                token = grouped_tokens[i]
                highest_power_index = i

    return grouped_tokens


def parse(tokens: list[str]):
    grouped_tokens, _ = _group([(token, _binding_power(token)) for token in tokens])
    print("grouped tokens:", grouped_tokens)
    bound_tokens = _bind(grouped_tokens)
    print("bound tokens:", bound_tokens)
    return bound_tokens
    # ast = []
    # binding_power = [_binding_power(token) for token in tokens]
    # return ast
