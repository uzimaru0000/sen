var v=Object.defineProperty;var V=(t,e,s)=>e in t?v(t,e,{enumerable:!0,configurable:!0,writable:!0,value:s}):t[e]=s;var h=(t,e,s)=>V(t,typeof e!="symbol"?e+"":e,s);(function(){"use strict";class t extends AudioWorkletProcessor{constructor(){super();h(this,"phase",0)}static get parameterDescriptors(){return[{name:"frequency",defaultValue:440,automationRate:"a-rate"},{name:"volume",defaultValue:1,minValue:0,maxValue:1,automationRate:"a-rate"}]}process(d,i,u){const c=i[0],p=globalThis.sampleRate,r=u.frequency,o=u.volume;for(let n=0;n<c.length;n++){const l=c[n];for(let a=0;a<l.length;a++){const m=r.length>1?r[a]:r[0],g=o.length>1?o[a]:o[0],f=this.phase<=.5?this.phase:1-this.phase;l[a]=(f*4-1)*g,this.phase=(this.phase+m/p)%1}}return!0}}registerProcessor("triangleWaveProcessor",t)})();
