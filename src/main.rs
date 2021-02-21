#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::{self, ops::Index};
use substring::Substring;
#[derive(Serialize, Deserialize)]
struct Numero {
    numero: String,
}

#[get("/Hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/convertirNumero", format = "application/json", data = "<numero>")]
fn new(numero: Json<Numero>) -> JsonValue {
    let mut num: String = numero.0.numero;

    json!({ "El número es": convertirNumeroATexto(&mut num) })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::ignite().mount("/", routes![new]).launch();
}

fn convertirNumeroATexto(numero: &mut str) -> String {
    let mut numeroFinalE: String = "".to_string();
    let mut numeroFinalD: String = "".to_string();
    let mut numConvertir: String = numero.to_string();

    if numero.contains(",") {
        numConvertir = numConvertir.replace(",", ".");
    }

    let re = Regex::new(r"^\d{1,120}(\.\d{1,24})?$").unwrap();

    if (re.is_match(&numConvertir)) {
        let mut entero: &str = "";
        let mut decimal: &str = "";
        let mut enteroF: f64 = 0.0;
        let mut decimalF: f64 = 0.0;

        if numConvertir.contains(".") {
            let token: Vec<&str> = numConvertir.split(".").collect();

            entero = &token[0];
            decimal = &token[1];

            enteroF = entero.parse::<f64>().unwrap();
            decimalF = decimal.parse::<f64>().unwrap();

            if decimalF >= 1e18 {
                numeroFinalD = getTrillonEnAdelante(decimal);
            } else if decimalF >= 1e15 {
                numeroFinalD = getMilBillones(decimal);
            } else if decimalF >= 1e12 {
                numeroFinalD = getBillon(decimal);
            } else if decimalF >= 1e9 {
                numeroFinalD = getMilMillones(decimal);
            } else if decimalF >= 1e6 {
                numeroFinalD = getMillones(decimal);
            } else {
                numeroFinalD = getMiles(decimal);
            }
        } else {
            entero = &numConvertir;
            enteroF = entero.parse::<f64>().unwrap();
        }

        if enteroF >= 1e18 {
            numeroFinalE = getTrillonEnAdelante(entero);
        } else if enteroF >= 1e15 {
            numeroFinalE = getMilBillones(entero);
        } else if enteroF >= 1e12 {
            numeroFinalE = getBillon(entero);
        } else if enteroF >= 1e9 {
            numeroFinalE = getMilMillones(entero);
        } else if enteroF >= 1e6 {
            numeroFinalE = getMillones(entero);
        } else {
            numeroFinalE = getMiles(entero);
        }
    } else {
        return "No se puede convertir ese numero no tiene el formato adecuado".to_string();
    }

    if !numeroFinalD.eq("") {
        return numeroFinalE + "CON " + &numeroFinalD;
    }
    return numeroFinalE;
}

