import {it } from 'node:test'
import { external, reference } from '../index.js';
it('external not dropped', () => {
    const compiler = external.Compiler();
    
})
it('reference dropped', () => {
    const compilation = reference.Compilation();
    const compiler = new reference.Compiler(compilation);
    
})