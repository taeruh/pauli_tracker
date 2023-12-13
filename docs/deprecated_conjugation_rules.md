# Conjugation of the Pauli group under Clifford operations

**this document is deprecated; we still keep it around for some time because it may
contain some useful stuff we forget to put into the new document**

*has to be viewed with katex*

*an improved version is in progress, but I have to wait until the university reviewed
some related work from me (otherwise, the related work is not going to pass because of
self-plagiarism ...), which is hopefully done in December*
___

This document captures all the [conjugation rules](#conjugation-of-the-pauli-gates) for
the Clifford operations, and some [other
operations](#other-operations-provided-by-the-library), provided by the library. But
first, we define some common concepts to avoid ambiguities.

In the following, $n$ denotes the number of qubits.

## Computational basis

Let's consider a subset of qubits with the indices $j_0, \ldots, j_{m-1}$, with
$j_0 \leq \ldots \leq j_{m-1}$, for some $m \in \N$ with $m \leq n$. The computational
basis in this subset of qubits is defined by
$$
  \ket{x_{j_{m-1}}}_{j_{m-1}} \ldots \ket{x_{j_0}}_{{j_0}} \eqqcolon \ket{x_{j_{m-1}}
  \ldots x_{j_0}} \eqqcolon \ket{x} \quad \text{where} \quad x = \sum_{k=0}^{m-1}
  x_{j_k} 2^{k}\,.
$$
Importantly, we keep the order of $\lbrace j_i \rbrace$ to
avoid ambiguities. In the vector representation, the computational basis is given by
$\bm{e}_x \cong \ket{x}$, where $\lbrace\bm{e}_x\rbrace$ is the standard Euler basis.
Analogously are covectors and matrices represented, i.e., for some $l, m < n$ and $A:
\mathbb{C}^{\otimes l} \to \mathbb{C}^{\otimes m}$, up to isomorphism, we define
$$
  A = \begin{pmatrix}
  \bra{0}A\ket{0} & \cdots & \bra{0}A\ket{2^l-1}\\
  \vdots & & \vdots\\
  \bra{2^m-1}A\ket{0} & \cdots & \bra{2^m-1}A\ket{2^l-1}
  \end{pmatrix}\,.
$$

### A useful tensor product rule

Let $g, l_0, \ldots, l_{m-1} \in \mathbb{N}_0$ for some $m \in \mathbb{N}$ with $g >
l_j$ for all $j = 0, \ldots, m-1$, and $A$ an operator in the subspace of qubit $g$ and
$B$ an operator in the subspace of the qubits $l_0, \ldots l_{m-1}$. Then the matrix
representation of $AB$ is given by
$$
  AB = \begin{pmatrix}A_{00}B&A_{01}B\\A_{10}B&A_{11}B\end{pmatrix}
$$
If $B$ itself is a tensor product of operators, one can use this rule recursively.

## The Pauli set

Let $P_n$ be the set of the canonical representatives of $\mathcal{P}_n/K_n$, where
$\mathcal{P}_n$ is the Pauli group and $K_n = \lbrace\pm, \pm i\rbrace \triangleleft
\mathcal{P}_n$, i.e.,
$$
P_n = \left\lbrace\bigotimes_{j = 1}^n \sigma_{\mu_j} \,\,\,\Big\vert\,\,\, \mu_j \in
{0, \ldots, 3}\right\rbrace \,,
$$
where $\sigma_0 = 1$, $\sigma_1 = X$, $\sigma_2 = Y$ and $\sigma_3 = Z$, with the Pauli
operators
$$
  X = \begin{pmatrix}0&1\\1&0\end{pmatrix} \qquad
  Y = \begin{pmatrix}0&-i\\i&0\end{pmatrix} \qquad
  Z = \begin{pmatrix}1&0\\0&-1\end{pmatrix}
  \,.
$$
The Pauli operators are unitary, hermitian, traceless and hold the equation
$$
  \sigma_j\sigma_k = \delta_{jk} + i \varepsilon_{jkl}\sigma_l
$$
for $1 <= j, k, l <= 3$.

Elements of $P_n$, or in general $\mathcal{P}_n$ are often called Pauli strings.

With the help of the [Hilbert-Schmidt inner
product](https://en.wikipedia.org/wiki/Hilbert%E2%80%93Schmidt_operator) it is easy to
see that a unitary $U$ is uniquely defined by its conjugation on $P_n$. Moreover, $U$ is
already uniquely defined its conjugation on Pauli string where only one of the Paulis
is not the identity, since a general Pauli string is just a product of those and the
conjugation is a automorphism with respect to the matrix multiplication (that's also
true for the matrix addition, but we need it for the multiplication). To simplify it
even further, we can ignore $Y$ Paulis since $Y = iXZ$.


## Conjugation of the Pauli gates

We are only interested in how $X$ and $Z$ are conjugated. Even though we don't need the
phases for the Pauli tracking, we keep them here for completeness. Proofs are at the end
of this section. 
___
### Single qubit operations
___
The Pauli gates $I$, $X = HSSH$, $Y = iHSSHSS$, $Z = SS$ (hermitian)

Rules:
$$\begin{aligned}
  IXI &= X; \qquad\,\,\,\,\,\,\, IZI = Z\\
  XXX &= X; \qquad\,\,\,\, XZX = -Z\\
  ZXZ &= -X; \qquad\, ZZZ = Z\\
  YXY &= -X; \qquad YZY = -Z
\end{aligned}$$
___
The Hadamard gate $H = (X + Z)/\sqrt{2}$ (hermitian)
$$
  H = \frac{1}{\sqrt{2}}\begin{pmatrix}1&1\\1&-1\end{pmatrix}
$$
Rules:
$$
  HXH = Z; \qquad HZH = X
$$
___
The Phase gate $S$ ($= \sqrt{Z}$)
$$
  S = \begin{pmatrix}1&0\\0&i\end{pmatrix}
$$
Rules:
$$
  S^{\dagger}XS = iZX; \qquad S^{\dagger}ZS = Z
$$
___
The Phase gate $S^{\dagger}$
$$
  S = \begin{pmatrix}1&0\\0&-i\end{pmatrix}
$$
Rules:
$$
  SXS^{\dagger} = -iZX; \qquad SZS^{\dagger} = Z
$$
___
The root X gate ($\sqrt{X} = HSH$)
$$
  \sqrt{X} = \frac{1}{2}\begin{pmatrix}1+i&1-i\\1-i&1+i\end{pmatrix}
$$
Rules:
$$
  \sqrt{X}^{\dagger}X\sqrt{X} = X; \qquad \sqrt{X}^{\dagger}Z\sqrt{X} = iXZ
$$

___
The $\sqrt{X}^{\dagger}$ gate
$$
  \sqrt{X}^{\dagger} = \frac{1}{2}\begin{pmatrix}1-i&1+i\\1+i&1-i\end{pmatrix}
$$
Rules:
$$
  \sqrt{X}X\sqrt{X}^{\dagger} = X; \qquad \sqrt{X}Z\sqrt{X}^{\dagger} = -iXZ
$$
___
The root Y gate ($\sqrt{Y} = \sqrt{i}HSS$)
$$
  \sqrt{Y} = \frac{1}{2}\begin{pmatrix}1+i&-1-i\\1+i&1+i\end{pmatrix}
$$
Rules:
$$
  \sqrt{Y}^{\dagger}X\sqrt{Y} = Z; \qquad \sqrt{Y}^{\dagger}Z\sqrt{Y} = -X
$$
___
The $\sqrt{Y}^{\dagger}$ gate
$$
  \sqrt{Y}^{\dagger} = \frac{1}{2}\begin{pmatrix}1-i&1-i\\-1+i&1-i\end{pmatrix}
$$
Rules:
$$
  \sqrt{Y}X\sqrt{Y}^{\dagger} = -Z; \qquad \sqrt{Y}Z\sqrt{Y}^{\dagger} = X
$$
___
The XY-Hadamard gate $H^{xy}$ ($= \mathrm{e}^{-i\pi/4}SHZH = (X + Y)/\sqrt{2}$) (swaps x
and y axes) (hermitian)
$$
  H^{xy} = \frac{1}{\sqrt{2}}\begin{pmatrix}0&1-i\\1+i&0\end{pmatrix}
$$
Rules:
$$
  H^{xy}XH^{xy} = iXZ; \qquad H^{xy}ZH^{xy} = -Z
$$
___
The YZ-Hadamard gate $H^{yz}$ ($= \mathrm{e}^{i\pi/4}ZHSH = (Y + Z)/\sqrt{2}$) (swaps y
and z axes) (hermitian)
$$
  H^{yz} = \frac{1}{\sqrt{2}}\begin{pmatrix}1&-i\\i&-1\end{pmatrix}
$$
Rules:
$$
  H^{yz}XH^{yz} = -X; \qquad H^{yz}ZH^{yz} = iXZ
$$
___
### Two qubit operation
___
The control $Z$ gate $\mathrm{CZ}$ (hermitian)
$$
  \mathrm{CZ}_{ab} = \begin{pmatrix}
  1&0&0&0\\
  0&1&0&0\\
  0&0&1&0\\
  0&0&0&-1
  \end{pmatrix} = \mathrm{CZ}_{ba}
$$
Rules:
$$\begin{aligned}
  \mathrm{CZ}_{ab}X_a\mathrm{CZ}_{ab} &= X_aZ_b\\
  \mathrm{CZ}_{ab}X_b\mathrm{CZ}_{ab} &= Z_aX_b\\
  \mathrm{CZ}_{ab}Z_a\mathrm{CZ}_{ab} &= Z_a\\
  \mathrm{CZ}_{ab}Z_b\mathrm{CZ}_{ab} &= Z_b
\end{aligned}$$
___
The control not gate $\mathrm{CNOT}$/$\mathrm{CX}$ ($\mathrm{CX}_{gl} = H_l
\mathrm{CZ}_{gl} H_l$) (hermitian); without loss of generality
let $g > l$ for $g, l \in \mathbb{N}_0$ (left index is control and right index is
target)
$$
  \mathrm{CX}_{g, l} = \begin{pmatrix}
  1&0&0&0\\
  0&1&0&0\\
  0&0&0&1\\
  0&0&1&0
  \end{pmatrix} \Leftrightarrow \mathrm{CX}_{l, g} = \begin{pmatrix}
  1&0&0&0\\
  0&0&0&1\\
  0&0&1&0\\
  0&1&0&0
  \end{pmatrix}
$$
Rules:
$$\begin{aligned}
  \mathrm{CX}_{ct}X_c\mathrm{CX}_{ct} &= X_cX_t\\
  \mathrm{CX}_{ct}X_t\mathrm{CX}_{ct} &= X_t\\
  \mathrm{CX}_{ct}Z_c\mathrm{CX}_{ct} &= Z_c\\
  \mathrm{CX}_{ct}Z_t\mathrm{CX}_{ct} &= Z_cZ_t
\end{aligned}$$
___
The control $Y$ gate $\mathrm{CY}$ ($\mathrm{CY}_{gl} = H^{yz}_l \mathrm{CZ}_{gl}
H^{yz}_l$) (hermitian); without loss of generality let $g > l$ for $g, l \in
\mathbb{N}_0$ (left index is control and right index is target)
$$
  \mathrm{CY}_{g, l} = \begin{pmatrix}
  1&0&0&0\\
  0&1&0&0\\
  0&0&0&-i\\
  0&0&i&0
  \end{pmatrix} \Leftrightarrow \mathrm{CY}_{l, g} = \begin{pmatrix}
  1&0&0&0\\
  0&0&0&-i\\
  0&0&1&0\\
  0&i&0&0
  \end{pmatrix}
$$
Rules:
$$\begin{aligned}
  \mathrm{CY}_{ct}X_c\mathrm{CY}_{ct} &= X_cY_t\\
  \mathrm{CY}_{ct}X_t\mathrm{CY}_{ct} &= Z_cX_t\\
  \mathrm{CY}_{ct}Z_c\mathrm{CY}_{ct} &= Z_c\\
  \mathrm{CY}_{ct}Z_t\mathrm{CY}_{ct} &= Z_cZ_t
\end{aligned}$$
___
The $\mathrm{SWAP}$ gate ($\mathrm{SWAP}_{ab} =
\mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab}$) (hermitian)
$$
  \mathrm{SWAP}_{ab} = \begin{pmatrix}
  1&0&0&0\\
  0&0&1&0\\
  0&1&0&0\\
  0&0&0&1
  \end{pmatrix} = \mathrm{SWAP}_{ba}
$$
Rules:
$$\begin{aligned}
  \mathrm{SWAP}_{ab}X_a\mathrm{SWAP}_{ab} &= X_b\\
  \mathrm{SWAP}_{ab}X_b\mathrm{SWAP}_{ab} &= X_a\\
  \mathrm{SWAP}_{ab}Z_a\mathrm{SWAP}_{ab} &= Z_b\\
  \mathrm{SWAP}_{ab}Z_b\mathrm{SWAP}_{ab} &= Z_a
\end{aligned}$$
___
The imginary $\mathrm{iSWAP}$ gate ($\mathrm{iSWAP}_{ab} =
H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b$)
$$
  \mathrm{iSWAP}_{ab} = \begin{pmatrix}
  1&0&0&0\\
  0&0&i&0\\
  0&i&0&0\\
  0&0&0&1
  \end{pmatrix} = \mathrm{SWAP}_{ba}
$$
Rules:
$$\begin{aligned}
  \mathrm{iSWAP}_{ab}X_a\mathrm{iSWAP}_{ab} &= iZ_aZ_bX_b\\
  \mathrm{iSWAP}_{ab}X_b\mathrm{iSWAP}_{ab} &= iZ_aX_aZ_b\\
  \mathrm{iSWAP}_{ab}Z_a\mathrm{iSWAP}_{ab} &= Z_b\\
  \mathrm{iSWAP}_{ab}Z_b\mathrm{iSWAP}_{ab} &= Z_a
\end{aligned}$$
___
The imginary $\mathrm{iSWAP}^{\dagger}$ gate ($\mathrm{iSWAP}_{ab}^{\dagger} =
S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b$)
$$
  \mathrm{iSWAP}_{ab}^{\dagger} = \begin{pmatrix}
  1&0&0&0\\
  0&0&-i&0\\
  0&-i&0&0\\
  0&0&0&1
  \end{pmatrix} = \mathrm{SWAP}_{ba}^{\dagger}
$$
Rules:
$$\begin{aligned}
  \mathrm{iSWAP}_{ab}X_a\mathrm{iSWAP}_{ab} &= -iZ_aZ_bX_b\\
  \mathrm{iSWAP}_{ab}X_b\mathrm{iSWAP}_{ab} &= -iZ_aX_aZ_b\\
  \mathrm{iSWAP}_{ab}Z_a\mathrm{iSWAP}_{ab} &= Z_b\\
  \mathrm{iSWAP}_{ab}Z_b\mathrm{iSWAP}_{ab} &= Z_a
\end{aligned}$$
___

### Proofs

Use [characteristics of the Paulis](#the-pauli-set) and [other useful
conjugations](#other-useful-rules). Trivial proofs, where all operators are
completely diagonal, are skipped. Without loss of generality, let $c > t$ and $a > b$.

$$\begin{aligned}
  XZX &= iXY = -Z\\
  ZXZ &= iYZ = -X\\

  2HXH &=
  \begin{pmatrix}1&1\\1&-1\end{pmatrix}
  \begin{pmatrix}0&1\\1&0\end{pmatrix}
  \begin{pmatrix}1&1\\1&-1\end{pmatrix} =
  \begin{pmatrix}1&1\\1&-1\end{pmatrix}
  \begin{pmatrix}1&-1\\1&1\end{pmatrix} =
  \begin{pmatrix}2&0\\0&2\end{pmatrix} = 2Z\\
  HZH &= HHXHH = X\\

  S^{\dagger}XS &=
  \begin{pmatrix}1&0\\0&-i\end{pmatrix}
  \begin{pmatrix}0&1\\1&0\end{pmatrix}
  \begin{pmatrix}1&0\\0&i\end{pmatrix} =
  \begin{pmatrix}1&0\\0&-i\end{pmatrix}
  \begin{pmatrix}0&i\\1&0\end{pmatrix} =
  \begin{pmatrix}0&i\\-i&0\end{pmatrix} = -Y = iZX\\

  SXS^{\dagger} &=
  ZS^{\dagger}XSZ = ZiZXZ = -iZX\\

  \sqrt{X}^{\dagger}X\sqrt{X} &=
  \sqrt{X}^{\dagger}\sqrt{X}\sqrt{X}\sqrt{X} =
  \sqrt{X}\sqrt{X} = X\\
  \sqrt{X}^{\dagger}Z\sqrt{X} &=
  HS^{\dagger}HZHSH = HS^{\dagger}XSH = iHZXH = iXZ\\

  \sqrt{X}X\sqrt{X}^{\dagger} &= \ldots = X\\
  \sqrt{X}Z\sqrt{X}^{\dagger} &= \ldots = -iXZ\\

  \sqrt{Y}^{\dagger}X\sqrt{Y} &=
  S^{\dagger}S^{\dagger}HXHSS =
  S^{\dagger}S^{\dagger}ZSS = Z\\

  \sqrt{Y}^{\dagger}Z\sqrt{Y} &=
  \left(\sqrt{Y}^{\dagger}\right)^2X\sqrt{Y}^2 = -X\\

  \sqrt{Y}X\sqrt{Y}^{\dagger} &=
  HSSXS^{\dagger}S^{\dagger}H =
  -iHSZXS^{\dagger}H =
  -HXH = -Z\\

  \sqrt{Y}Z\sqrt{Y}^{\dagger} &=
  -\left(\sqrt{Y}^{\dagger}\right)^2X\sqrt{Y}^2 = X\\

  (H^{xy})^{\dagger} X H^{xy} &=
  HZHS^{\dagger} X SHZH = iHZH ZX HZH =
  i H ZX H = iXZ = Y\\

  (H^{xy})^{\dagger} Z H^{xy} &=
  HZHS^{\dagger} Z SHZH = HZ X ZH =
  -H X H = -Z\\

  (H^{yz})^{\dagger} X H^{yz} &=
  HS^{\dagger}HZ X ZHSH = -HS^{\dagger} Z SH =
  -H Z H = - X\\

  (H^{yz})^{\dagger} Z H^{yz} &=
  HS^{\dagger}HZ Z ZHSH = HS^{\dagger} X SH =
  iH ZX H = iXZ = Y\\


  \mathrm{CZ}_{ab}X_b\mathrm{CZ}_{ab} &=
  \begin{pmatrix}1&0\\0&Z\end{pmatrix}
  \begin{pmatrix}X&0\\0&X\end{pmatrix}
  \begin{pmatrix}1&0\\0&Z\end{pmatrix} =
  \begin{pmatrix}X&0\\0&-X\end{pmatrix} = Z_aX_b\\

  \mathrm{CX}_{ct}X_t\mathrm{CX}_{ct} &=
  \begin{pmatrix}1&0\\0&X\end{pmatrix}
  \begin{pmatrix}X&0\\0&X\end{pmatrix}
  \begin{pmatrix}1&0\\0&X\end{pmatrix} =
  \begin{pmatrix}X&0\\0&X\end{pmatrix} = X_t\\

  \mathrm{CX}_{ct}X_c\mathrm{CX}_{ct} &=
  H_t^2\mathrm{CX}_{ct}H_t^2X_cH_t^2\mathrm{CX}_{ct}H_t^2 =
  H_t\mathrm{CZ}_{ct}X_c\mathrm{CZ}_{ct}H_t =
  H_tZ_tX_cH_t = X_cX_t\\

  \mathrm{CX}_{ct}Z_t\mathrm{CX}_{ct} &=
  H_t^2\mathrm{CX}_{ct}H_t^2Z_tH_t^2\mathrm{CX}_{ct}H_t^2 =
  H_t\mathrm{CZ}_{ct}X_t\mathrm{CZ}_{ct}H_t =
  H_tZ_cX_tH_t = Z_cZ_t\\

  \mathrm{CX}_{ct}Z_c\mathrm{CX}_{ct} &= \ldots = Z_c\\

  \mathrm{CY}_{ct}Z_t\mathrm{CY}_{ct} &= \ldots = Z_cZ_t\\
  \mathrm{CY}_{ct}Z_c\mathrm{CY}_{ct} &= \ldots = Z_c\\

  \mathrm{CY}_{ct}X_t\mathrm{CY}_{ct} &=
  \mathrm{CY}_{ct}iZ_tY_t\mathrm{CY}_{ct} =
  iZ_cZ_tY_t =  Z_cX_t\\

  \mathrm{CY}_{ct}X_c\mathrm{CY}_{ct} &=
  \mathrm{CY}_{ct}iZ_cY_c\mathrm{CY}_{ct} =
  iZ_cY_cY_t =  X_cY_t\\

  \mathrm{SWAP}_{ab} X_b \mathrm{SWAP}_{ab} &=
  \mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab}
  X_b
  \mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab} =
  \mathrm{CX}_{ab}\mathrm{CX}_{ba} X_b \mathrm{CX}_{ba}\mathrm{CX}_{ab}\\
  &= \mathrm{CX}_{ab} X_aX_b \mathrm{CX}_{ab} = X_a\\

  \mathrm{SWAP}_{ab} Z_a \mathrm{SWAP}_{ab} &=
  \mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab}
  Z_a
  \mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab} =
  \mathrm{CX}_{ab}\mathrm{CX}_{ba} Z_a \mathrm{CX}_{ba}\mathrm{CX}_{ab}\\
  &= \mathrm{CX}_{ab} Z_aZ_b \mathrm{CX}_{ab} = Z_b\\

  \mathrm{iSWAP}_{ab}^{\dagger} X_a \mathrm{iSWAP}_{ab} &=
  S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b
  X_a
  H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b\\
  &= S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}
  X_a
  \mathrm{CX}_{ab}H_aS_aS_b\\
  &= S_a^{\dagger}S_b^{\dagger} Z_aX_b S_aS_b = iZ_aZ_bX_b\\

  \mathrm{iSWAP}_{ab}^{\dagger} Z_a \mathrm{iSWAP}_{ab} &=
  S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b
  Z_a
  H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b\\
  &= S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}
  Z_aZ_b
  \mathrm{CX}_{ab}H_aS_aS_b\\
  &= S_a^{\dagger}S_b^{\dagger} Z_b S_aS_b = Z_b\\

  \mathrm{iSWAP}_{ab} X_a \mathrm{iSWAP}_{ab}^{\dagger} &=
  H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b
  X_a
  S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b\\
  &= -iH_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}
  Z_aX_a
  \mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b\\
  &= -iH_b\mathrm{CX}_{ba} Z_aX_aX_b \mathrm{CX}_{ba}H_b
  = -iH_b Z_aX_aX_b H_b = -iZ_aZ_bX_b\\

  \mathrm{iSWAP}_{ab} Z_a \mathrm{iSWAP}_{ab}^{\dagger} &=
  H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b
  Z_a
  S_a^{\dagger}S_b^{\dagger}H_a\mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b\\
  &= H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab} X_a \mathrm{CX}_{ab}\mathrm{CX}_{ba}H_b\\
  &= H_b\mathrm{CX}_{ba} X_aX_b \mathrm{CX}_{ba}H_b = H_b X_b H_b = Z_b\\
  

