import type { Coord } from "./types";

export const createCube = (random: typeof Math.random, dim: number) => {
    const nextInt = (max: number) => random() * max | 0;
    const numTotalElements = dim * dim * dim;

    const shuffleArray = <T>(array: T[]) => {
        for (let i = array.length - 1; i >= 0; i--) {
            const j = Math.floor(random() * (i + 1));
            [array[i], array[j]] = [array[j], array[i]];
        }
        return array;
    }

    const validNeighbours = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ]

    const getNeighbours = ([x, y, z]: Coord) => {
        const neighbours: Coord[] = [];
        for (let [dx, dy, dz] of validNeighbours) {
            const nx = x + dx;
            const ny = y + dy;
            const nz = z + dz;
            if (nx < dim && ny < dim && nz < dim &&
                nx >= 0 && ny >= 0 && nz >= 0) {
                neighbours.push([nx, ny, nz]);
            }
        }
        return neighbours;
    }

    const createCubeRec = (chain: Coord[]) => {
        if (chain.length == numTotalElements) return chain;
        const head = chain[chain.length - 1];
        const neighbours = shuffleArray(getNeighbours(head));
        outer: for (let neighbour of neighbours) {
            const [nx, ny, nz] = neighbour;
            for (let i = chain.length - 1; i >= 0; i--) {
                const [px, py, pz] = chain[i];
                if (px == nx && py == ny && pz == nz) {
                    if(++globalThis.NUM_TRIED % globalThis.PRINT_EVERY === 0) {
                        console.log("//D", globalThis.NUM_TRIED/(new Date().getTime() - globalThis.START_TIME));
                    }
                    continue outer;
                }
            }

            const result = createCubeRec(chain.concat([neighbour]));
            if (result) return result;
        }
        return undefined;
    };

    const start: Coord = [nextInt(dim), nextInt(dim), nextInt(dim)];
    const r = nextInt(3);
    start[r] = start[(r+1)%3] = 0;

    return createCubeRec([start]);
};
