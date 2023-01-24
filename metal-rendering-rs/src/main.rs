use metal_rendering_rs::run;

fn main() {
    pollster::block_on(run()); //run e async ent temos de meter alguma coisa tipo o tokio main, usamos o pollster pq e mais leve
}
