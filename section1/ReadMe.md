# Zadanie 1
**Liczbami Gaussa nazywamy pierścień `Z[i] = {a+bı : a, b ∈ Z}` z dodawaniem i dzieleniem odziedziczonym z liczb zespolonych. Na Z[i] określamy funkcję (zwaną normą) `N(a+bı) = a^2+b^2`. Dla `x, y ∈ Z[i]` określamy `x|y ←→ (∃z ∈ Z[i])(y = x · z)`**

1. Napisz algorytm dzielenia z resztą w pierścieniu liczb Gaussa Z[i], czyli algorytm, który dla danych
`x, y ∈ Z[i], y ̸= 0` wyznaczy q, r ∈ Z[i] takie, że `x = q · y + r` oraz `N(r) < N(y)`.
2. Największym wspólnym dzielnikiem liczb `u, v ∈ Z[i]` nazywamy takie `d ∈ Z[i]`, że `(d|u)∧(d|v)` oraz
`(∀x ∈ Z[i])(x|u ∧ x|v → x|d)`.
Oprogramuj algorytm wyznaczania NWD dla Z[i].
3. Oprogramuj funkcję wyznaczającą NWW(x, y) dla x, y ∈ Z[i].
4. Ideał generowany przez liczby a1, . . . , ak oznaczamy przez (a1, . . . , ak). Korzystając z poprzednich
dwóch podpunktów znajdź takie c i d, że `(3 + 4ı, 1 + 3ı) = (c)` oraz `(3 + 4ı) ∩ (1 + 3ı) = (d)`.

# Zadnie 2
**Wielomian `a_0 + a_1 · x + . . . + a_nx^n ∈ R[x]` interpretujemy jako ciąg `[a_0, . . . , a_n]`.**
1. Napisz algorytm dzielenia z resztą w pierścieniu wielomianów R[x].
2. Oprogramuj algorytm wyznaczania NWD dla R[x].
3. Oprogramuj funkcję wyznaczającą NWW(x, y) dla `x, y ∈ R[x]`.
4. Korzystając z poprzednich dwóch podpunktów znajdź takie `c(x), d(x) ∈ R[x]`, że `(1 + x^2, 1 + 2x + x^2) = (c)` oraz `(1 + x^2) ∩ (1 + 2x + x^2) = (d)`.

# Zadnie 3
**Skorzystaj z jakiejś biblioteki (np. mplot3d z Matplotlib) do wyświetlenia następujących rozmaitości algebraicznych w R^3:**
1. ```V(z − x^2 − y^2)```
2. ```V(z^2 − x^2 − y^2)```
3. ```V(z − x^2 + y^2)```
4. ```V(xz, yz)```

# Zadanie 4
**Krzywą czterolistną nazywaną krzywą zadaną następującym równaniem `r(θ) = sin(2θ)` we współrzędnych biegunowych.**
1. Narysuj wykres tej krzywej na płaszczyźnie.
2. Spróbuj znaleźć wielomian w(x, y) taki, `że r[R] = V (w)`.

# Zadanie 5
**Zapoznaj się poleceniami systemu Wolfram Alpha służącymi go działań na wielomianach (np. PolynomialQuotientRemainder) oraz generowania krzywych i powierzchni zadawanych równaniami parametrycznymi.**
