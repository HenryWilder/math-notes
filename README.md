# AmyMath Preprocessor

Preprocessor for AmyMath (`.math`) files.
Converts basic math expressions into structured $\LaTeX$ code.

**Note:**
Use Rust version (`./amymath_preprocessor/`) instead of Python version (`./preprocessor/`).
The Python version is being depricated due to the lack of static types making refactoring and maintentance difficult. (e.g. )

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
\def\brA#1{{\color{ffd700}#1}}
\def\brB#1{{\color{da70d6}#1}}
\def\brC#1{{\color{179fff}#1}}
\def\lit#1{{\color{b5cea8}#1}}
\def\op#1{\mathbin{\color{569cd6}#1}}
\def\stmt#1{\mathrel{\color{c586c0}#1}}
\def\var#1{{\color{9cdcfe}#1}}
\def\const#1{{\color{4fc1ff}#1}}
\def\fn#1{{\color{dcdcaa}#1}}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\begin{gathered}
    \op{\frac{\fn{\log_{\var{n}}\brA{(\var{x})}}}{\fn{\log_{\var{n}}\brA{(\var{m})}}}} \stmt{=} \fn{\log_{\var{m}}\brA{(\var{x})}} \\[3ex]
    \var{x} \stmt{=} \var{y} \stmt{\bigwedge} \var{y} \stmt{=} \var{z} \stmt{\implies} \var{x} \stmt{=} \var{z} \\[3ex]
    \brA{(\lit{9}\op{\cdot}\brB{(\lit{9}\op{\cdot}\brC{(\lit{9}\op{\cdot}\brA{(\lit{9}\op{\cdot}\brB{(\lit{9})})})})})}
\end{gathered}
$$
$\Large\textbf{0.2 ~~ Bar}$
$$
\def\brA#1{{\color{ffd700}#1}}
\def\brB#1{{\color{da70d6}#1}}
\def\brC#1{{\color{179fff}#1}}
\def\lit#1{{\color{b5cea8}#1}}
\def\op#1{\mathbin{\color{569cd6}#1}}
\def\stmt#1{\mathrel{\color{c586c0}#1}}
\def\var#1{{\color{9cdcfe}#1}}
\def\const#1{{\color{4fc1ff}#1}}
\def\fn#1{{\color{dcdcaa}#1}}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\begin{gathered}
    \op{\frac{\var{x}}{\var{x}}} \stmt{=} \lit{1} \stmt{\textrm{where}} \var{x} \stmt{\ne} \lit{0} \\[3ex]
    \brA{(\const{z} \op{\cdot} \var{x})}^{\op{\prime}} \stmt{=} \const{z} \op{\cdot} {\var{x}}^{\op{\prime}} \\[3ex]
    {\lit{e}}^{\theta\op{!}} \\[3ex]
    \lit{e} \lit{\pi} \lit{\tau} \lit{\varphi} \lit{\varphi} \var{\phi} \var{\theta} \var{\psi}
\end{gathered}
$$

\* **Disclaimer:** This code was processed by hand. The tool is not yet complete enough to produce this output, though it is the goal.
