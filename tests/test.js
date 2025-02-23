import {it } from 'node:test'
import { external, reference, value, _class } from '../index.js';
it('value', () => {
    let counter = value.createCounter(100);
    console.log(counter.cnt);
    counter.cnt+=1;
    console.log(counter.cnt)
})
it('class', () => {
    let counter = new _class.Counter(100);
    console.log(counter.cnt);
    counter.add(20);
    console.log(counter.cnt)
})
it('external not dropped', () => {
    const compiler = new external.Compiler();
    console.log(compiler);
    const compilation = compiler.createCompilation();
    
})
it('reference dropped', () => {
    const compilation = new reference.Compilation();
    const compiler = new reference.Compiler(compilation);
    
})