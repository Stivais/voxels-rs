use crate::world::chunk::chunk::{Chunk, CS};

// todo: binary baked ao greedy meshing
pub fn greedy_mesh(chunk: &Chunk) -> Vec<Vec<u64>> {
    let mut vertices: Vec<Vec<u64>> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    // top
    for y in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y + 1, z)
        }

        for x in 0..CS {
            for z in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[x + z * CS] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS && is_visible(chunk, x + w, y, z) && !visited[(x + w) + z * CS] {
                        w += 1;
                    }

                    'outer: while z + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[(x + i) + (z + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (z + dz) * CS] = true;
                        }
                    }

                    vertices[0].push(pack_data(x, y + 1, z, w, d, 0));
                }
            }
        }
    }

    // bottom
    for y in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y - 1, z)
        }

        for x in 0..CS {
            for z in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[x + z * CS] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS && is_visible(chunk, x + w, y, z) && !visited[(x + w) + z * CS] {
                        w += 1;
                    }

                    'outer: while z + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[(x + i) + (z + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (z + dz) * CS] = true;
                        }
                    }

                    vertices[1].push(pack_data(x + w, y, z, w, d, 0));
                }
            }
        }
    }

    // right
    for x in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x + 1, y, z)
        }

        for y in 0..CS {
            for z in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[z + y * CS] {
                    let mut w = 1;
                    let mut d = 1;

                    while z + w < CS && is_visible(chunk, x, y, z + w) && !visited[(z + w) + y * CS] {
                        w += 1;
                    }

                    'outer: while y + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[(z + i) + (y + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(z + dx) + (y + dz) * CS] = true;
                        }
                    }

                    vertices[2].push(pack_data(x + 1, y, z, d, w, 0));
                }
            }
        }
    }

    // left
    for x in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x - 1, y, z)
        }

        for y in 0..CS {
            for z in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[z + y * CS] {
                    let mut w = 1;
                    let mut d = 1;

                    while z + w < CS && is_visible(chunk, x, y, z + w) && !visited[(z + w) + y * CS] {
                        w += 1;
                    }

                    'outer: while y + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[(z + i) + (y + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for width in 0..w {
                        for depth in 0..d {
                            visited[(z + width) + (y + depth) * CS] = true;
                        }
                    }
                    vertices[3].push(pack_data(x, y, z, d, w, 0));
                }
            }
        }
    }

    // front
    for z in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z - 1)
        }

        for y in 0..CS {
            for x in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[(x + y * CS)] {
                    let mut w = 1;
                    let mut d = 1;


                    while z + w < CS && is_visible(chunk, x + w, y, z) && !visited[(x + w) + y * CS] {
                        w += 1;
                    }

                    'outer: while y + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[(x + i) + (y + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (y + dz) * CS] = true;
                        }
                    }

                    vertices[4].push(pack_data(x + w, y, z, w, d, 0));
                }
            }
        }
    }

    // back
    for z in 0..CS {
        let mut visited = vec![false; CS * CS];

        #[inline]
        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z + 1)
        }

        for y in 0..CS {
            for x in 0..CS {
                if is_visible(chunk, x, y, z) && !visited[x + y * CS] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS && is_visible(chunk, x + w, y, z) && !visited[(x + w) + y * CS] {
                        w += 1;
                    }

                    'outer: while y + d < CS {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[(x + i) + (y + d) * CS] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (y + dz) * CS] = true;
                        }
                    }

                    vertices[5].push(pack_data(x, y, z + 1, w, d, 0));
                }
            }
        }
    }
    vertices
}

fn pack_data(x: usize, y: usize, z: usize, width: usize, height: usize, texture_id: u8) -> u64 {
    (x as u64) |
    ((y as u64) << 6) |
    ((z as u64) << 12) |
    ((width as u64) << 18) |
    ((height as u64) << 24) |
    ((texture_id as u64) << 30)
}
