import {it } from 'node:test'
import { external, reference } from '../index.js';
it('external not dropped', () => {
    const compiler = new external.Compiler();
    console.log(compiler);
    const compilation = compiler.createCompilation();
    
})
it('reference dropped', () => {
    const compilation = new reference.Compilation();
    const compiler = new reference.Compiler(compilation);
    
})