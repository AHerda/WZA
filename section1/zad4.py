import numpy as np
import matplotlib.pyplot as plt

# Definicja funkcji r(θ)
def r(theta):
    return np.sin(2 * theta)

def w(x, y):
    return (x**2 + y**2)**3 - 4 * x**2 * y**2

# Tworzenie wartości kąta od 0 do 2*pi
theta_values = np.linspace(0, 2 * np.pi, 1000)

# Obliczenie wartości promienia r dla każdego kąta
r_values = r(theta_values)

# Konwersja współrzędnych biegunowych na kartezjańskie
x_values = r_values * np.cos(theta_values)
y_values = r_values * np.sin(theta_values)

# Narysowanie wykresu
plt.figure(0, figsize=(8, 8))
plt.plot(x_values, y_values, label='r(θ) = sin(2θ)')
plt.title('Krzywa czterolistna w układzie współrzędnych kartezjańskich')
plt.xlabel('x')
plt.ylabel('y')
plt.axis('equal')
plt.grid(True)
plt.legend()
plt.savefig('section1/wykresy/zad4_1.png', dpi=400)

# Define your function here
def f(x, y):
    # Example equation: x^2 + y^2 - 1
    equation = w(x, y)
    epsilon = 0.001
    return np.abs(equation) < epsilon

# Generate x and y values
x_values = np.linspace(-1, 1, 1000)
y_values = np.linspace(-1, 1, 1000)

# Create a grid of points
x, y = np.meshgrid(x_values, y_values)

# Apply the function to the grid
mask = f(x, y)

# Plot the points where the function is close to 0
plt.figure(1, figsize=(8, 8))
plt.scatter(x[mask], y[mask], s=1)
plt.title('Points where the function is close to 0')
plt.xlabel('x')
plt.ylabel('y')
plt.axis('equal')
plt.grid(True)
plt.savefig('section1/wykresy/zad4_2.png', dpi=400)