\end{aligned}$$

## Other operations provided by the library

The library also provides other operations, which are not conjugations, like the moving
Paulis from one qubit to another. We define the operation $\mathrm{move\_x\_to\_x}_{s,
d}$ to move the $X$ Pauli on the qubit $s$, if it is present as standalone $X$ or as part
of $Y \propto XZ$, to the qubit $d$. Analog we define "x to z", "z to x" and "z to z".
They are all homomorphisms, and it is probably clearer to define them as homomorphism
via
$$\begin{aligned}
\mathrm{move\_x\_to\_x}_{s, d} (X_s) &= X_d\\
\mathrm{move\_x\_to\_x}_{s, d} (Z_s) &= Z_s\\
\mathrm{move\_x\_to\_x}_{s, d} (X_d) &= X_d\\
\mathrm{move\_x\_to\_x}_{s, d} (Z_d) &= Z_d\\
\end{aligned}$$
and analog for the other variants, i.e.,
$$\begin{aligned}
\mathrm{move\_x\_to\_z}_{s, d} (X_s) &= Z_d \,,\\
\mathrm{move\_z\_to\_x}_{s, d} (Z_s) &= X_d \,,\\
\mathrm{move\_z\_to\_z}_{s, d} (Z_s) &= Z_d \,,
\end{aligned}$$
and on the other elements they are the identity.


