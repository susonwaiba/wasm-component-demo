import assert from 'assert';
import { divide } from './index.js';

const inputs = [
    { x: 4, y: 2, z: 2 },
    { x: 6, y: 3, z: 2 },
    { x: 9, y: 3, z: 3 },
    { x: 10, y: 5, z: 2 },
    { x: 16, y: 2, z: 8 },
    { x: 2, y: 0, z: Infinity },
    { x: 0, y: 2, z: 0 },
];

for (const input of inputs) {
    const x = input.x;
    const y = input.y;
    const z = divide(x, y);
    console.log(`input: ${x}/${y} | result: ${z}`);
    assert.strictEqual(z, input.z, 'result should be ' + input.z);
}

console.log('');
console.log('All tests completed');
