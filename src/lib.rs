pub fn calcular_oleadas(total: i32, por_oleada: i32) -> i32 {
    total / por_oleada
}

pub fn calcular_dano_critico(dano_base: i32, multiplicador: i32) -> i32 {
    dano_base * multiplicador
}

pub fn calcular_dano_promedio(dano_base: i32, oleadas: i32) -> i32 {
    dano_base / oleadas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_calcular_oleadas() {
        assert_eq!(calcular_oleadas(100, 10), 10);
    }

    #[test]
    fn prueba_calcular_dano_critico() {
        assert_eq!(calcular_dano_critico(50, 2), 100);
    }

    #[test]
    fn prueba_calcular_dano_promedio() {
        assert_eq!(calcular_dano_promedio(50, 10), 5);
    }
}