## Other useful rules

$$\begin{aligned}
  S^{\dagger} &= SZ = ZS\\
  \sqrt{X} &= HSH\\
  \sqrt{X}^{2} &= X = HSSH\\
  \sqrt{X}^{\dagger} &= \sqrt{X}^{-1}\\
  \sqrt{Z} &= S\\
  \sqrt{Z}^{2} &= Z = SS\\
  \sqrt{Z}^{\dagger} &= \sqrt{Z}^{-1}\\
  \sqrt{Y} &= \sqrt{i}HSS \propto HZ \propto ZH\\
  \sqrt{Y}^{2} &= Y = iHSSHSS \propto HSSHSS \propto SSHSSH\\
  \sqrt{Y}^{\dagger} &= \sqrt{Y}^{-1}\\
  H^{xy} &= \mathrm{e}^{-i\pi/4}SHZH\\
  H^{yz} &= \mathrm{e}^{i\pi/4}ZHSH\\

  \mathrm{CX}_{tc} &= H_c \mathrm{CZ}_{ct} H_c\\
  \mathrm{CX}_{ct} &= H_t \mathrm{CZ}_{ct} H_t\\
  \mathrm{CY}_{ct} &= H^{yz}_t \mathrm{CZ}_{ct} H^{yz}_t\\
  \mathrm{CY}_{tc} &= H^{yz}_c \mathrm{CZ}_{ct} H^{yz}_c\\
  \mathrm{SWAP}_{ab} &= \mathrm{CX}_{ab}\mathrm{CX}_{ba}\mathrm{CX}_{ab}\\
  \mathrm{iSWAP}_{ab} &= H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b\\
