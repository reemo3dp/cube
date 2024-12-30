import type { Coord } from "./types";

export const createCube = (random: typeof Math.random, dim: number) => {
    const nextInt = (max: number) => random() * max | 0;
    const numTotalElements = dim * dim * dim;


    const validNeighbours = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ]

    const getOneNeighbour = ([x, y, z]: Coord): Coord => {
        for (let [dx, dy, dz] of validNeighbours) {
            const nx = x + dx;
            const ny = y + dy;
            const nz = z + dz;
            if (nx < dim && ny < dim && nz < dim &&
                nx >= 0 && ny >= 0 && nz >= 0) {
                return [nx, ny, nz];
            }
        }
        throw undefined;
    }

    const createCubeRec = (chain: Coord[]) => {
        if (chain.length == numTotalElements) return chain;
        const head = chain[chain.length - 1];
        const neighbour = getOneNeighbour(head);
        const [nx, ny, nz] = neighbour;
        for (let i = chain.length - 1; i >= 0; i--) {
            const [px, py, pz] = chain[i];
            if (px == nx && py == ny && pz == nz) {
                if (++globalThis.NUM_TRIED % globalThis.PRINT_EVERY === 0) {
                    console.log("//D", globalThis.NUM_TRIED / (new Date().getTime() - globalThis.START_TIME));
                }
                return undefined;
            }
        }
        return createCubeRec(chain.concat([neighbour]));
    };

    while (true) {
        const start: Coord = [nextInt(dim), nextInt(dim), nextInt(dim)];
        const r = nextInt(3);
        start[r] = start[(r + 1) % 3] = 0;

        const result = createCubeRec([start])
        if (result) return result;
    }

};
