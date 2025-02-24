import { tsfn } from '../index.js';
//import whyIsNodeRunning from 'why-is-node-running';

const counter = new tsfn.Counter((err, num) => {
    console.log(err, num)
});
counter.add(10);
counter.add(5);
//setImmediate(() => whyIsNodeRunning())
