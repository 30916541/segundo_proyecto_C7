# Conversor de Notaciones

**Universidad** — Proyecto de Lenguajes de Programación

Conversor de expresiones matemáticas entre notación infija, postfija (RPN) y prefija, implementado en Rust.

## Descripción

El programa lee un archivo de texto con una expresión y una tabla de símbolos, tokeniza la expresión, y genera la tabla de tokens junto con las conversiones a notación postfija y prefija utilizando el algoritmo **Shunting Yard**.

## Formato de entrada

```
EXPRESION=<expresion>
<simbolo>,<tipo>,<precedencia>
```

Ejemplo:
```
EXPRESION=A+B*C
A,OPERANDO,0
B,OPERANDO,0
C,OPERANDO,0
+,OPERADOR,1
*,OPERADOR,2
```

## Uso

```bash
cargo run [archivo_entrada]
```

Si no se especifica un archivo, usa `entrada.txt` por defecto.

## Ejemplo de salida

```
=== Tabla de tokens ===
Token    Tipo          Precedencia
A        OPERANDO      0
+        OPERADOR      1
B        OPERANDO      0
*        OPERADOR      2
C        OPERANDO      0

Expresion infija:   A+B*C
Expresion postfija: ABC*+
Expresion prefija:  +A*BC
```

## Requisitos

- Rust (edition 2024)