fn getTrillonEnAdelante(numE: &str) -> String {
    let mut val = HashMap::new();

    val.insert("1e18", "trillon ");
    val.insert("1e24", "cuatrillon ");
    val.insert("1e30", "quintillon ");
    val.insert("1e36", "sextillon ");
    val.insert("1e42", "septillon ");
    val.insert("1e48", "octillon ");
    val.insert("1e54", "nonillon ");
    val.insert("1e60", "decillon ");
    val.insert("1e66", "undecillon ");
    val.insert("1e72", "duodecillon ");
    val.insert("1e78", "tridecillon ");
    val.insert("1e84", "cuadradecillon ");
    val.insert("1e90", "quintidecillon ");
    val.insert("1e96", "sextidecillon ");
    val.insert("1e102", "septidecillon ");
    val.insert("1e108", "octidecillon ");
    val.insert("1e114", "nonidecillon ");
    val.insert("1e120", "vigintillon ");

    let mut aux = numE;
    let num: f64 = aux.parse().unwrap();
    let numero = aux;
    if num == 1e18 {
        return "un trillon".to_string();
    } else if num == 1e24 {
        return "un cuatrillon".to_string();
    } else if num == 1e30 {
        return "un quintillon".to_string();
    } else if num == 1e36 {
        return "un sextillon".to_string();
    } else if num == 1e42 {
        return "un septillon".to_string();
    } else if num == 1e48 {
        return "un octillon".to_string();
    } else if num == 1e54 {
        return "un nonillon".to_string();
    } else if num == 1e60 {
        return "un decillon".to_string();
    } else if num == 1e66 {
        return "un undecillon".to_string();
    } else if num == 1e72 {
        return "un duodecillon".to_string();
    } else if num == 1e78 {
        return "un tridecillon".to_string();
    } else if num == 1e84 {
        return "un cuadradecillon".to_string();
    } else if num == 1e90 {
        return "un quintidecillon".to_string();
    } else if num == 1e96 {
        return "un sextidecillon".to_string();
    } else if num == 1e102 {
        return "un septidecillon".to_string();
    } else if num == 1e108 {
        return "un octidecillon".to_string();
    } else if num == 1e114 {
        return "un nonidecillon".to_string();
    } else if num == 1e120 {
        return "un vigintillon".to_string();
    } else if num > 1e18 && num < 1e24 {
        let a = 18;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getMilBillones(numero.substring(numero.len() - a, numero.len()));
        return mil + &"trillones ".to_string() + &mill;
    } else if num >= 1e24 && num < 1e30 {
        let a = 24;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"cuadrillones ".to_string() + &mill;
    } else if num >= 1e30 && num < 1e36 {
        let a = 30;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"quintillones ".to_string() + &mill;
    } else if num >= 1e36 && num < 1e42 {
        let a = 36;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"sextillones ".to_string() + &mill;
    } else if num >= 1e42 && num < 1e48 {
        let a = 42;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"septillones ".to_string() + &mill;
    } else if num >= 1e48 && num < 1e54 {
        let a = 48;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"octillones ".to_string() + &mill;
    } else if num >= 1e54 && num < 1e60 {
        let a = 54;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"nonillones ".to_string() + &mill;
    } else if num >= 1e60 && num < 1e66 {
        let a = 60;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"decillones ".to_string() + &mill;
    } else if num >= 1e66 && num < 1e72 {
        let a = 66;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"undecillones ".to_string() + &mill;
    } else if num >= 1e72 && num < 1e78 {
        let a = 72;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"duodecillones ".to_string() + &mill;
    } else if num >= 1e78 && num < 1e84 {
        let a = 78;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"tridecillones ".to_string() + &mill;
    } else if num >= 1e84 && num < 1e90 {
        let a = 84;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"tridecillones ".to_string() + &mill;
    } else if num >= 1e90 && num < 1e96 {
        let a = 90;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"quintidecillones ".to_string() + &mill;
    } else if num >= 1e96 && num < 1e102 {
        let a = 96;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"sextidecillones ".to_string() + &mill;
    } else if num >= 1e102 && num < 1e108 {
        let a = 102;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"septidecillones ".to_string() + &mill;
    } else if num >= 1e108 && num < 1e114 {
        let a = 108;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"octidecillones ".to_string() + &mill;
    } else if num >= 1e114 && num < 1e120 {
        let a = 114;
        let mil = getMiles(numero.substring(0, numero.len() - a));
        let mill = getTrillonEnAdelante(numero.substring(numero.len() - a, numero.len()));
        return mil + &"nonidecillones ".to_string() + &mill;
    }

    return "".to_string();
}

fn getMilBillones(numE: &str) -> String {
    let mut aux = numE;
    let num: f64 = aux.parse().unwrap();
    let numero = &num.to_string();
    if num == 1e15 {
        return "mil billones ".to_string();
    } else if num > 1e15 && num < 2e15 {
        let c = numero.substring(1, numero.len() + 1);
        let cen = getBillon(c);
        return "mil billones ".to_string() + &cen;
    } else if num >= 2e15 && num < 10e15 {
        let m = numero.substring(0, 1);
        let c = numero.substring(1, numero.len() + 1);
        let mill = getUnidades(m);
        let mil = getBillon(c);
        return mill + "mil billones " + &mil;
    } else if num >= 10e15 && num < 1e18 {
        let a = 15;
        let mil = getMiles(numE.substring(0, numE.len() - a));
        let mill = getBillon(numE.substring(numE.len() - a, numE.len()));
        return mil + &"mil billones ".to_string() + &mill;
    }

    return "".to_string();
}

