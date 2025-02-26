import { tsfn } from '../index.js';
const registry = new FinalizationRegistry((val) => {
    console.log('finalize', val.target);
})
function main() {
    function tracingLog(num){
        console.log('num:',num);
    }
    const counter = new tsfn.Counter(tracingLog);
    counter.add(10);
    counter.add(5);
    registry.register(counter, 'counter');
    registry.register(tracingLog,'callback');
    
}

main();