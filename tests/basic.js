import { tsfn } from '../index.js';
//import whyIsNodeRunning from 'why-is-node-running';
const registry = new FinalizationRegistry((val) => {
    console.log('finalize', val);
})
function main() {
    function callback(err,num){
        console.log('num:',num);
    }
    const counter = new tsfn.Counter(callback);
    const counter2 = new tsfn.Counter(callback);
    counter.add(10);
    counter.add(5);
    registry.register(counter, 'counter');
    registry.register(callback,'callback');
    registry.register(counter2,'counter2');

}

main();
//setImmediate(() => whyIsNodeRunning())
