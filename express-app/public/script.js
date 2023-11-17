import { add } from './component/adder.js';

const result = add.add(2, 2);

console.log(`result: ${result}`);

document.getElementById('result').innerText = `result: ${result}`;
