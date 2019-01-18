use std::io;
use std::io::Write; // <--- bring flush() into scope

pub struct LCG {
    x: [u32; 101],
    a: u32,
    c: u32,
    m: u32,
}

// Initial seeds for a 100 streams
pub fn zrng(r: &mut LCG) {
    r.x = [0,
	1973272912,	281629770,	20006270,	1280689831,	2096730329,	1933576050,
	913566091,	246780520,	1363774876,	604901985,	1511192140,	1259851944,
	824064364,	150493284,	242708531,	75253171,	1964472944,	1202299975,
	233217322,	1911216000,	726370533,	403498145,	993232223,	1103205531,
	762430696,	1922803170,	1385516923,	76271663,	413682397,	726466604,
	336157058,	1432650381,	1120463904,	595778810,	877722890,	1046574445,
	68911991,	2088367019,	748545416,	622401386,	2122378830,	640690903,
	1774806513,	2132545692,	2079249579,	78130110,	852776735,	1187867272,
	1351423507,	1645973084,	1997049139,	922510994,	2045512870,	898585771,
	243649545,	1004818771,	773686062,	403188473,	372279877,	1901633463,
	498067494,	2087759558,	493157915,	597104727,	1530940798,	1814496276,
	536444882,	1663153658,	855503735,	67784357,	1432404475,	619691088,
	119025595,	880802310,	176192644,	1116780070,	277854671,	1366580350,
	1142483975,	2026948561,	1053920743,	786262391,	1792203830,	1494667770,
	1923011392,	1433700034,	1244184613,	1147297105,	539712780,	1545929719,
	190641742,	1645390429,	264907697,	620389253,	1502074852,	927711160,
	364849192,	2049576050,	638580085,	547070247]
}

// Set seed for this stream
pub fn randst(r: &mut LCG, zset: u32, stream: usize) {
    r.x[stream] = zset;
}

// Return the integer actual of sequence in this stream
pub fn randgt(r: LCG, stream: usize) -> u32 {
    r.x[stream]
}

pub fn rand(r: &mut LCG, stream: usize) -> f32 {
    //x = ((a * x) + c ) % m; Panic multiply with overflow
    let mut aux: u64;       // Solution to overflow of u32 

    aux = r.a as u64 * r.x[stream] as u64;
    aux = aux + r.c as u64;
    aux = aux % r.m as u64;
    r.x[stream] = aux as u32;
    (r.x[stream] as f32 / r.m as f32)
}

fn main() {
    println!("Linear Congruential Generator: Xn+1 = (aXn + c) mod m");

    let mut entrada = String::new();
    let n: u32; 
    let mut r = LCG {x: [0; 101], a: 630_360_016, c: 0, m: 2_147_483_647};
    zrng(&mut r);

    print!("X0? [1.973.272.912] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();          // Remove CR last char ;)
    if !entrada.is_empty() {
        r.x[1] = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }
  
    entrada.clear();
    print!("a? [630.360.016] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.a = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("c? [0] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.c = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("m? [2.147.483.647] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.m = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("n secuència a generar? [25]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if entrada.is_empty() {
        n = 25;
    } else {
        n = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    for _i in 0..n {
        print!("{:.6}, ", rand(&mut r, 1));
    }
    println!("...");
}
