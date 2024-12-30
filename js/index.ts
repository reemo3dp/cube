import seedrandom from "seedrandom";
import { createCube as dynamic_createCube } from "./dynamic_programming.ts";
import { createCube as randomizer_createCube } from "./randomizer.ts";
import { createCube as super_random_createCube } from "./super_random.ts";



globalThis.START_TIME = new Date().getTime();
globalThis.NUM_TRIED = 0;
globalThis.PRINT_EVERY = 1000000;




const seed = process.env.SEED || (Math.random() * 0x10000000 + 0x1000000 % 0x100000 | 0).toString(16);
const random = seedrandom(seed);
const dim = Number(process.argv[2]);

const cube = randomizer_createCube(random, dim);
console.log(`
// Seed: ${seed}
DIM = ${dim};
PATH = ${JSON.stringify(cube)};
`);
