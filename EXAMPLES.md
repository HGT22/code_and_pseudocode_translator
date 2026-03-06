# Ejemplos de Traducción

Este documento contiene ejemplos de código que puedes usar para probar el traductor.

## Ejemplo 1: Factorial en Python → C

### Entrada (Python)
```python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

result = factorial(5)
print(result)
```

### Salida esperada (C)
```c
#include <stdio.h>
#include <stdlib.h>

int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

int result = factorial(5);
printf("%d\n", result);
```

---

## Ejemplo 2: Suma de Array en JavaScript → Python

### Entrada (JavaScript)
```javascript
function sumArray(arr) {
    let sum = 0;
    for (let i = 0; i < arr.length; i++) {
        sum = sum + arr[i];
    }
    return sum;
}

let numbers = [1, 2, 3, 4, 5];
let total = sumArray(numbers);
```

### Salida esperada (Python)
```python
def sumArray(arr):
    sum = 0
    for i in arr:
        sum = sum + i
    return sum

numbers = [1, 2, 3, 4, 5]
total = sumArray(numbers)
```

---

## Ejemplo 3: Algoritmo Burbuja en C → Pseudocode

### Entrada (C)
```c
void bubbleSort(int arr[], int n) {
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}
```

### Salida esperada (Pseudocode)
```
FUNCIÓN bubbleSort(arr, n)
    PARA i EN n - 1 HACER
        PARA j EN n - i - 1 HACER
            SI arr[j] > arr[j + 1] ENTONCES
                temp ← arr[j]
                arr[j] ← arr[j + 1]
                arr[j + 1] ← temp
            FIN SI
        FIN PARA
    FIN PARA
FIN FUNCIÓN
```

---

## Ejemplo 4: Validación de Número en Pseudocode → JavaScript

### Entrada (Pseudocode)
```
FUNCIÓN esParE(n)
    SI n % 2 == 0 ENTONCES
        retorna verdadero
    SINO
        retorna falso
    FIN SI
FIN FUNCIÓN
```

### Salida esperada (JavaScript)
```javascript
function esParE(n) {
    if (n % 2 == 0) {
        return true;
    } else {
        return false;
    }
}
```

---

## Cómo usar estos ejemplos

1. Copia el código de entrada
2. Ejecuta el traductor con `cargo run`
3. Selecciona "Traducir código"
4. Elige el lenguaje de entrada
5. Elige el lenguaje de salida
6. Pega el código
7. Escribe "FIN" cuando termines
8. Verifica el resultado con la salida esperada

---

**Nota**: Los resultados pueden variar ligeramente dependiendo de la completitud del traductor y las optimizaciones específicas del lenguaje.
