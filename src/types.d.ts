export type Vec = {
    x: number,
    y: number
}

export type Body = {
    position: Vec,
    velocity: Vec,
    mass: number,
}

export type Boundary = {
    min: Vec,
    max: Vec
}

export type Tree = {
    boundaries: Boundary[],
    center_of_mass: Vec,
}
