use std::io;
use std::io::stdin;
use std::io::ErrorKind::Other;
use std::io::Write;
use std::process;

/// online_banking
/// author: F. Yañiquez
/// date: 05/01/2024
/// online banking system with the following features:
/* 
* Users must be able to log in with a username and password.
* If the user enters the wrong credentials three times, the system must lock them out.
* The initial balance in the bank account is $2000.
* The system must allow users to deposit, withdraw, view, and transfer money.
* The system must display a menu for users to perform transactions.2. 
*/

#[derive(Debug)]
struct Cuenta {
    nombre: String,
    password: String,
    saldo: f64,
    intentos: i32,
    activo: bool,
}

// inicializa la base de datos
fn inicializa() -> Vec<Cuenta> {
    let mut cuentas: Vec<Cuenta> = Vec::new();
    cuentas.push(Cuenta {
        nombre: String::from("U1"),
        password: String::from("P1"),
        saldo: 2000.0,
        intentos: 0,
        activo: true,
    });
    cuentas.push(Cuenta {
        nombre: String::from("U2"),
        password: String::from("P2"),
        saldo: 2000.0,
        intentos: 0,
        activo: true,
    });
    cuentas.push(Cuenta {
        nombre: String::from("U3"),
        password: String::from("P3"),
        saldo: 2000.0,
        intentos: 0,
        activo: true,
    });
    cuentas.push(Cuenta {
        nombre: String::from("U4"),
        password: String::from("P4"),
        saldo: 2000.0,
        intentos: 0,
        activo: true,
    });
    cuentas.push(Cuenta {
        nombre: String::from("U5"),
        password: String::from("P5"),
        saldo: 2000.0,
        intentos: 0,
        activo: true,
    });
    cuentas
}

