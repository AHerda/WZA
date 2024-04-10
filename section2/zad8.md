Funkcja NWD_Wielomianów(f, g):
	f2 := f
	g2 := g
    Jeżeli stopień(g) > stopień(f):
        Zamień(f2, g2)

    dopóki g2 ≠ 0:
        r := f2 % g2
        f2 := g2
        g2 := r

	nwd := f2
    Jeżeli f ≠ 0:
        A := f / nwd
        B := g / nwd
    W przeciwnym razie:
        A := 0
        B := 0

    Zwróć A, B
