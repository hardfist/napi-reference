import {it } from 'node:test'
import { external, reference, value, _class,external_value, reference_callback,tsfn, async_fn, nested_call} from '../index.js';
it.only('value', () => {
    let counter = value.createCounter(100);
    console.log(counter.cnt);
    counter.cnt+=1;
    console.log(counter.cnt)
    let new_counter = value.fromJsClone(counter)
    new_counter.cnt+=1;
    console.log('new counter:',new_counter);
    console.log('old counter:',counter);

    let new_counter2 = value.fromJsSerde(counter);
    counter.cnt+=1;
    console.log('old counter:', counter);
    console.log('new counter:', new_counter2)
})

it('class', () => {
    let counter = new _class.Counter(100);
    console.log(counter.cnt);
    counter.add(20);
    console.log(counter.cnt)
})
it('external_value', () => {
    let counter = external_value.createCounter(100);
    external_value.printExternal(counter);
    external_value.addCounter(counter,20);
    external_value.printExternal(counter);
})
it('external not dropped', () => {
    const compiler = new external.Compiler();
    console.log(compiler);
    const compilation = compiler.createCompilation();
    
})
it('reference callback', () => {
    function callback(){
        console.log('call');
    }
    let compiler = new reference_callback.Compiler(callback)
    compiler.run();
    callback();
    compiler.run();
})
it('reference dropped', () => {
    const compilation = new reference.Compilation();
    const compiler = new reference.Compiler(compilation);
    
})
it('tsfn', () => {
    const counter = new tsfn.Counter((err,num) => {
        console.log(err,num)
    });
    counter.add(10);
    counter.add(5);
})
it.only('async fn', () => {
    async_fn.asyncFn2((res)=> {
        console.log('res:',res);
    })
})

it('nest call', () => {
    const compiler =new nested_call.
})