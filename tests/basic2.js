import { tsfn } from '../index.js';
const registr = new FinalizationRegistry((val) => {
    console.log('finalize', val);
});
function main(){
    function tracingLog(num){
        console.log('num:',num);
    }
    const container = new tsfn.Container(tracingLog);
    console.log('start');
    container.runInBackground(100);
    console.log('end');

    registr.register(container, 'container');
    registr.register(tracingLog, 'callback');
    
}
main();