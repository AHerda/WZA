import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

# Generowanie wartości dla θ
theta = np.linspace(0, 2*np.pi, 1000)

# Obliczanie wartości r(θ) = sin(2θ)
r = np.sin(2*theta)

# Przekształcanie współrzędnych biegunowych na kartezjańskie
x = r * np.cos(theta)
y = r * np.sin(theta)

# Narysowanie wykresu
plt.figure(figsize=(8, 8))
plt.plot(x, y, label='r(θ) = sin(2θ)')
plt.title('Krzywa czterolistna')
plt.xlabel('x')
plt.ylabel('y')
plt.axis('equal')
plt.grid(True)
plt.legend()
plt.show()

# Definicja wielomianu do dopasowania
def poly(x, a, b, c, d):
    return a*x**3 + b*x**2 + c*x + d

# Dopasowanie wielomianu do danych
popt, pcov = curve_fit(poly, x, y)
print("Współczynniki wielomianu: ", popt)
