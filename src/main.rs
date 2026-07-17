// Importaciones de la biblioteca estandar para manejo de archivos y E/S
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

// Constantes que definen los tamanos maximos de los arreglos estaticos
const MAX_SIMBOLOS: usize = 50;
const MAX_EXPRESION: usize = 100;

// Enumeracion que representa el tipo de un token en la expresion
#[derive(Clone, Copy, PartialEq)]
enum Tipo {
    Operando,
    Operador,
}

// Busca un caracter en la tabla de simbolos y devuelve su tipo y precedencia
fn buscar_simbolo(
    simbolos: &[char; MAX_SIMBOLOS],
    tipos: &[Tipo; MAX_SIMBOLOS],
    precedencias: &[i32; MAX_SIMBOLOS],
    cantidad: usize,
    caracter: char,
) -> Option<(Tipo, i32)> {
    for i in 0..cantidad {
        if simbolos[i] == caracter {
            return Some((tipos[i], precedencias[i]));
        }
    }
    None
}

// Convierte la expresion en una cadena a un arreglo de tokens con su informacion
fn tokenizar(
    texto: &str,
    simbolos: &[char; MAX_SIMBOLOS],
    tipos: &[Tipo; MAX_SIMBOLOS],
    precedencias: &[i32; MAX_SIMBOLOS],
    cantidad_simbolos: usize,
    tokens_simbolos: &mut [char; MAX_EXPRESION],
    tokens_tipos: &mut [Tipo; MAX_EXPRESION],
    tokens_precedencias: &mut [i32; MAX_EXPRESION],
) -> Result<usize, String> {
    let mut cantidad_tokens = 0;
    for pos in 0..texto.len() {
        let caracter_actual = texto.chars().nth(pos).unwrap();
        match buscar_simbolo(simbolos, tipos, precedencias, cantidad_simbolos, caracter_actual) {
            Some((tipo, prec)) => {
                tokens_simbolos[cantidad_tokens] = caracter_actual;
                tokens_tipos[cantidad_tokens] = tipo;
                tokens_precedencias[cantidad_tokens] = prec;
                cantidad_tokens += 1;
            }
            None => {
                return Err(format!(
                    "simbolo '{}' no definido en la tabla de simbolos",
                    caracter_actual
                ));
            }
        }
    }
    Ok(cantidad_tokens)
}

// Imprime en consola la tabla de tokens con formato alineado
fn mostrar_tabla_tokens(
    tokens_simbolos: &[char; MAX_EXPRESION],
    tokens_tipos: &[Tipo; MAX_EXPRESION],
    tokens_precedencias: &[i32; MAX_EXPRESION],
    cantidad: usize,
) {
    println!("=== Tabla de tokens ===");
    println!("{:<8} {:<12} {}", "Token", "Tipo", "Precedencia");
    for i in 0..cantidad {
        let tipo_str = if tokens_tipos[i] == Tipo::Operando {
            "OPERANDO"
        } else {
            "OPERADOR"
        };
        println!("{:<8} {:<12} {}", tokens_simbolos[i], tipo_str, tokens_precedencias[i]);
    }
    println!();
}

