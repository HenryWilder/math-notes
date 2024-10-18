# AmyMath Preprocessor

Preprocessor for AmyMath (`.math`) files.
Converts basic math expressions into structured $\LaTeX$ code.

**Note:**
Use Rust version (`./amymath_preprocessor/`) instead of Python version (`./preprocessor/`).
The Python version is being depricated due to the lack of static types making refactoring and maintentance difficult.
(e.g. changing order of tuple items not being reflected nor warned of in consumers of the tuple)

## Example

### Source AmyMath
```tex
let x, y, z be in Real
let n, m be in Integer

## Foo

% Note how the preprocessor converts a binary "a/b" operator into a `\frac{a}{b}` LaTeX command.
log[n](x) / log[n](m) = log[m](x)
x = y /\ y = z => x = z

% Note how each layer of the parentheses changes color in the output.
% This is actually done entirely within LaTeX itself using a counter, the preprocessor just applies the `\br` command.
(9*(9*(9*(9*(9)))))

## Bar
x / x = 1 where x != 0

const z is in Real
% Note how "z" is now defined as a constant because of the line above, and how that is reflected in the generated LaTeX.
(z*x)' = z*x'

% Note that "theta" is recognized as a symbol and converted to the LaTeX `\theta`.
% It is not highlighted as a variable here though, as it has not been defined at this point.
e ^ theta!  % (btw: factorial has greater precedence than exponentiation)

% Note also that "e" is highlighted as a numeric literal, rather than a "constant".
% This is also true of other mathematical constants, such as `\pi` and `\varphi`.
% (Regular `\phi` is left free for the user to define as a variable, while `\varphi` (or "gold") represents the golden ratio.)
let phi, theta, psi be in Real
e pi tau varphi gold phi theta psi
```

### Generated $\LaTeX$
```tex
% [template code]
    \section{Foo}
    \begin{gather*}
        \op{\frac{\fn{\log_{{\var{n}}}}{\br({{\var{x}}})}}{\fn{\log_{{\var{n}}}}{\br({{\var{m}}})}}} \stmt{=} \fn{\log_{{\var{m}}}}{\br({{\var{x}}})} \\
        {\var{x}} \stmt{=} {\var{y}} \stmt{\bigwedge} {\var{y}} \stmt{=} {\var{z}} \stmt{\implies} {\var{x}} \stmt{=} {\var{z}} \\
        {\br({{\lit{9}}\op{\cdot}{\br({{\lit{9}}\op{\cdot}{\br({{\lit{9}}\op{\cdot}{\br({{\lit{9}}\op{\cdot}{\br({{\lit{9}}})}})}})}})}})}
    \end{gather*}
    \section{Bar}
    \begin{gather*}
        \op{\frac{{\var{x}}}{{\var{x}}}} \stmt{=} {\lit{1}} \stmt{\where} {\var{x}} \stmt{\ne} {\lit{0}} \\
        {\br({{\const{z}}\op{\cdot}{\var{x}}})}^{\op{\prime}} \stmt{=} {\const{z}}\op{\cdot}{\var{x}}^{\op{\prime}} \\
        {\lit{e}} ^ {\theta\op{!}} \\
        {\lit{e}} {\lit{\pi}} {\lit{\tau}} {\lit{\varphi}} {\lit{\varphi}} {\var{\phi}} {\var{\theta}} {\var{\psi}}
    \end{gather*}
% [template code]
```

### How this might look

$\Large\textbf{0.1 ~~ Foo}$

$$
{\color{569cd6}
    \frac{
        {\color{dcdcaa}{
            \log_{
                {\color{9cdcfe}{n}}
            }
            {\color{ffd700}{(
                {\color{9cdcfe}{x}}
            )}}
        }}
    }{
        {\color{dcdcaa}{
            \log_{
                {\color{9cdcfe}{n}}
            }
            {\color{ffd700}{(
                {\color{9cdcfe}{m}}
            )}}
        }}
    }
}
\\;{\color{c586c0}{=}}\\;
{\color{dcdcaa}{
    \log_{
        {\color{9cdcfe}{m}}
    }
    {\color{ffd700}{(
        {\color{9cdcfe}{x}}
    )}}
}}
$$

$$
{\color{9cdcfe}{x}}
\\;{\color{c586c0}{=}}\\;
{\color{9cdcfe}{y}}
\\;{\color{c586c0}{\bigwedge}}\\;
{\color{9cdcfe}{y}}
\\;{\color{c586c0}{=}}\\;
{\color{9cdcfe}{z}}
\\;{\color{c586c0}{\implies}}\\;
{\color{9cdcfe}{x}}
\\;{\color{c586c0}{=}}\\;
{\color{9cdcfe}{z}}
$$

$$
{\color{ffd700}{(
    {\color{b5cea8}{9}}
    \\:{\color{569cd6}\cdot}\\:
    {\color{da70d6}{(
        {\color{b5cea8}{9}}
        \\:{\color{569cd6}\cdot}\\:
        {\color{179fff}{(
            {\color{b5cea8}{9}}
            \\:{\color{569cd6}\cdot}\\:
            {\color{ffd700}{(
                {\color{b5cea8}{9}}
                \\:{\color{569cd6}\cdot}\\:
                {\color{da70d6}{(
                    {\color{b5cea8}{9}}
                )}}
            )}}
        )}}
    )}}
)}}
$$

$\Large\textbf{0.2 ~~ Bar}$

$$
{\color{569cd6}
    \frac{
        {\color{9cdcfe}{x}}
    }{
        {\color{9cdcfe}{x}}
    }
}
\\;{\color{c586c0}{=}}\\;
{\color{b5cea8}{1}}
\\;{\color{c586c0}{\textrm{where}}}\\;
{\color{9cdcfe}{x}}
\\;{\color{c586c0}{\ne}}\\;
{\color{b5cea8}{0}}
$$

$$
{\color{ffd700}{(
    {\color{4fc1ff}{z}}
    \\:{\color{569cd6}\cdot}\\:
    {\color{9cdcfe}{x}}
)}}^{
    {\color{569cd6}\prime}
}
\\;{\color{c586c0}{=}}\\;
{\color{4fc1ff}{z}}
\\:{\color{569cd6}\cdot}\\:
{
    {\color{9cdcfe}{x}}
}^{
    {\color{569cd6}\prime}
}
$$

$$
{
    {\color{b5cea8}{e}}
}^{
    \theta
    {\color{569cd6}!}
}
$$

$$
{\color{b5cea8}{e}}
{\color{b5cea8}{\pi}}
{\color{b5cea8}{\tau}}
{\color{b5cea8}{\varphi}}
{\color{b5cea8}{\varphi}}
{\color{9cdcfe}{\phi}}
{\color{9cdcfe}{\theta}}
{\color{9cdcfe}{\psi}}
$$

\* **Disclaimer:** This code was processed by hand. The tool is not yet complete enough to produce this output, though it is the goal.
