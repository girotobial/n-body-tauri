export type Vec = {
    x: number,
    y: number
}

export type Body = {
    position: Vec,
    velocity: Vec,
    mass: number,
    radius: number,
}

export type Boundary = {
    min: Vec,
    max: Vec
}

export type Tree = {
    boundaries: Boundary[],
    center_of_mass: Vec,
    outer_bounds: Boundary,
    center: Vec,
}