fn main() {
    // Obtiene el nombre del archivo desde los argumentos o usa el valor por defecto
    let argumentos: Vec<String> = std::env::args().collect();
    let nombre_archivo = if argumentos.len() > 1 {
        &argumentos[1]
    } else {
        "entrada.txt"
    };

    // Lee y parsea el archivo de entrada, maneja errores de apertura/lectura
    let (texto_expresion, tabla_simbolos, tabla_tipos, cantidad_simbolos, tabla_precedencias) =
        match leer_archivo(nombre_archivo) {
            Ok(resultado) => resultado,
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        };

    // Validaciones de la expresion antes de procesarla
    if texto_expresion.len() == 0 {
        eprintln!("Error: la expresion esta vacia");
        process::exit(1);
    }

    if texto_expresion.len() > MAX_EXPRESION {
        eprintln!("Error: expresion demasiado larga (max {})", MAX_EXPRESION);
        process::exit(1);
    }

    let texto_expresion: String = texto_expresion
        .chars()
        .filter(|&c| c != '(' && c != ')')
        .collect();

    if texto_expresion.len() == 0 {
        eprintln!("Error: la expresion esta vacia luego de eliminar los parentesis");
        process::exit(1);
    }

    // Arreglos paralelos para almacenar los tokens de la expresion
    let mut tokens_simbolos: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut tokens_tipos: [Tipo; MAX_EXPRESION] = [Tipo::Operando; MAX_EXPRESION];
    let mut tokens_precedencias: [i32; MAX_EXPRESION] = [0; MAX_EXPRESION];

    // Tokeniza la expresion usando la tabla de simbolos
    let cantidad_tokens = match tokenizar(
        &texto_expresion,
        &tabla_simbolos,
        &tabla_tipos,
        &tabla_precedencias,
        cantidad_simbolos,
        &mut tokens_simbolos,
        &mut tokens_tipos,
        &mut tokens_precedencias,
    ) {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    // Muestra la tabla de tokens en consola
    mostrar_tabla_tokens(&tokens_simbolos, &tokens_tipos, &tokens_precedencias, cantidad_tokens);

    // Muestra la expresion original en notacion infija
    print!("Expresion infija:   ");
    for i in 0..texto_expresion.len() {
        print!("{}", texto_expresion.chars().nth(i).unwrap());
    }
    println!();

    // Convierte y muestra la expresion en notacion postfija (RPN)
    let mut expresion_postfija: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let longitud_postfija = infija_a_postfija(
        &tokens_simbolos,
        &tokens_tipos,
        &tokens_precedencias,
        cantidad_tokens,
        &mut expresion_postfija,
    );

    print!("Expresion postfija: ");
    for i in 0..longitud_postfija {
        print!("{}", expresion_postfija[i]);
    }
    println!();

    // Convierte y muestra la expresion en notacion prefija
    let mut expresion_prefija: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let longitud_prefija = infija_a_prefija(
        &tokens_simbolos,
        &tokens_tipos,
        &tokens_precedencias,
        cantidad_tokens,
        &mut expresion_prefija,
    );

    print!("Expresion prefija:  ");
    for i in 0..longitud_prefija {
        print!("{}", expresion_prefija[i]);
    }
    println!();
}

// Lee y parsea el archivo de entrada. Retorna la expresion y la tabla de simbolos.
fn leer_archivo(
    nombre_archivo: &str,
) -> Result<(String, [char; MAX_SIMBOLOS], [Tipo; MAX_SIMBOLOS], usize, [i32; MAX_SIMBOLOS]), String>
{
    // Intenta abrir el archivo, retorna error si no existe
    let archivo = match File::open(nombre_archivo) {
        Ok(a) => a,
        Err(e) => {
            return Err(format!("No se pudo abrir '{}': {}", nombre_archivo, e));
        }
    };

    let lector = BufReader::new(archivo);
    let mut lineas = lector.lines();

    // Lee la primera linea que debe contener EXPRESION=
    let primera_linea = match lineas.next() {
        Some(Ok(linea)) => linea,
        Some(Err(e)) => return Err(format!("Error leyendo archivo: {}", e)),
        None => return Err("Archivo vacio".to_string()),
    };

    if !primera_linea.starts_with("EXPRESION=") {
        return Err("La primera linea debe comenzar con EXPRESION=".to_string());
    }

    // Extrae la expresion quitando el prefijo EXPRESION=
    let expresion = primera_linea["EXPRESION=".len()..].trim().to_string();

    // Arreglos paralelos para la tabla de simbolos
    let mut simbolos: [char; MAX_SIMBOLOS] = ['\0'; MAX_SIMBOLOS];
    let mut tipos: [Tipo; MAX_SIMBOLOS] = [Tipo::Operando; MAX_SIMBOLOS];
    let mut precedencias: [i32; MAX_SIMBOLOS] = [0; MAX_SIMBOLOS];
    let mut cantidad: usize = 0;

    // Procesa cada linea de la tabla de simbolos (simbolo,tipo,precedencia)
    for linea in lineas {
        let linea = match linea {
            Ok(l) => l,
            Err(e) => return Err(format!("Error leyendo archivo: {}", e)),
        };

        let linea_limpia = linea.trim();

        // Ignora lineas en blanco
        if linea_limpia.is_empty() {
            continue;
        }

        // Divide la linea en 3 campos separados por coma
        let partes: Vec<&str> = linea_limpia.split(',').collect();

        if partes.len() != 3 {
            return Err(format!("Formato invalido en linea: '{}'", linea_limpia));
        }

        if cantidad >= MAX_SIMBOLOS {
            return Err(format!("Demasiados simbolos (max {})", MAX_SIMBOLOS));
        }

        // Extrae el caracter del primer campo
        let caracter = match partes[0].trim().chars().next() {
            Some(c) => c,
            None => return Err("Simbolo vacio en la linea".to_string()),
        };

        // Determina el tipo: OPERANDO u OPERADOR
        let tipo = match partes[1].trim() {
            "OPERANDO" => Tipo::Operando,
            "OPERADOR" => Tipo::Operador,
            _ => return Err(format!("Tipo desconocido '{}' en linea '{}'", partes[1].trim(), linea_limpia)),
        };

        // Parsea la precedencia como entero
        let precedencia: i32 = match partes[2].trim().parse() {
            Ok(p) => p,
            Err(_) => return Err(format!("Precedencia invalida en linea '{}'", linea_limpia)),
        };

        // Almacena el simbolo en la tabla
        simbolos[cantidad] = caracter;
        tipos[cantidad] = tipo;
        precedencias[cantidad] = precedencia;
        cantidad += 1;
    }

    Ok((expresion, simbolos, tipos, cantidad, precedencias))
}

// Algoritmo Shunting Yard: convierte de notacion infija a postfija (RPN)
fn infija_a_postfija(
    simbolos: &[char; MAX_EXPRESION],
    tipos: &[Tipo; MAX_EXPRESION],
    precedencias: &[i32; MAX_EXPRESION],
    cantidad: usize,
    salida: &mut [char; MAX_EXPRESION],
) -> usize {
    // Pila de operadores con sus precedencias (arreglos paralelos)
    let mut pila_simbolos: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut pila_precedencias: [i32; MAX_EXPRESION] = [0; MAX_EXPRESION];
    let mut tope: usize = 0;
    let mut indice_salida: usize = 0;

    // Recorre los tokens de izquierda a derecha
    for i in 0..cantidad {
        if tipos[i] == Tipo::Operando {
            // Los operandos van directamente a la salida
            salida[indice_salida] = simbolos[i];
            indice_salida += 1;
        } else {
            // Los operadores: saca de la pila mientras tengan mayor o igual precedencia
            while tope > 0 && pila_precedencias[tope - 1] >= precedencias[i] {
                salida[indice_salida] = pila_simbolos[tope - 1];
                indice_salida += 1;
                tope -= 1;
            }
            // Luego mete el operador actual en la pila
            pila_simbolos[tope] = simbolos[i];
            pila_precedencias[tope] = precedencias[i];
            tope += 1;
        }
    }

    // Saca todos los operadores restantes de la pila a la salida
    while tope > 0 {
        salida[indice_salida] = pila_simbolos[tope - 1];
        indice_salida += 1;
        tope -= 1;
    }

    indice_salida
}

// Convierte de notacion infija a prefija invirtiendo los tokens y aplicando Shunting Yard
fn infija_a_prefija(
    simbolos: &[char; MAX_EXPRESION],
    tipos: &[Tipo; MAX_EXPRESION],
    precedencias: &[i32; MAX_EXPRESION],
    cantidad: usize,
    salida: &mut [char; MAX_EXPRESION],
) -> usize {
    // Pila de operadores con sus precedencias (arreglos paralelos)
    let mut pila_simbolos: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut pila_precedencias: [i32; MAX_EXPRESION] = [0; MAX_EXPRESION];
    let mut tope: usize = 0;
    let mut indice_salida: usize = 0;

    // Recorre los tokens de derecha a izquierda (inverso)
    let mut i = cantidad;
    while i > 0 {
        i -= 1;

        if tipos[i] == Tipo::Operando {
            salida[indice_salida] = simbolos[i];
            indice_salida += 1;
        } else {
            // Comparacion estricta (>) en lugar de (>=) usada en postfija
            while tope > 0 && pila_precedencias[tope - 1] > precedencias[i] {
                salida[indice_salida] = pila_simbolos[tope - 1];
                indice_salida += 1;
                tope -= 1;
            }

            pila_simbolos[tope] = simbolos[i];
            pila_precedencias[tope] = precedencias[i];
            tope += 1;
        }
    }

    // Saca los operadores restantes
    while tope > 0 {
        salida[indice_salida] = pila_simbolos[tope - 1];
        indice_salida += 1;
        tope -= 1;
    }

    // Invierte el resultado para obtener la notacion prefija
    salida[..indice_salida].reverse();
    indice_salida
}