fn getBillon(numE: &str) -> String {
    let mut aux = numE;
    let num: f64 = aux.parse().unwrap();
    let numero = &num.to_string();
    if num == 1e12 {
        return "un billon ".to_string();
    } else if num > 1e12 && num < 2e12 {
        let c = numero.substring(1, numero.len() + 1);
        let cen = getMilMillones(c);
        return "un billon ".to_string() + &cen;
    } else if num >= 2e12 && num < 10e12 {
        let m = numero.substring(0, 1);
        let c = numero.substring(1, numero.len() + 1);
        let mill = getUnidades(m);
        let mil = getMilMillones(c);
        return mill + "billones " + &mil;
    } else if num >= 10e12 && num < 100e12 {
        let m = numero.substring(0, 2);
        let c = numero.substring(2, numero.len() + 1);
        let dec = getDecenas(m);
        let mil = getMilMillones(c);

        return dec + "billones " + &mil;
    } else if num >= 100e12 && num < 1e15 {
        let m = numero.substring(0, 3);
        let c = numero.substring(3, numero.len() + 1);
        let mil = getCentenas(m);
        let cen = getMilMillones(c);
        return mil + "billones " + &cen;
    }

    return "".to_string();
}

fn getMilMillones(numE: &str) -> String {
    let mut aux = numE;
    let num: f64 = aux.parse().unwrap();
    let numero = &num.to_string();
    if num == 1e9 {
        return "mil millones ".to_string();
    } else if num > 1e9 && num < 2e9 {
        let c = numero.substring(1, numero.len() + 1);
        let cen = getMillones(c);
        return "mil millones ".to_string() + &cen;
    } else if num >= 2e9 && num < 10e9 {
        let m = numero.substring(0, 1);
        let c = numero.substring(1, numero.len() + 1);
        let mill = getUnidades(m);
        let mil = getMillones(c);
        return mill + "mil millones " + &mil;
    } else if num >= 10e9 && num < 100e9 {
        let m = numero.substring(0, 2);
        let c = numero.substring(2, numero.len() + 1);
        let dec = getDecenas(m);
        let mil = getMillones(c);

        return dec + "mil millones " + &mil;
    } else if num >= 100e9 && num < 1e12 {
        let m = numero.substring(0, 3);
        let c = numero.substring(3, numero.len() + 1);
        let mil = getCentenas(m);
        let cen = getMillones(c);
        return mil + "mil millones " + &cen;
    }

    return "".to_string();
}

fn getMillones(numE: &str) -> String {
    let mut aux = numE;
    let num: i64 = aux.parse().unwrap();
    let numero = &num.to_string();
    if num == 1000000 {
        return "un millon ".to_string();
    } else if num > 1000000 && num < 2000000 {
        let c = numero.substring(1, numero.len() + 1);
        let cen = getMiles(c);
        return "un millon ".to_string() + &cen;
    } else if num >= 2000000 && num < 10000000 {
        let m = numero.substring(0, 1);
        let c = numero.substring(1, numero.len() + 1);
        let mill = getUnidades(m);
        let mil = getMiles(c);
        return mill + "millones " + &mil;
    } else if num >= 10000000 && num < 100000000 {
        let m = numero.substring(0, 2);
        let c = numero.substring(2, numero.len() + 1);
        let dec = getDecenas(m);
        let mil = getMiles(c);

        return dec + "millones " + &mil;
    } else if num >= 100000000 && num < 1000000000 {
        let m = numero.substring(0, 3);
        let c = numero.substring(3, numero.len() + 1);
        let mil = getCentenas(m);
        let cen = getMiles(c);
        return mil + "millones " + &cen;
    }

    return "".to_string();
}

