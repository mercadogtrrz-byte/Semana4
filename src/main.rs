fn calcular_oleadas(total: i32, por_oleada: i32) -> i32 {
    total / por_oleada
}

fn calcular_dano_critico(dano_base: i32, multiplicador: i32) -> i32 {
    dano_base * multiplicador
}

fn calcular_dano_promedio(dano_base: i32, oleadas: i32) -> i32 {
    dano_base / oleadas
}

fn main() {
    let total_enemigos: i32 = 100;
    let enemigos_por_oleada: i32 = 10;

    let dano_base: i32 = 50;
    let multiplicador_critico: i32 = 2;

    let oleadas = calcular_oleadas(
        total_enemigos,
        enemigos_por_oleada,
    );

    let dano_critico = calcular_dano_critico(
        dano_base,
        multiplicador_critico,
    );

    let dano_promedio = calcular_dano_promedio(
        dano_base,
        oleadas,
    );

    println!("Proyecto: tipos_operandos_rust");
    println!("Total de enemigos: {}", total_enemigos);
    println!("Enemigos por oleada: {}", enemigos_por_oleada);
    println!("Número de oleadas: {}", oleadas);
    println!("Daño base: {}", dano_base);
    println!("Daño crítico: {}", dano_critico);
    println!("Daño promedio: {}", dano_promedio);
}
