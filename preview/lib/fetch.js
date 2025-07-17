// deno-lint-ignore-file
// deno-fmt-ignore-file
// @ts-nocheck
// @ts-self-types="./fetch.d.ts
async function fetch(method,url,body){
// TODO how to handle API key
var t=new Headers,e=(body&&t.append("content-type","application/json"),localStorage.getItem("token"));e&&t.append("Authorization",e);let o=null;body&&(o=JSON.stringify(body));e=await self.fetch(url,{method:method,body:o,headers:t}).catch(()=>new Response(null,{status:500}));if(e.ok)return(body=e.headers.get("Authorization"))&&localStorage.setItem("token",body),{status:"ok",body:await e.json()};if(401===e.status)return{status:"unauthorized"};if(400<=e.status&&e.status<500){let t=await e.json();return{status:"clientError",problems:t.problems??[]}}return{status:"serverError"}}export{fetch};