fn getMiles(numE: &str) -> String {
    let mut aux = numE;
    let num: i64 = aux.parse().unwrap();
    let numero = &num.to_string();
    if num == 1000 {
        return "mil ".to_string();
    } else if num > 1000 && num < 2000 {
        let c = numero.substring(numero.len() - 3, numero.len() + 1);
        let cen = getCentenas(c);
        return "mil ".to_string() + &cen;
    } else if num >= 2000 && num < 10000 {
        let m = numero.substring(0, 1);
        let c = numero.substring(numero.len() - 3, numero.len() + 1);
        let mil = getUnidades(m);
        let cen = getCentenas(c);

        return mil + "mil " + &cen;
    } else if num >= 10000 && num < 100000 {
        let m = numero.substring(0, 2);
        let c = numero.substring(numero.len() - 3, numero.len() + 1);
        let mil = getDecenas(m);
        let cen = getCentenas(c);

        return mil + "mil " + &cen;
    } else if num >= 100000 && num < 1000000 {
        let m = numero.substring(0, 3);
        let c = numero.substring(numero.len() - 3, numero.len() + 1);
        let mil = getCentenas(m);
        let cen = getCentenas(c);
        return mil + "mil " + &cen;
    } else if num < 1000 {
        return getCentenas(numero);
    }

    return "".to_string();
}

fn getCentenas(numE: &str) -> String {
    let mut val = HashMap::new();
    val.insert("100", "ciento ");
    val.insert("200", "doscientos ");
    val.insert("300", "trescientos ");
    val.insert("400", "cuatrocientos ");
    val.insert("500", "quinientos ");
    val.insert("600", "seiscientos ");
    val.insert("700", "setecientos ");
    val.insert("800", "ochocientos ");
    val.insert("900", "novecientos ");
    let mut aux = numE;
    let num: i64 = aux.parse().unwrap();
    let mut numero: &str = &num.to_string();
    if num > 99 {
        if num == 100 {
            return "cien ".to_string();
        } else {
            let d = getDecenas(&(numero.substring(numero.len() - 2, numero.len()).to_string()));
            let c = numero.substring(0, 1).to_string() + &"00";
            let cen: &str = &c;
            return val[cen].to_string() + &d;
        }
    } else {
        return getDecenas(numero);
    }

    return "".to_string();
}

fn getDecenas(numE: &str) -> String {
    let mut aux = numE;

    let mut val = HashMap::new();
    val.insert("10", "diez ");
    val.insert("11", "once ");
    val.insert("12", "doce ");
    val.insert("13", "trece ");
    val.insert("14", "catorce ");
    val.insert("15", "quince ");
    val.insert("16", "dieciseis ");
    val.insert("17", "diecisiete ");
    val.insert("18", "dieciocho ");
    val.insert("19", "diecinueve ");
    val.insert("20", "veinte ");
    val.insert("30", "treinta ");
    val.insert("40", "cuarenta ");
    val.insert("50", "cincuenta ");
    val.insert("60", "sesenta ");
    val.insert("70", "setenta ");
    val.insert("80", "ochenta ");
    val.insert("90", "noventa ");

    let num: i64 = aux.parse().unwrap();
    let mut numero: &str = &num.to_string();
    if num < 10 {
        return getUnidades(numero);
    } else if num > 19 {
        let u = getUnidades(&num.to_string());
        if u.eq("") {
            let a = numero.substring(0, 1).to_string();
            let b = a + &"0";
            let c: &str = &b;
            return val[c].to_string();
        } else {
            let a = numero.substring(0, 1).to_string();
            let b = a + &"0";
            let c: &str = &b;
            return val[c].to_string() + "y " + &u;
        }
    } else {
        return val[numero].to_string();
    }

    return "".to_string();
}

fn getUnidades(numero: &str) -> String {
    let num = numero.substring(numero.len() - 1, numero.len() + 1);

    let mut val = HashMap::new();

    val.insert("1", "uno ");
    val.insert("2", "dos ");
    val.insert("3", "tres ");
    val.insert("4", "cuatro ");
    val.insert("5", "cinco ");
    val.insert("6", "seis ");
    val.insert("7", "siete ");
    val.insert("8", "ocho ");
    val.insert("9", "nueve ");

    if val.contains_key(num) {
        return val[num].to_string();
    } else {
        return "".to_string();
    }
}