fn leer_valor() -> Result<f64, io::Error> {
    let mut valor_str: String = String::new();
    print!("Monto: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut valor_str).unwrap();
    valor_str
        .trim()
        .parse::<f64>()
        .map_err(|e| io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}
fn leer_key(mensaje: &str) -> Result<String, io::Error> {
    print!("{}", mensaje);
    std::io::stdout().flush().unwrap();
    let mut key: String = String::new();
    stdin().read_line(&mut key)?;
    Ok(key.trim().to_string().to_uppercase())
}
// devuelve la cuenta destino de la transferencia y el monto a transferir o 
// en caso de err
fn trx_transferencia_datos(
    cuentas: &Vec<Cuenta>,
    idx_cuenta: usize,
) -> Result<(f64, usize), io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Transferencia");
    println!("=============");
    println!("Usuario: {}", cuentas[idx_cuenta].nombre);
    println!("============================");

    // ingresar cuenta destino
    let mut cta_destino: String = String::new();
    print!("Cuenta destino: ");
    std::io::stdout().flush().unwrap();

    stdin().read_line(&mut cta_destino)?;
    let idx_cta_destino = validar_usuario(&cuentas, &cta_destino)?;

    let monto = leer_valor()?;

    if monto > cuentas[idx_cuenta].saldo {
        let mensaje = "Saldo insuficiente";
        _ = leer_key(&mensaje);
        return Err(io::Error::new(std::io::ErrorKind::InvalidData, mensaje));
    }
    Ok((monto, idx_cta_destino))
}


// Transacciones
fn trx_deposito(cuenta: &mut Cuenta) -> Result<(), io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Depósito");
    println!("========");
    println!("Usuario: {}", cuenta.nombre);
    println!("============================");
    let deposito = leer_valor()?;
    let mensaje = format!(
        "Se depositarán {}$ a su cuenta, Esta seguro (s/n): ",
        deposito
    );
    match leer_key(&mensaje)?.as_str() {
        "S" => {
            cuenta.saldo += deposito;
            let mensaje = format!(
                "Depósito realizado, su nuevo saldo: {}, presione <Enter>",
                cuenta.saldo
            );
            _ = leer_key(&mensaje)?;
        }
        _ => _ = leer_key("Operación cancelada, presione <Enter>")?,
    }
    Ok(())
}
fn trx_retiro(cuenta: &mut Cuenta) -> Result<(), io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Retiro");
    println!("========");
    println!("Usuario: {}", cuenta.nombre);
    println!("============================");
    let retiro = leer_valor()?;
    if retiro > cuenta.saldo {
        let mensaje = "Saldo insuficiente";
        _ = leer_key(&mensaje);
        return Err(io::Error::new(std::io::ErrorKind::InvalidData, mensaje));
    }
    let mensaje = format!("Se retirarán {}$ de su cuenta, Esta seguro (s/n): ", retiro);
    match leer_key(&mensaje)?.as_str() {
        "S" => {
            cuenta.saldo -= retiro;
            let mensaje = format!(
                "Retiro realizado, su nuevo saldo: {}, presione <Enter>",
                cuenta.saldo
            );
            _ = leer_key(&mensaje)?;
        }
        _ => _ = leer_key("Operación cancelada, presione <Enter>")?,
    }
    Ok(())
}
fn trx_consulta(cuenta: &Cuenta) -> Result<(), io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Consulta");
    println!("========");
    println!("Usuario: {}", cuenta.nombre);
    println!("============================");
    println!("Saldo: {}", cuenta.saldo);
    println!("Ingresos fallidos: {}", cuenta.intentos);
    println!(
        "Estado: {}",
        match cuenta.activo {
            true => "Uctivo",
            false => "Bloqueado",
        }
    );
    _ = leer_key("Para continuar, presione <Enter>")?;
    Ok(())
}
fn trx_transferencia(cuentas: &mut Vec<Cuenta>, idx_cuenta: usize) -> Result<(), io::Error> {
    while true {
        match trx_transferencia_datos(cuentas, idx_cuenta) {
            Ok((retiro, idx_cta_destino)) => {
                let mensaje = format!(
                    "Se transferirán {}$ de su cuenta a la cuenta: {}, Esta seguro (s/n): ",
                    retiro, cuentas[idx_cta_destino].nombre
                );
                match leer_key(&mensaje)?.as_str() {
                    "S" => {
                        cuentas[idx_cuenta].saldo -= retiro;
                        cuentas[idx_cta_destino].saldo += retiro;
                        let mensaje = format!(
                            "transferencia realizada, su nuevo saldo: {}, presione <Enter>",
                            cuentas[idx_cuenta].saldo
                        );
                        _ = leer_key(&mensaje)?;
                        break;
                    }
                    _ => _ = leer_key("Operación cancelada, presione <Enter>")?,
                }
                break;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}
// si todo correcto retorna el indice del usuario en el vector,
// si el usuario existe y demasiados intentos retorna error
// si el usuario no existe retorna un error
fn validar_usuario(cuentas: &Vec<Cuenta>, usuario: &str) -> Result<usize, io::Error> {
    if usuario.trim() == "" {
        return Err(io::Error::new(Other, "Error: Introduzca usuario"));
    }
    for (i, cuenta) in cuentas.iter().enumerate() {
        if cuenta.nombre == usuario.trim() {
            if !cuenta.activo {
                return Err(io::Error::new(Other, "Error: Usuario bloqueado"));
            }
            if cuenta.intentos > 3 {
                return Err(io::Error::new(Other, "Error: Demasiados intentos"));
            }
            return Ok(i);
        }
    }
    Err(io::Error::new(
        Other,
        "Error: Usuario inexistente, intente de nuevo",
    ))
}

// si todo correcto retorna el indice del usuario en el vector,
// si el usuario existe y el password es incorrecto incrementa su contador de intentos Y error
// si el usuario existe y el contador de intentos >= 3 retorna error
// si el usuario no existe retorna un error
fn validar_contraseña(cuenta: &mut Cuenta, password: &str) -> Result<(), io::Error> {
    if password.trim() == "" {
        return Err(io::Error::new(Other, "Error: Introduzca contraseña"));
    }
    if cuenta.intentos > 2 {
        cuenta.activo = false; //user lock
        return Err(io::Error::new(
            Other,
            "Error: Demasiados intentos, usuario bloqueado",
        ));
    }
    if cuenta.password != password.trim() {
        cuenta.intentos += 1;
        return Err(io::Error::new(Other, "Error: Password incorrecto"));
    }
    Ok(())
}

// retorna el indice del usuario o el error al introducir datos
fn login(cuentas: &mut Vec<Cuenta>) -> Result<usize, io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Login");
    println!("=====");

    let mut usuario: String = String::new();
    print!("Usuario: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut usuario)?;
    let idx_usuario = validar_usuario(&cuentas, &usuario)?;

    let mut password: String = String::new();
    print!("Password: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut password)?;
    validar_contraseña(&mut cuentas[idx_usuario], &password)?;

    Ok(idx_usuario)
}

fn transacciones(cuentas: &mut Vec<Cuenta>, idx_cuenta: usize) -> Result<bool, io::Error> {
    process::Command::new("clear").status().unwrap();
    println!("Online Banking");
    println!("==============");
    println!("Usuario: {}", cuentas[idx_cuenta].nombre);
    println!("============================");
    println!("(D)epósitos");
    println!("(R)etiros");
    println!("(C)onsulta");
    println!("(T)ransferencia");
    println!("(S)alir");
    print!("Introduzca su opción: ");
    let mut opcion: String = String::new();
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut opcion)?;
    match opcion.to_uppercase().trim() {
        "D" => trx_deposito(&mut cuentas[idx_cuenta])?,
        "R" => trx_retiro(&mut cuentas[idx_cuenta])?,
        "C" => trx_consulta(&cuentas[idx_cuenta])?,
        "T" => trx_transferencia(cuentas, idx_cuenta)?,
        "S" => match leer_key("Seleccionó Salir, esta seguro? (s/n)")?.as_str() {
            "S" => return Ok(false),
            _ => (),
        },
        _ => {
            _ = leer_key("opción incorrecta, pulse <Enter>")?;
            return Ok(true);
        }
    }
    Ok(true)
}

fn main() -> Result<(), io::Error> {
    let mut cuentas: Vec<Cuenta> = inicializa();

    while true {
        let cuenta = match login(&mut cuentas) {
            Ok(idx_cuenta) => {
                while transacciones(&mut cuentas, idx_cuenta)? {}
            }
            Err(error) => println!("{:?}", error),
        };
    }
    Ok(())
}