\end{aligned}$$

### Proofs

Without loss of generality, let $c > t$ and $a > b$.
$$\begin{aligned}
  2HSH &=
  \begin{pmatrix}1&1\\1&-1\end{pmatrix}
  \begin{pmatrix}1&0\\0&i\end{pmatrix}
  \begin{pmatrix}1&1\\1&-1\end{pmatrix}\\
  &= \begin{pmatrix}1&1\\1&-1\end{pmatrix}
  \begin{pmatrix}1&1\\i&-i\end{pmatrix} =
  \begin{pmatrix}1+i&1-i\\1-i&1+i\end{pmatrix} = 2\sqrt{X}\\

  \sqrt{X}^{2} &= HSHHSH = HZH = X\\

  \sqrt{X}^{\dagger}\sqrt{X} &= HS^{\dagger}HHSH = 1\\

  2\sqrt{i}HZ &=
  \sqrt{2i}\begin{pmatrix}1&1\\1&-1\end{pmatrix}
  \begin{pmatrix}1&0\\0&-1\end{pmatrix} =
  \sqrt{2i}\begin{pmatrix}1&-1\\1&1\end{pmatrix} =
  \begin{pmatrix}1+i&-1-i\\1+i&1+i\end{pmatrix} = 2\sqrt{Y}\\

  \sqrt{Y}^{2} &= iHSSHSS = iXZ = Y\\

  \sqrt{Y}^{\dagger}\sqrt{Y} &= S^{\dagger}S^{\dagger}HHSS = 1\\

  SHZH &= SX = \begin{pmatrix}0&1\\i&0\end{pmatrix} = H^{xy}\sqrt{2}/(1-i)\\

  2ZHSH &=
  \begin{pmatrix}1&1\\-1&1\end{pmatrix}
  \begin{pmatrix}1&1\\i&-i\end{pmatrix} =
  \begin{pmatrix}1+i&1-i\\-1+i&-1-i\end{pmatrix} = 2H^{yz}(1+i)/\sqrt{2}\\

  H_t \mathrm{CZ}_{ct} H_t &=
  \begin{pmatrix}H&0\\0&H\end{pmatrix}
  \begin{pmatrix}1&0\\0&Z\end{pmatrix}
  \begin{pmatrix}H&0\\0&H\end{pmatrix}
  = \begin{pmatrix}1&0\\0&X\end{pmatrix} = \mathrm{CX}_{ct}\\
  H_c\mathrm{CZ}_{ct}H_c &= H_t \mathrm{CX}_{tc} H_t = \mathrm{CX}_{tc}\\
  H^{yz}_t\mathrm{CZ}_{ct}H^{yz}_t &= \ldots = \mathrm{CY}_{ct}\\
  H^{yz}_c\mathrm{CZ}_{ct}H^{yz}_c &= \ldots = \mathrm{CY}_{tc}\\

  \mathrm{CX}_{ct}\mathrm{CX}_{tc}\mathrm{CX}_{ct} &=
  \begin{pmatrix}1&0\\0&X\end{pmatrix}
  \begin{pmatrix}1&0&0&0\\0&0&0&1\\0&0&1&0\\0&1&0&0\end{pmatrix}
  \begin{pmatrix}1&0\\0&X\end{pmatrix} =
  \begin{pmatrix}1&0\\0&X\end{pmatrix}
  \begin{pmatrix}1&0&0&0\\0&0&1&0\\0&0&0&1\\0&1&0&0\end{pmatrix} = \mathrm{SWAP}_{ct}\\

  2H_b\mathrm{CX}_{ba}\mathrm{CX}_{ab}H_aS_aS_b &=
  \begin{pmatrix}1&0&0&1\\1&0&0&-1\\0&1&1&0\\0&-1&1&0\end{pmatrix}
  \begin{pmatrix}1&1\\X&-X\end{pmatrix}
  \begin{pmatrix}S&0\\0&iS\end{pmatrix}\\
  &= \begin{pmatrix}1&0&0&1\\1&0&0&-1\\0&1&1&0\\0&-1&1&0\end{pmatrix}
  \begin{pmatrix}1&0&i&0\\0&i&0&-1\\0&i&0&1\\1&0&-i&0\end{pmatrix}\\
  &= \begin{pmatrix}2&0&0&0\\0&0&2i&0\\0&2i&0&0\\0&0&0&2\end{pmatrix} = 2 \mathrm{iSWAP}

\end{aligned}$$
