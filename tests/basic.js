import { tsfn } from '../index.js';
import whyIsNodeRunning from 'why-is-node-running';
const registry = new FinalizationRegistry((val) => {
    console.log('finalize', val.target);
})
function main() {
    function callback(num){
        console.log('num:',num);
    }
    const counter = new tsfn.Counter(callback);
    counter.add(10);
    counter.add(5);
    registry.register(counter, 'counter');
    registry.register(callback,'callback');
    
}

main();