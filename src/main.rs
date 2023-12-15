use serde::{Deserialize, Serialize};
use std::fs;
fn main{ let path1 = "data/interactiondata.json".to_owned();
    let cont1 = fs::read_to_string(path1);

let path2 = "data/factordata.json".to_owned();
    let cont2 = fs::read_to_string(path2);

let path3 = "data/render.json".to_owned();
    let cont3 = fs::read_to_string(path3);

parser::file1(&cont1);
parser::file2(&cont2);


#[derive(Debug, Deserialize)]
struct interactiondata {
    faclist: Vec<String>,
    fmodval: Hashmap<String, String>,
    emodval: Hashmap<String, String>
}
#[derive(Debug, Deserialize)]
struct factordata {
    factorname: String,
    facvalue: f64,
    expvalue: f64
}

#[derive(Debug, Deserialize)]
struct render {
    x:Hashmap<factname, factval, exprval>,
    y:Hashmap<factname, factval, exprval>
}

struct factname(String, String);

struct factval(String, Vec<f64>);

struct exprval(String, Vec<f64>);

let i: interactiondata = serde_json::from_str(cont1)?;
let f: factordata = serde_json::from_str(cont2)?;
let r: render = serde_json::from_str(cont3)?;

fn wemathin(optype) {   
    let e = (v[optype]);
    let ops = ['+', '-', '*', '/', '^'];
    let values: Vec<f64> = e.split(&ops).map(|v| v.trim().parse().unwrap()).collect();
    let operands: Vec<_> = e.matches(&ops).collect();
    
    let (&(mut curr), values) = values.split_first().unwrap();
    for (op, &value) in operands.into_iter().zip(values) {
        match op {
            "+" => { curr = curr + value },
            "-" => { curr = curr - value },
            "*" => { curr = curr * value },
            "/" => { curr = curr / value },
            "^" => { curr = pow(curr, value)},
            _ => unreachable!(),
        }
    }
};


fn upd(facname, facnamein, facval, optype) {
    match self.facnamein {
        self.facname => self.facval = self.wemathin(optype)
        _ => self.facval = self.facval
    }
}

//цикл на ран для каждого случая в системе заданной 1 координат
//анонс значений
//чек на замену значений
// после ретерн векторов и их запись
//пуш данных
for val in r.x{
let &mut localfval =     
    self(match self.f.factorname {
        r.x => self.r.facval
        r.y => self.r.facval
        _ => self.f.facvalue;
    }
    )
    let &mut localval =     
    self(match self.f.factorname {
        r.x => self.r.facval
        r.y => self.r.facval
        _ => self.f.facvalue;
    }
    )
for val in r.y{

for val in f.factorname {
    let localfname = self.f.factorname

// добавить модификацию операции на факэксп
        for val in interactiondata {
            match self.i.faclist {
                self.localfname => upd(f.factoname, i.faclist, localfval, fmodval)
                _ => self.tempf
            }
            match self.i.faclist {
                self.localfname => upd(f.factoname, i.faclist, localeval, fmodval)
                _ => self.tempe
            }
        }
    }
}
}


//создание матрицы из значений фактора
//матрица в файл текстовый
//рендер пикчи по значению в матрице

}